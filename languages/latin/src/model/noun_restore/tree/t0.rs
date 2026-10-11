use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => super::t0::n1(s, out),
		['b', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 38);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n39(s, out),
				['b', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['a', 'h', 't', 'i', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 305);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 306);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 307);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1192);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2847);
											match s {
												[] => true,
												['a', 't', 'e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 4799);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => super::t0::n2879(s, out),
										['o', 't', 'e', 'h', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2849);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1193);
									s.is_empty()
								}
								['t', s @ ..] => super::t0::n1206(s, out),
								_ => false,
							}
						}
						['y', 's', '\u{1}', s @ ..] => super::t0::n308(s, out),
						_ => false,
					}
				}
				['d', 'o', '\u{1}', s @ ..] => super::t0::n42(s, out),
				['g', s @ ..] => super::t0::n43(s, out),
				['h', s @ ..] => super::t0::n44(s, out),
				['l', s @ ..] => super::t0::n45(s, out),
				['m', s @ ..] => super::t0::n46(s, out),
				['n', s @ ..] => super::t0::n47(s, out),
				['o', 'l', 'a', '\u{1}', s @ ..] => super::t0::n48(s, out),
				['p', s @ ..] => super::t0::n49(s, out),
				['r', s @ ..] => super::t0::n50(s, out),
				['t', s @ ..] => super::t0::n51(s, out),
				['u', s @ ..] => super::t0::n52(s, out),
				['v', s @ ..] => super::t0::n53(s, out),
				['y', 'h', 'p', 'a', '\u{1}', s @ ..] => super::t0::n54(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n5(s, out),
		['i', s @ ..] => super::t0::n6(s, out),
		['l', s @ ..] => super::t0::n7(s, out),
		['m', s @ ..] => super::t0::n8(s, out),
		['n', s @ ..] => super::t0::n9(s, out),
		['o', s @ ..] => super::t0::n10(s, out),
		['r', s @ ..] => super::t0::n11(s, out),
		['s', s @ ..] => super::t0::n12(s, out),
		['t', s @ ..] => super::t0::n13(s, out),
		['u', s @ ..] => super::t0::n14(s, out),
		['x', s @ ..] => super::t0::n15(s, out),
		['y', 'l', 'o', 'm', '\u{1}', s @ ..] => super::t0::n16(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 123);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 124);
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 680);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 125);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2172);
									s.is_empty()
								}
								['t', 'i', 'r', 'g', 'a', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 126);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 127);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 682);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2174);
									s.is_empty()
								}
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
			super::push(out, 18);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 128);
					match s {
						[] => true,
						['h', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 683);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 684);
							match s {
								[] => true,
								['r', 'e', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 685);
							match s {
								[] => true,
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2176);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 686);
							s.is_empty()
						}
						['r', 'a', 'd', 'n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 687);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 688);
							s.is_empty()
						}
						['z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 692);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4370);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 4371);
											s.is_empty()
										}
										['r', 'i', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4372);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 4373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', 'n', 'e', 'm', 'u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 693);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 694);
							match s {
								[] => true,
								['o', 'r', 'i', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 4374);
											s.is_empty()
										}
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n131(s, out),
				['n', s @ ..] => {
					super::push(out, 132);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 704);
							match s {
								[] => true,
								['l', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 133);
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 134);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 135);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 707);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 708);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 709);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 710);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4394);
											s.is_empty()
										}
										['b', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 4395);
											s.is_empty()
										}
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
					super::push(out, 136);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 711);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 712);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n716(s, out),
						['t', s @ ..] => {
							super::push(out, 714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n19(s, out),
		['e', s @ ..] => super::t0::n20(s, out),
		['f', s @ ..] => super::t0::n21(s, out),
		['g', s @ ..] => super::t0::n22(s, out),
		['h', s @ ..] => super::t0::n23(s, out),
		['i', s @ ..] => super::t0::n24(s, out),
		['l', s @ ..] => super::t0::n25(s, out),
		['m', s @ ..] => super::t0::n26(s, out),
		['n', s @ ..] => super::t0::n27(s, out),
		['o', s @ ..] => super::t0::n28(s, out),
		['p', s @ ..] => super::t0::n29(s, out),
		['r', s @ ..] => super::t0::n30(s, out),
		['s', s @ ..] => super::t0::n31(s, out),
		['t', s @ ..] => super::t0::n32(s, out),
		['u', s @ ..] => super::t0::n33(s, out),
		['v', s @ ..] => super::t0::n34(s, out),
		['x', s @ ..] => super::t0::n35(s, out),
		['y', s @ ..] => {
			super::push(out, 36);
			s.is_empty()
		}
		['z', s @ ..] => super::t0::n37(s, out),
		_ => false,
	}
}

/// Words ending -ae.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['i', 'c', 'i', 'o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						['u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2832);
							match s {
								[] => true,
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['e', 't', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2834);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2835);
							match s {
								[] => true,
								['v', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2836);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4796);
									s.is_empty()
								}
								['t', 'i', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2839);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['i', 'c', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['f', 'o', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2841);
							s.is_empty()
						}
						['g', 'i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						['n', 'i', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['s', 'a', 'n', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['c', 'e', 'h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irice.
pub(crate) fn n2879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2848);
	match s {
		[] => true,
		['l', 'a', 'd', 'n', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 4800);
			s.is_empty()
		}
		['p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4801);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tice.
pub(crate) fn n1206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	s.is_empty()
}

/// The word syce.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	s.is_empty()
}

/// The word ode.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	s.is_empty()
}

/// Words ending -ge.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['n', 'y', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					match s {
						[] => true,
						['m', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						['s', s @ ..] => {
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

/// Words ending -he.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['y', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['e', 'l', 'a', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['b', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['i', 'v', 'r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['m', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2855);
							match s {
								[] => true,
								['h', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['u', 'm', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['n', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						['u', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						['y', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['t', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['a', 'r', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2865);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						['p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						['s', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						['o', s @ ..] => {
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
			super::push(out, 316);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['m', 'i', 'u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['t', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['m', 'u', 'c', 'e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				['s', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['e', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['i', 's', 'a', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['e', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						['g', 'e', 'l', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aloe.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -pe.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['s', 'u', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['s', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['d', 'i', 's', 'l', 'u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						['u', 'b', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['u', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'a', 'g', 'e', 't', 'o', 'm', 'o', 'z', '\u{1}', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 337);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['i', 'r', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['u', 'r', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'r', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', 'c', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 340);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aphye.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// Words ending -h.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['t', 'e', 'n', 'a', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 55);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['o', 'g', 'a', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['m', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 346);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'b', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2882);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2883);
									s.is_empty()
								}
								['r', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2884);
									s.is_empty()
								}
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
			super::push(out, 60);
			match s {
				[] => true,
				['l', 'i', 'b', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['u', 'r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['i', 't', 'i', 'n', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['m', 'i', 'c', 'e', 'd', 'a', 't', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', 'c', 'a', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				['r', 'e', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['g', 'i', 'm', 'a', 'u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'a', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 355);
					s.is_empty()
				}
				['i', 'v', 's', 'e', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['i', 'l', 'i', 'b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						['r', 'u', 'i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'i', 'h', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['c', 'i', 'v', 'r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				['g', 'i', 't', 'c', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				['i', 'p', 'u', 's', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['a', 'h', 'c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						['i', 'm', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['u', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['u', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['g', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['h', 'o', 'c', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['o', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2885);
									s.is_empty()
								}
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

/// Words ending -m.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['u', s @ ..] => super::t0::n72(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3232);
									match s {
										[] => true,
										['i', 't', '\u{1}', s @ ..] => {
											super::push(out, 5237);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3233);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3234);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5238);
											s.is_empty()
										}
										['u', 'c', 'i', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'u', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['g', 'a', 't', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				['h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1391);
							match s {
								[] => true,
								['b', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3235);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 3236);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3237);
									match s {
										[] => true,
										['i', 'm', 'r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 5240);
											s.is_empty()
										}
										['o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 5241);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 3238);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3239);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => super::t0::n3273(s, out),
								['n', s @ ..] => {
									super::push(out, 3241);
									s.is_empty()
								}
								['r', s @ ..] => super::t0::n3275(s, out),
								['t', s @ ..] => {
									super::push(out, 3243);
									match s {
										[] => true,
										['c', s @ ..] => super::t0::n5324(s, out),
										['u', s @ ..] => super::t0::n5325(s, out),
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3244);
									s.is_empty()
								}
								['v', s @ ..] => super::t0::n3278(s, out),
								_ => false,
							}
						}
						['e', s @ ..] => super::t0::n1407(s, out),
						['g', s @ ..] => super::t0::n1408(s, out),
						['i', s @ ..] => super::t0::n1409(s, out),
						['l', s @ ..] => super::t0::n1410(s, out),
						['o', s @ ..] => super::t0::n1411(s, out),
						['r', s @ ..] => super::t0::n1412(s, out),
						['u', s @ ..] => super::t0::n1413(s, out),
						['y', 'h', '\u{1}', s @ ..] => super::t0::n1414(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n396(s, out),
				['t', s @ ..] => super::t0::n397(s, out),
				['u', s @ ..] => super::t0::n398(s, out),
				_ => false,
			}
		}
		['i', 'h', 'p', 'l', 'e', 'd', '\u{1}', s @ ..] => super::t0::n75(s, out),
		['o', s @ ..] => super::t0::n76(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 416);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 417);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1470);
							match s {
								[] => true,
								['e', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 419);
					match s {
						[] => true,
						['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						['r', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 420);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 421);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1476);
							match s {
								[] => true,
								['r', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 3325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1480);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1481);
							match s {
								[] => true,
								['u', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1483);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3329);
									s.is_empty()
								}
								['t', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3330);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 3331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1485);
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['t', s @ ..] => super::t0::n1503(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n81(s, out),
		['f', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['g', s @ ..] => super::t0::n83(s, out),
		['h', s @ ..] => super::t0::n84(s, out),
		['i', s @ ..] => super::t0::n85(s, out),
		['l', s @ ..] => super::t0::n86(s, out),
		['m', s @ ..] => super::t0::n87(s, out),
		['n', s @ ..] => super::t0::n88(s, out),
		['p', s @ ..] => super::t0::n89(s, out),
		['r', s @ ..] => super::t0::n90(s, out),
		['s', s @ ..] => super::t0::n91(s, out),
		['t', s @ ..] => super::t0::n92(s, out),
		['u', s @ ..] => super::t0::n93(s, out),
		['v', s @ ..] => super::t0::n94(s, out),
		['x', s @ ..] => super::t0::n95(s, out),
		['z', 'i', 'p', 'i', 'p', '\u{1}', s @ ..] => super::t0::n96(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1586);
							s.is_empty()
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['a', 'p', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['i', 'v', 'l', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						['u', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 487);
					match s {
						[] => true,
						['e', 'l', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 490);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1596);
							match s {
								[] => true,
								['l', 'e', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3475);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 3476);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 3477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1599);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1600);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1601);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5561);
											s.is_empty()
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 5562);
											s.is_empty()
										}
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
					super::push(out, 492);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1603);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3480);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5563);
											s.is_empty()
										}
										['u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 5564);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 't', 'p', 'e', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3481);
									s.is_empty()
								}
								['s', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3482);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1605);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3484);
									match s {
										[] => true,
										['a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 5565);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3485);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3486);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5566);
											s.is_empty()
										}
										['u', 'l', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 5567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['m', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['e', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 1608);
							match s {
								[] => true,
								['i', 'm', 'e', 's', '\u{1}', s @ ..] => super::t0::n3524(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t0::n500(s, out),
				['t', s @ ..] => super::t0::n501(s, out),
				['v', s @ ..] => super::t0::n502(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n99(s, out),
		['o', s @ ..] => super::t0::n100(s, out),
		['u', s @ ..] => super::t0::n101(s, out),
		['y', s @ ..] => super::t0::n102(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n103(s, out),
		['b', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n105(s, out),
		['i', s @ ..] => super::t0::n106(s, out),
		['m', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						['g', 'i', 't', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1872);
							match s {
								[] => true,
								['g', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1873);
							match s {
								[] => true,
								['i', 'm', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3838);
									s.is_empty()
								}
								['o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1874);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3840);
									match s {
										[] => true,
										['o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 5919);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 3841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1876);
							match s {
								[] => true,
								['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 3842);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 3843);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 5920);
											s.is_empty()
										}
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
					super::push(out, 581);
					match s {
						[] => true,
						['c', 's', 'e', 'l', 'u', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1878);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3844);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3845);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3846);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 5921);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1880);
							match s {
								[] => true,
								['c', 'i', 'f', 'f', 'e', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3847);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3848);
									s.is_empty()
								}
								['t', 'e', 'm', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1882);
							match s {
								[] => true,
								['o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1883);
							match s {
								[] => true,
								['l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				['b', s @ ..] => super::t0::n587(s, out),
				['c', s @ ..] => super::t0::n588(s, out),
				['d', s @ ..] => super::t0::n589(s, out),
				['e', s @ ..] => super::t0::n590(s, out),
				['g', s @ ..] => super::t0::n591(s, out),
				['h', s @ ..] => super::t0::n592(s, out),
				['i', s @ ..] => super::t0::n593(s, out),
				['l', s @ ..] => super::t0::n594(s, out),
				['m', s @ ..] => super::t0::n595(s, out),
				['n', s @ ..] => super::t0::n596(s, out),
				['o', 'r', 'e', 'h', '\u{1}', s @ ..] => super::t0::n597(s, out),
				['p', s @ ..] => super::t0::n598(s, out),
				['r', s @ ..] => super::t0::n599(s, out),
				['s', s @ ..] => super::t0::n600(s, out),
				['t', s @ ..] => super::t0::n601(s, out),
				['z', s @ ..] => super::t0::n602(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n110(s, out),
		['r', s @ ..] => super::t0::n111(s, out),
		['u', s @ ..] => super::t0::n112(s, out),
		['y', s @ ..] => super::t0::n113(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	s.is_empty()
}

/// Words ending -u.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	s.is_empty()
}

/// Words ending -x.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['c', 'i', 'd', 'e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						['o', 'n', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2115);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4327);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 635);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4329);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2118);
							match s {
								[] => true,
								['l', 'o', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 4331);
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2120);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 4333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4334);
									s.is_empty()
								}
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 4335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 639);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2128);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2129);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4336);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 4337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2130);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4338);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6394);
											match s {
												[] => true,
												['g', 'r', 'o', '\u{1}', s @ ..] => {
													super::push(out, 7970);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4339);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6395);
											s.is_empty()
										}
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
					super::push(out, 644);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2131);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2132);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2133);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2134);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4342);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 6396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2135);
							match s {
								[] => true,
								['i', 'r', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2136);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2137);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2138);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2139);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2140);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4345);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 4346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2141);
							s.is_empty()
						}
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						['e', 'c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						['u', s @ ..] => super::t0::n2168(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t0::n652(s, out),
				['v', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n116(s, out),
		['l', s @ ..] => super::t0::n117(s, out),
		['n', s @ ..] => super::t0::n118(s, out),
		['o', s @ ..] => super::t0::n119(s, out),
		['r', s @ ..] => super::t0::n120(s, out),
		['u', s @ ..] => super::t0::n121(s, out),
		['y', s @ ..] => super::t0::n122(s, out),
		_ => false,
	}
}

/// The word moly.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	s.is_empty()
}

/// Words ending -ica.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2179);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4376);
							s.is_empty()
						}
						['n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['u', 'p', s @ ..] => {
							super::push(out, 4378);
							match s {
								[] => true,
								['s', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4379);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['c', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', 'a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['r', 't', 'c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 700);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4385);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2194);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4387);
							s.is_empty()
						}
						['t', 'e', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2201);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['m', 'e', 'h', 't', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2205);
					match s {
						[] => true,
						['m', 'h', 't', 'i', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2209);
					match s {
						[] => true,
						['u', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ruca.
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['p', 'm', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['c', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						['p', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['u', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 'm', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2220);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4397);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4398);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4399);
									match s {
										[] => true,
										['a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 6440);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6441);
											match s {
												[] => true,
												['r', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 8021);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 6442);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 8022);
													match s {
														[] => true,
														['p', '\u{1}', s @ ..] => super::t0::n9115(s, out),
														_ => false,
													}
												}
												['u', '\u{1}', s @ ..] => {
													super::push(out, 8023);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4400);
									match s {
										[] => true,
										['c', 'e', 's', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 6443);
											s.is_empty()
										}
										['n', 'a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 6444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4401);
									s.is_empty()
								}
								['i', 'r', 't', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						['o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['o', 'h', 'r', 'r', 'o', 'm', 'e', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				['p', 'e', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						['y', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['v', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['c', 's', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['l', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ea.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2228);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4403);
									s.is_empty()
								}
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
			super::push(out, 147);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['c', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['t', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2230);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'd', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['t', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['m', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2234);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2236);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['p', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				['n', 'o', 'c', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['o', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['u', 't', '\u{1}', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				['l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['r', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
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
				['g', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['a', 'm', 'o', 's', 'i', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2245);
							match s {
								[] => true,
								['l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				['s', 'i', 'u', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ha.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4405);
									match s {
										[] => true,
										['n', 'i', 'l', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 6445);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 6446);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6447);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', 'm', 'o', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
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
			super::push(out, 174);
			match s {
				[] => true,
				['i', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['r', 'y', 'm', 'o', 'r', 'e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['i', 'r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
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
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['b', 'u', 'b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								['o', 't', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						['g', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4409);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 4410);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2253);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2254);
							s.is_empty()
						}
						['t', 's', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['p', 'o', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2259);
							match s {
								[] => true,
								['e', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4412);
									s.is_empty()
								}
								['f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['c', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4414);
									s.is_empty()
								}
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['a', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['u', 'd', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4416);
									s.is_empty()
								}
								['n', 'e', 'r', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 4417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['p', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2267);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2268);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2270);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4421);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6448);
											match s {
												[] => true,
												['r', 'i', '\u{1}', s @ ..] => {
													super::push(out, 8024);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 6449);
											s.is_empty()
										}
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
					super::push(out, 794);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2271);
							s.is_empty()
						}
						['m', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4422);
									s.is_empty()
								}
								['e', 'r', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 4423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2275);
							match s {
								[] => true,
								['c', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2276);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6450);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 8025);
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													super::push(out, 8026);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 6451);
											s.is_empty()
										}
										['o', 's', '\u{1}', s @ ..] => {
											super::push(out, 6452);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n181(s, out),
		['g', s @ ..] => super::t0::n182(s, out),
		['h', s @ ..] => super::t0::n183(s, out),
		['l', s @ ..] => super::t1::n184(s, out),
		['m', s @ ..] => super::t1::n185(s, out),
		['n', s @ ..] => super::t1::n186(s, out),
		['p', s @ ..] => super::t1::n187(s, out),
		['r', s @ ..] => super::t1::n188(s, out),
		['s', s @ ..] => super::t1::n189(s, out),
		['t', s @ ..] => super::t1::n190(s, out),
		['u', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['q', s @ ..] => super::t1::n867(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => super::t1::n192(s, out),
		['x', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['e', 'l', 'p', 'o', 'p', 'a', '\u{1}', s @ ..] => super::t1::n871(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 871);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['i', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 873);
					s.is_empty()
				}
				['u', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['a', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2468);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2469);
							match s {
								[] => true,
								['t', 's', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['a', 'r', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2471);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2472);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2473);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				['z', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2475);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4559);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 6535);
											s.is_empty()
										}
										['l', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 6536);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4560);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2476);
							match s {
								[] => true,
								['i', 'v', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2477);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4563);
									match s {
										[] => true,
										['l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 6537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2478);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4564);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 6538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4565);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 6539);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4566);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2479);
							match s {
								[] => true,
								['a', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2480);
							match s {
								[] => true,
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2481);
							match s {
								[] => true,
								['e', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 4570);
									s.is_empty()
								}
								['o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4572);
									match s {
										[] => true,
										['l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 6540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n889(s, out),
				['o', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['c', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 2497);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n200(s, out),
		['p', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		['u', s @ ..] => super::t1::n202(s, out),
		['y', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', 'y', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['y', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 919);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						['e', 't', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 920);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2578);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['m', 'y', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						['o', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						['y', 'h', 'p', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4672);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['l', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 4674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['i', 'm', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2596);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['r', 'g', 'o', 'i', 'l', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2607);
							match s {
								[] => true,
								['p', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2608);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 4677);
									s.is_empty()
								}
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['u', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['i', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n215(s, out),
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['c', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['r', 'e', 'l', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 967);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['o', 'i', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2620);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 2622);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2623);
							match s {
								[] => true,
								['y', 'h', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 4681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2625);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4682);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['o', 'c', 's', 'o', 'n', 'g', 'i', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2629);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2632);
							match s {
								[] => true,
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4683);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2633);
							s.is_empty()
						}
						['l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2634);
							s.is_empty()
						}
						['o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2635);
							s.is_empty()
						}
						['u', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2638);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4685);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2639);
							match s {
								[] => true,
								['b', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 4686);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4687);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6594);
											s.is_empty()
										}
										['i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 6595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => super::t1::n4746(s, out),
								['v', '\u{1}', s @ ..] => super::t1::n4747(s, out),
								_ => false,
							}
						}
						['n', 'u', 'r', '\u{1}', s @ ..] => super::t1::n2671(s, out),
						['r', 'a', 's', '\u{1}', s @ ..] => super::t1::n2672(s, out),
						['s', s @ ..] => super::t1::n2673(s, out),
						['u', s @ ..] => super::t1::n2674(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t1::n991(s, out),
				['g', s @ ..] => super::t1::n992(s, out),
				['h', s @ ..] => super::t1::n993(s, out),
				['l', s @ ..] => super::t1::n994(s, out),
				['m', s @ ..] => super::t1::n995(s, out),
				['n', s @ ..] => super::t1::n996(s, out),
				['o', 'r', 'e', 'h', '\u{1}', s @ ..] => super::t1::n997(s, out),
				['p', s @ ..] => super::t1::n998(s, out),
				['r', s @ ..] => super::t1::n999(s, out),
				['s', s @ ..] => super::t1::n1000(s, out),
				['t', s @ ..] => super::t1::n1001(s, out),
				['u', s @ ..] => super::t1::n1002(s, out),
				['v', s @ ..] => super::t1::n1003(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n220(s, out),
		['m', s @ ..] => super::t1::n221(s, out),
		['n', s @ ..] => super::t1::n222(s, out),
		['o', s @ ..] => super::t1::n223(s, out),
		['r', s @ ..] => super::t1::n224(s, out),
		['u', s @ ..] => super::t1::n225(s, out),
		['y', s @ ..] => super::t1::n226(s, out),
		_ => false,
	}
}

/// Words ending -oa.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['n', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['p', 'u', 'm', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 't', 'c', 'y', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 235);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 't', 'o', 'l', 'e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				['b', 'a', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['i', 't', '\u{1}', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['a', 'c', 'l', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['l', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2685);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['u', 'n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 241);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1031);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2689);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1036);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2694);
							match s {
								[] => true,
								['r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4718);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 4719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2695);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2696);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2697);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1039);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', 'o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2700);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2701);
							match s {
								[] => true,
								['r', 'u', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['t', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['i', 'l', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2703);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2704);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						['t', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				['p', 's', 'o', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['v', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1049);
					match s {
						[] => true,
						['h', 't', 'i', 'm', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2708);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2709);
							match s {
								[] => true,
								['i', 't', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['v', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['l', 'o', 'l', 'o', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2712);
							match s {
								[] => true,
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'g', 'y', 't', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2714);
							match s {
								[] => true,
								['o', 's', 's', 'o', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				['s', s @ ..] => super::t1::n1066(s, out),
				['u', s @ ..] => super::t1::n1067(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n252(s, out),
		['y', s @ ..] => super::t1::n253(s, out),
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 254);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['s', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				['v', 'r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 256);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['n', 'i', 'g', 'i', 'l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2743);
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
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['r', 'e', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2746);
							match s {
								[] => true,
								['o', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'h', 't', 'y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2747);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2748);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4762);
									s.is_empty()
								}
								['i', 'c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4763);
									s.is_empty()
								}
								['o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 4764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2750);
							match s {
								[] => true,
								['a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4765);
									s.is_empty()
								}
								['e', 'h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2751);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2752);
							match s {
								[] => true,
								['g', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['n', 'e', 't', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['o', 'h', 'r', 'r', 'o', 'm', 'e', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2754);
							match s {
								[] => true,
								['r', 'e', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -ta.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2755);
							match s {
								[] => true,
								['d', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['o', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2758);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2759);
							match s {
								[] => true,
								['d', 'r', 'o', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4770);
									s.is_empty()
								}
								['m', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'm', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['c', 'o', 'm', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2761);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						['c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4772);
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['d', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2772);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2774);
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
				['a', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				['d', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2775);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2776);
							s.is_empty()
						}
						['t', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2780);
							match s {
								[] => true,
								['d', 'e', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4774);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2781);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2782);
							match s {
								[] => true,
								['e', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4777);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', 'l', 'n', 'i', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['o', 'n', 'e', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['h', 'c', 's', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				['l', 'e', 'a', 'r', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2787);
							match s {
								[] => true,
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4779);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'g', 'o', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2789);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 4781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2791);
							match s {
								[] => true,
								['a', 'h', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 4782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 267);
			s.is_empty()
		}
		['n', s @ ..] => super::t1::n268(s, out),
		['o', s @ ..] => super::t1::n269(s, out),
		['p', s @ ..] => super::t1::n270(s, out),
		['r', s @ ..] => super::t1::n271(s, out),
		['s', s @ ..] => super::t1::n272(s, out),
		['t', s @ ..] => super::t1::n273(s, out),
		['u', s @ ..] => super::t1::n274(s, out),
		['x', s @ ..] => super::t1::n275(s, out),
		['y', s @ ..] => super::t1::n276(s, out),
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['i', 'n', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['s', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['a', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['e', 's', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['a', 'g', 'o', 'r', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2822);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 's', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2825);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['y', 'w', 's', 'u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['d', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['f', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', 'r', 'r', 'y', 'c', 'y', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -um.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['n', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['b', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2889);
							match s {
								[] => true,
								['n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2890);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4805);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4806);
									match s {
										[] => true,
										['a', 'h', 'p', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 6626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1267);
					match s {
						[] => true,
						['b', 'm', 'e', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2896);
							match s {
								[] => true,
								['b', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2897);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4808);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4809);
									s.is_empty()
								}
								['o', 'c', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2898);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2899);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2900);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2901);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4813);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 6627);
											match s {
												[] => true,
												['o', 'r', 'a', '\u{1}', s @ ..] => {
													super::push(out, 8137);
													s.is_empty()
												}
												['r', 'o', 'f', '\u{1}', s @ ..] => {
													super::push(out, 8138);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 4814);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4815);
									match s {
										[] => true,
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 6629);
											s.is_empty()
										}
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
					super::push(out, 1268);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1269);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2903);
							match s {
								[] => true,
								['b', 'i', s @ ..] => {
									super::push(out, 4816);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2904);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 4817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2905);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2906);
							s.is_empty()
						}
						['d', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['p', 'i', 'p', 'e', 'l', 'e', 'l', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['d', 'o', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2911);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2912);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1276);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2913);
							match s {
								[] => true,
								['r', 'u', 'i', 's', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['d', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1278);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n372(s, out),
		['f', 'a', 'r', 'g', 'o', 'r', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['g', s @ ..] => super::t1::n374(s, out),
		['h', s @ ..] => super::t1::n375(s, out),
		['i', s @ ..] => super::t1::n376(s, out),
		['l', s @ ..] => super::t1::n377(s, out),
		['m', s @ ..] => super::t1::n378(s, out),
		['n', s @ ..] => super::t1::n379(s, out),
		['o', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['p', s @ ..] => super::t1::n381(s, out),
		['r', s @ ..] => super::t1::n382(s, out),
		['s', s @ ..] => super::t1::n383(s, out),
		['t', s @ ..] => super::t1::n384(s, out),
		['u', s @ ..] => super::t1::n385(s, out),
		['v', s @ ..] => super::t1::n386(s, out),
		['x', s @ ..] => super::t1::n387(s, out),
		['y', 'h', 't', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lamen.
pub(crate) fn n3273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3240);
	match s {
		[] => true,
		['c', 'y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5243);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5244);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 6950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5245);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'i', 't', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5246);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5247);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6952);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5248);
			match s {
				[] => true,
				['t', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ramen.
pub(crate) fn n3275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3242);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5249);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5250);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5251);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5252);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5253);
			match s {
				[] => true,
				['s', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5254);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ctamen.
pub(crate) fn n5324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5255);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6956);
			s.is_empty()
		}
		['n', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6957);
			s.is_empty()
		}
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utamen.
pub(crate) fn n5325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5256);
	match s {
		[] => true,
		['t', '\u{1}', s @ ..] => {
			super::push(out, 6959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vamen.
pub(crate) fn n3278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3245);
	s.is_empty()
}

/// Words ending -emen.
pub(crate) fn n1407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 3246);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gmen.
pub(crate) fn n1408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	s.is_empty()
}

/// Words ending -imen.
pub(crate) fn n1409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3247);
			match s {
				[] => true,
				['e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 5257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 3248);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3249);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3250);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3251);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5259);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 6960);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3252);
			match s {
				[] => true,
				['t', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3253);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lmen.
pub(crate) fn n1410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	s.is_empty()
}

/// Words ending -omen.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3254);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmen.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	s.is_empty()
}

/// Words ending -umen.
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	match s {
		[] => true,
		['g', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 3255);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			super::push(out, 3256);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 5262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 3257);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hymen.
pub(crate) fn n1414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	s.is_empty()
}

/// Words ending -ren.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ten.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['a', 'c', 'a', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1402);
			s.is_empty()
		}
		['u', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uen.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	s.is_empty()
}

/// The word delphin.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	s.is_empty()
}

/// Words ending -on.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1404);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 3259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3261);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						['t', 's', 'o', 'n', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['g', 'a', 'm', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1410);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['l', 'e', 'a', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['e', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3267);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3268);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['u', 'l', 'o', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3270);
							s.is_empty()
						}
						['y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3271);
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
				['b', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3273);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5263);
									s.is_empty()
								}
								['r', 'i', 'h', 'c', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3274);
							s.is_empty()
						}
						['e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3275);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3276);
							match s {
								[] => true,
								['y', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3277);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3278);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5267);
									match s {
										[] => true,
										['r', 't', 'o', 'o', 'z', '\u{1}', s @ ..] => {
											super::push(out, 6962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'i', 'n', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['e', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3279);
							s.is_empty()
						}
						['i', 'd', 'o', 'c', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3281);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3282);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 5268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 3283);
							match s {
								[] => true,
								['d', 'r', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 5269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3285);
							match s {
								[] => true,
								['r', 'c', 'o', 'h', 't', 's', 'i', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3286);
							match s {
								[] => true,
								['g', 'o', 'n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5271);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3288);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['a', 'd', 'i', 'g', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						['d', 'a', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3292);
							match s {
								[] => true,
								['t', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3293);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5274);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['a', 'm', 'e', 'r', 'o', 'e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', 'p', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['c', 'e', 'm', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n407(s, out),
		['n', s @ ..] => super::t1::n408(s, out),
		['o', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['y', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n411(s, out),
		['s', s @ ..] => super::t1::n412(s, out),
		['t', s @ ..] => super::t1::n413(s, out),
		['x', s @ ..] => super::t1::n414(s, out),
		['y', s @ ..] => super::t1::n415(s, out),
		['z', s @ ..] => super::t1::n416(s, out),
		_ => false,
	}
}

/// Words ending -tudo.
pub(crate) fn n1503(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3334);
			match s {
				[] => true,
				['b', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5282);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5283);
					match s {
						[] => true,
						['x', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5284);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8347);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 8348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5285);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8349);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8350);
									s.is_empty()
								}
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
			super::push(out, 3335);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5286);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 6968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5287);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5288);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5289);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5290);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6969);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 8351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6970);
							match s {
								[] => true,
								['m', 'i', 's', 'i', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6971);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 6972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5291);
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6973);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6974);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8353);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8354);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6975);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 8356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6977);
							match s {
								[] => true,
								['l', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6978);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6979);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6980);
							match s {
								[] => true,
								['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8359);
									s.is_empty()
								}
								['v', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6981);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6982);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 8361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5294);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5295);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6983);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6984);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6985);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6986);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6987);
							match s {
								[] => true,
								['m', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6989);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5296);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6990);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6991);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 8365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 5297);
					match s {
						[] => true,
						['i', 'l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6992);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['h', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1491);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3339);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 5298);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3345);
							match s {
								[] => true,
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3348);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3349);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3350);
							match s {
								[] => true,
								['p', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -io.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3354);
							s.is_empty()
						}
						['p', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3355);
							s.is_empty()
						}
						['r', 't', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						['s', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1508);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['t', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['m', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3362);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 5303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3363);
							match s {
								[] => true,
								['r', 'e', 'p', 's', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5304);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 3365);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3366);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1516);
					match s {
						[] => true,
						['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1520);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['c', 'u', 's', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3370);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3371);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['a', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
						['h', 'p', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3377);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n446(s, out),
		['t', s @ ..] => super::t1::n447(s, out),
		['v', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['i', 'l', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1556);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 5554);
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5555);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5556);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3468);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7357);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 8616);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 7358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 5558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3469);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1558);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		['e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 449);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3471);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3472);
							s.is_empty()
						}
						['t', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				['n', 'n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 456);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['h', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		['u', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['p', 'o', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['s', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', 'g', 'a', '\u{1}', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['g', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['l', 'i', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	s.is_empty()
}

/// Words ending -vo.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	s.is_empty()
}

/// Words ending -xo.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	s.is_empty()
}

/// The word pipizo.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	s.is_empty()
}

/// The word semicaper.
pub(crate) fn n3524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	s.is_empty()
}

/// Words ending -ser.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1609);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1610);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1611);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['e', 'h', 't', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3489);
					s.is_empty()
				}
				['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3490);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3491);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3492);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3493);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3494);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 3495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['a', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5569);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 3496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5571);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3498);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1616);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['a', 'h', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'l', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1619);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5573);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5574);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5575);
							match s {
								[] => true,
								['o', 'r', s @ ..] => {
									super::push(out, 7359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5576);
							s.is_empty()
						}
						['n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5577);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5579);
							match s {
								[] => true,
								['e', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3502);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3503);
					s.is_empty()
				}
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3505);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['m', 'e', '\u{1}', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3507);
							match s {
								[] => true,
								['i', 't', 'n', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3509);
							match s {
								[] => true,
								['c', 'e', 'd', 'n', s @ ..] => {
									super::push(out, 5581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['t', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['e', 's', 'i', 't', 'n', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1632);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3513);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
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
				['a', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['t', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['n', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1641);
					match s {
						[] => true,
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1643);
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['l', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3528);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3529);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 3530);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5583);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['u', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3533);
							match s {
								[] => true,
								['f', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3534);
							match s {
								[] => true,
								['p', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3535);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7362);
											s.is_empty()
										}
										['e', '\u{1}', s @ ..] => {
											super::push(out, 7363);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'v', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5587);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7364);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5588);
									match s {
										[] => true,
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7365);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => super::t1::n5670(s, out),
								['s', s @ ..] => {
									super::push(out, 5590);
									match s {
										[] => true,
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t1::n3576(s, out),
						['u', s @ ..] => super::t1::n3577(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n1683(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n517(s, out),
		['u', s @ ..] => super::t1::n518(s, out),
		['v', s @ ..] => super::t1::n519(s, out),
		['x', s @ ..] => super::t1::n520(s, out),
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 521);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yr.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['t', 'r', 'a', 'm', 'o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -as.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['o', 'h', 'p', 'o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['d', 'o', s @ ..] => {
							super::push(out, 3594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3595);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5743);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 5744);
									s.is_empty()
								}
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
			super::push(out, 528);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['e', 'l', 'p', 'm', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['s', 'a', 'r', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 3597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['c', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['a', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3600);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3601);
							match s {
								[] => true,
								['g', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['e', 'l', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3604);
							match s {
								[] => true,
								['h', 'c', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3605);
							s.is_empty()
						}
						['y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['y', 'n', 'o', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1699);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3608);
							s.is_empty()
						}
						['o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['e', 'm', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3610);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3613);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'y', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'b', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['e', 'l', 'b', 'o', 't', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['e', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'p', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 538);
			s.is_empty()
		}
		['t', s @ ..] => super::t1::n541(s, out),
		['v', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['i', 't', 'a', 'm', 'i', 't', 's', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['d', 'a', 'm', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['p', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1720);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3632);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 5817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['n', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3634);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3635);
							match s {
								[] => true,
								['o', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'm', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1725);
					match s {
						[] => true,
						['n', 'o', 'r', 't', 'a', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1726);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1727);
					match s {
						[] => true,
						['i', 'p', 'e', 'l', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3638);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3639);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3641);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3642);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3643);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3645);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5819);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3646);
							match s {
								[] => true,
								['m', 'a', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 5821);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5822);
									match s {
										[] => true,
										['y', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5823);
									match s {
										[] => true,
										['d', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'o', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['f', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['t', 's', 'a', 'm', 'o', 'r', 'e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'r', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['r', 'a', 'i', s @ ..] => {
							super::push(out, 3650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['a', 'l', 'l', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3652);
							match s {
								[] => true,
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5824);
									s.is_empty()
								}
								['u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 5825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['e', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3654);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3655);
							match s {
								[] => true,
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5826);
									s.is_empty()
								}
								['c', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5827);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 5828);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 7627);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 7628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 5829);
									s.is_empty()
								}
								['t', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n1767(s, out),
				['u', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1750);
					match s {
						[] => true,
						['u', 'l', s @ ..] => super::t1::n3699(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n555(s, out),
		['m', s @ ..] => super::t1::n556(s, out),
		['n', s @ ..] => super::t1::n557(s, out),
		['p', s @ ..] => super::t1::n558(s, out),
		['r', s @ ..] => super::t1::n559(s, out),
		['s', s @ ..] => super::t1::n560(s, out),
		['t', s @ ..] => super::t1::n561(s, out),
		['u', s @ ..] => super::t1::n562(s, out),
		['v', 'i', 'd', '\u{1}', s @ ..] => super::t1::n563(s, out),
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1797);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['r', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['a', 'r', 'h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['t', 's', 'o', 'c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3729);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7642);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 7643);
											match s {
												[] => true,
												['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
													super::push(out, 8764);
													s.is_empty()
												}
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
							super::push(out, 3730);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5872);
									match s {
										[] => true,
										['a', 'p', 'o', 'l', 'e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3731);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5873);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 7645);
											match s {
												[] => true,
												['t', 'c', 'i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 8765);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7646);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5874);
									match s {
										[] => true,
										['e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7647);
											s.is_empty()
										}
										['i', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7648);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3733);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5875);
									match s {
										[] => true,
										['s', 'u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 7649);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5876);
									s.is_empty()
								}
								['y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3734);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5878);
									s.is_empty()
								}
								['o', 'd', 'r', 'e', 'c', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 3736);
							match s {
								[] => true,
								['i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['c', 's', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3738);
							s.is_empty()
						}
						['s', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3739);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3740);
							match s {
								[] => true,
								['r', 't', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 7650);
											s.is_empty()
										}
										['r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7651);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['c', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						['h', 'p', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 3743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['p', 'o', 'r', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['l', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 5881);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3748);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n573(s, out),
		['o', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				['t', 'n', 'a', 'l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n575(s, out),
		['r', s @ ..] => super::t1::n576(s, out),
		['s', s @ ..] => super::t1::n577(s, out),
		['t', s @ ..] => super::t1::n578(s, out),
		['u', s @ ..] => super::t1::n579(s, out),
		['v', s @ ..] => super::t1::n580(s, out),
		['x', s @ ..] => super::t1::n581(s, out),
		_ => false,
	}
}

/// Words ending -bos.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 585);
	s.is_empty()
}

/// Words ending -cos.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1884);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1885);
			s.is_empty()
		}
		['e', 'n', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dos.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1887);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['e', 'c', 'a', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3852);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eos.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['g', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1890);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1891);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 3853);
					s.is_empty()
				}
				['l', 'i', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3854);
					s.is_empty()
				}
				['o', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gos.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	s.is_empty()
}

/// Words ending -hos.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['i', 'r', 'a', 't', 'o', 'r', 'y', 't', '\u{1}', s @ ..] => {
					super::push(out, 3856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 3857);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3858);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['a', 'c', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3860);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ios.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['r', 'a', 'c', 'i', 'l', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 3861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -los.
pub(crate) fn n594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['d', 'r', 'a', 'c', 'i', 'h', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1899);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 1900);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1901);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1902);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1903);
			match s {
				[] => true,
				['b', 'o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3864);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5923);
							s.is_empty()
						}
						['s', 'o', 'r', 'p', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mos.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 593);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1904);
			s.is_empty()
		}
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1906);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['a', 'h', 't', 'h', 'p', 'o', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1908);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nos.
pub(crate) fn n596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['l', 'p', 's', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1912);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				['m', 'a', 'l', 'c', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1914);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'r', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1915);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word heroos.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	s.is_empty()
}

/// Words ending -pos.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 596);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3874);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3875);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1917);
			s.is_empty()
		}
		['r', 'a', 'c', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ros.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1919);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1920);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1921);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3876);
					match s {
						[] => true,
						['o', 'n', 'i', s @ ..] => {
							super::push(out, 5927);
							match s {
								[] => true,
								['h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7666);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 7667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3877);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3878);
					s.is_empty()
				}
				['p', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 3880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['p', 'e', 'n', 'u', 'd', 'a', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1924);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['h', 'p', 'o', 's', 'i', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'c', 'e', 'p', 'o', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1927);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1928);
			match s {
				[] => true,
				['h', 't', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sos.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['o', 'l', 'g', 'o', 'n', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tos.
pub(crate) fn n601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1931);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1932);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1933);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5928);
							s.is_empty()
						}
						['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1934);
			s.is_empty()
		}
		['p', 'e', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1935);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['a', 'r', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3888);
					s.is_empty()
				}
				['e', 'b', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3889);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zos.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	s.is_empty()
}

/// Words ending -ps.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1938);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3891);
							match s {
								[] => true,
								['n', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5930);
									s.is_empty()
								}
								['r', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3892);
							match s {
								[] => true,
								['i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 604);
			match s {
				[] => true,
				['c', 'u', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				['h', 'c', 'n', 'y', 'h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1943);
					match s {
						[] => true,
						['c', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3893);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3894);
							match s {
								[] => true,
								['n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				['r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 608);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3897);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['o', 'h', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3901);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5935);
									match s {
										[] => true,
										['l', 'o', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7668);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'y', 'h', 't', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3903);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3904);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1957);
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
					super::push(out, 1958);
					match s {
						[] => true,
						['h', 't', 'i', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3905);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3906);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5939);
									match s {
										[] => true,
										['o', 'z', '\u{1}', s @ ..] => {
											super::push(out, 7669);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'u', 's', 'y', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3907);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1960);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3910);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 't', 'i', 'p', 'o', 'c', 'r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3911);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n1980(s, out),
				['l', s @ ..] => {
					super::push(out, 1962);
					match s {
						[] => true,
						['u', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1963);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3926);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3928);
							match s {
								[] => true,
								['m', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5972);
									s.is_empty()
								}
								['t', 'n', 'o', 'c', 'e', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3929);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3930);
							s.is_empty()
						}
						['i', s @ ..] => super::t1::n3976(s, out),
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n1986(s, out),
				['y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n613(s, out),
		['e', s @ ..] => super::t1::n614(s, out),
		['f', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n616(s, out),
		['h', s @ ..] => super::t1::n617(s, out),
		['i', s @ ..] => super::t1::n618(s, out),
		['l', s @ ..] => super::t1::n619(s, out),
		['m', s @ ..] => super::t1::n620(s, out),
		['n', s @ ..] => super::t1::n621(s, out),
		['o', s @ ..] => super::t1::n622(s, out),
		['p', s @ ..] => super::t1::n623(s, out),
		['r', s @ ..] => super::t1::n624(s, out),
		['s', s @ ..] => super::t1::n625(s, out),
		['t', s @ ..] => super::t1::n626(s, out),
		['u', s @ ..] => super::t1::n627(s, out),
		['v', s @ ..] => super::t1::n628(s, out),
		['x', s @ ..] => super::t1::n629(s, out),
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4325);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'a', 'i', '\u{1}', s @ ..] => {
			super::push(out, 630);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['h', 'p', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['t', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urex.
pub(crate) fn n2168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2146);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 4347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tex.
pub(crate) fn n652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ix.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2149);
					match s {
						[] => true,
						['r', 'i', 'v', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2152);
					match s {
						[] => true,
						['e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2153);
					match s {
						[] => true,
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'o', 'h', 't', '\u{1}', s @ ..] => {
			super::push(out, 655);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2156);
					match s {
						[] => true,
						['v', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2157);
					match s {
						[] => true,
						['t', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2158);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6397);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 6398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4354);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4355);
							match s {
								[] => true,
								['t', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['m', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4356);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'i', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				['t', s @ ..] => super::t1::n2184(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['s', 'a', 'm', 'o', 'i', 't', 'e', 'v', 'l', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lx.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	s.is_empty()
}

/// Words ending -nx.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2165);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2166);
					match s {
						[] => true,
						['y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['c', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ox.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['d', 'a', 'p', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 667);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['r', 'b', 'o', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['s', 'e', '\u{1}', s @ ..] => {
			super::push(out, 670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rx.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	s.is_empty()
}

/// Words ending -ux.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yx.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 675);
			s.is_empty()
		}
		['c', 'c', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		['d', 'n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 679);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word patricida.
pub(crate) fn n9115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8958);
	s.is_empty()
}

/// Words ending -eia.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['b', 'u', 'b', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 798);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['u', 't', 'u', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4425);
							s.is_empty()
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'o', 't', 'a', 'm', 'o', 'n', 'o', '\u{1}', s @ ..] => {
			super::push(out, 800);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 801);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gia.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['h', 'p', 'o', 'n', 'e', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				['l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['p', 'o', 'n', 'e', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4428);
							match s {
								[] => true,
								['r', 't', 's', 'o', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['f', 'f', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6454);
									s.is_empty()
								}
								['r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4430);
							match s {
								[] => true,
								['e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6456);
									s.is_empty()
								}
								['r', 'e', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6458);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 8027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6459);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													super::push(out, 8959);
													s.is_empty()
												}
												['i', 'n', 'r', 'o', '\u{1}', s @ ..] => {
													super::push(out, 8960);
													s.is_empty()
												}
												['y', s @ ..] => {
													super::push(out, 8961);
													match s {
														[] => true,
														['m', '\u{1}', s @ ..] => super::t1::n9512(s, out),
														['z', '\u{1}', s @ ..] => super::t1::n9513(s, out),
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
									super::push(out, 6460);
									match s {
										[] => true,
										['h', 'c', 'e', 'o', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 8028);
											s.is_empty()
										}
										['r', 'e', 't', 'c', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 8029);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6461);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6462);
									match s {
										[] => true,
										['a', 'r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 8030);
											s.is_empty()
										}
										['e', 'h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8031);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 8032);
											s.is_empty()
										}
										['o', 'm', 'u', 'e', 'n', 'h', 'c', 'i', '\u{1}', s @ ..] => {
											super::push(out, 8033);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 6463);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6464);
									match s {
										[] => true,
										['o', 'r', 'h', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8034);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6465);
									match s {
										[] => true,
										['o', 'e', 't', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8035);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6466);
									s.is_empty()
								}
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
			super::push(out, 808);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['h', 't', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4432);
							s.is_empty()
						}
						['t', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 6467);
									s.is_empty()
								}
								['y', 's', '\u{1}', s @ ..] => {
									super::push(out, 6468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['p', 'i', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2295);
					match s {
						[] => true,
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hia.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', 's', 'y', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6469);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6470);
									match s {
										[] => true,
										['c', 'y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2300);
					match s {
						[] => true,
						['a', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2301);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 6471);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6472);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 8037);
											s.is_empty()
										}
										['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8038);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 8039);
											match s {
												[] => true,
												['a', 'r', 'u', '\u{1}', s @ ..] => {
													super::push(out, 8962);
													s.is_empty()
												}
												['e', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 8963);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 8964);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 8040);
											match s {
												[] => true,
												['o', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 8965);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 8041);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 8966);
													s.is_empty()
												}
												['o', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 8967);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 8042);
											s.is_empty()
										}
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
					super::push(out, 2302);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['m', 'o', 't', 's', 'e', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

