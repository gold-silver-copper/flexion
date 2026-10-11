use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => super::t0::n1(s, out),
		['β', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 45);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 320);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 321);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 322);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 323);
							match s {
								[] => true,
								['λ', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1302);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1303);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 324);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 325);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 326);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 857);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 327);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 859);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 860);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 861);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 329);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 48);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 331);
							match s {
								[] => true,
								['ζ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 332);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 864);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 333);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 50);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 335);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 866);
									s.is_empty()
								}
								['σ', s @ ..] => {
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 1304);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 1305);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 336);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 337);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 51);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 339);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 340);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 341);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 53);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 342);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 343);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 345);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 346);
							match s {
								[] => true,
								['ε', 'α', s @ ..] => {
									super::push(out, 868);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 56);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 347);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 348);
							match s {
								[] => true,
								['ω', 'σ', 'κ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 349);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 351);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 353);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 355);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n4(s, out),
		['ε', s @ ..] => super::t0::n5(s, out),
		['ζ', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 85);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 86);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 87);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 437);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n8(s, out),
		['ι', s @ ..] => super::t0::n9(s, out),
		['κ', s @ ..] => super::t0::n10(s, out),
		['λ', s @ ..] => super::t0::n11(s, out),
		['μ', s @ ..] => super::t0::n12(s, out),
		['ν', s @ ..] => super::t0::n13(s, out),
		['ξ', s @ ..] => super::t0::n14(s, out),
		['ο', s @ ..] => super::t0::n15(s, out),
		['π', s @ ..] => super::t0::n16(s, out),
		['σ', s @ ..] => super::t0::n17(s, out),
		['τ', s @ ..] => super::t0::n18(s, out),
		['υ', s @ ..] => super::t0::n19(s, out),
		['φ', s @ ..] => super::t0::n20(s, out),
		['χ', s @ ..] => super::t0::n21(s, out),
		['ψ', s @ ..] => {
			super::push(out, 22);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 213);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						['ω', s @ ..] => super::t0::n735(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n214(s, out),
				['η', s @ ..] => super::t0::n215(s, out),
				['ι', s @ ..] => super::t0::n216(s, out),
				['υ', s @ ..] => super::t0::n217(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t0::n23(s, out),
		['ῥ', s @ ..] => super::t0::n24(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['α', 'ω', '\u{1}', s @ ..] => {
			super::push(out, 25);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 226);
					match s {
						[] => true,
						['λ', 'λ', 'ι', 'α', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 732);
							s.is_empty()
						}
						['ν', 'α', 'κ', 'τ', 'ε', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 229);
					match s {
						[] => true,
						['ζ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 231);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 735);
							match s {
								[] => true,
								['ζ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 232);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 1544);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 1545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 1181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', 'ε', s @ ..] => {
					super::push(out, 233);
					s.is_empty()
				}
				['χ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						['ω', '\u{1}', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 738);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						['ρ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 29);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 239);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['ρ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 246);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 744);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 249);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						['ι', 'α', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 250);
					match s {
						[] => true,
						['μ', 'α', s @ ..] => {
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 1182);
									s.is_empty()
								}
								['λ', 'ω', s @ ..] => {
									super::push(out, 1183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 253);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 749);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1184);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 254);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 760);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['α', s @ ..] => super::t0::n264(s, out),
				['β', s @ ..] => super::t0::n265(s, out),
				['ε', s @ ..] => super::t0::n266(s, out),
				['η', s @ ..] => super::t0::n267(s, out),
				['ν', s @ ..] => super::t0::n268(s, out),
				['ο', s @ ..] => super::t0::n269(s, out),
				['υ', s @ ..] => super::t0::n270(s, out),
				['φ', s @ ..] => super::t0::n271(s, out),
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n35(s, out),
		['ξ', 'ι', 'ο', 'ω', '\u{1}', s @ ..] => super::t0::n36(s, out),
		['ο', s @ ..] => super::t0::n37(s, out),
		['π', s @ ..] => super::t0::n38(s, out),
		['ρ', s @ ..] => super::t0::n39(s, out),
		['σ', s @ ..] => super::t0::n40(s, out),
		['τ', s @ ..] => super::t0::n41(s, out),
		['υ', s @ ..] => super::t0::n42(s, out),
		['φ', s @ ..] => super::t0::n43(s, out),
		['χ', s @ ..] => super::t0::n44(s, out),
		_ => false,
	}
}

/// Words ending -δ.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 357);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 360);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 362);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 364);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['μ', 'ι', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 875);
							match s {
								[] => true,
								['α', 'λ', s @ ..] => {
									super::push(out, 1306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 876);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1307);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1308);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1591);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 1592);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 878);
							match s {
								[] => true,
								['ω', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1593);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 1594);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 880);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1309);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1310);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1311);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 1595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1313);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1314);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1596);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 1315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 885);
							match s {
								[] => true,
								['ρ', 'ε', s @ ..] => {
									super::push(out, 1316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1317);
									s.is_empty()
								}
								['κ', s @ ..] => {
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1598);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 1710);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 1711);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 1599);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1318);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 887);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1320);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 888);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 889);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1323);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 373);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 375);
					match s {
						[] => true,
						['λ', s @ ..] => super::t0::n912(s, out),
						_ => false,
					}
				}
				['σ', s @ ..] => super::t0::n378(s, out),
				['υ', s @ ..] => super::t0::n379(s, out),
				['ψ', s @ ..] => super::t0::n380(s, out),
				['ω', s @ ..] => super::t0::n381(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n65(s, out),
		['ρ', s @ ..] => super::t0::n66(s, out),
		['υ', s @ ..] => super::t0::n67(s, out),
		['ω', s @ ..] => super::t0::n68(s, out),
		_ => false,
	}
}

/// Words ending -ε.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 391);
					match s {
						[] => true,
						['ι', 'γ', s @ ..] => {
							super::push(out, 897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 898);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1329);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1330);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1331);
									match s {
										[] => true,
										['α', 'μ', s @ ..] => {
											super::push(out, 1600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 899);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 900);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1332);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 901);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1334);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1335);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 903);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 906);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 398);
					match s {
						[] => true,
						['ι', 'δ', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 911);
							match s {
								[] => true,
								['μ', 'π', 'λ', s @ ..] => {
									super::push(out, 1337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 912);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1338);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 914);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 915);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 916);
							s.is_empty()
						}
						['ρ', 'χ', s @ ..] => {
							super::push(out, 917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 408);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 918);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 919);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 1601);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 1602);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 412);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 414);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 416);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['ρ', 'ε', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1341);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 923);
							s.is_empty()
						}
						['κ', 'ο', 'λ', 'ο', 'υ', 'θ', 'ε', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 927);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1343);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n419(s, out),
				['ι', s @ ..] => super::t0::n420(s, out),
				['ο', s @ ..] => super::t0::n421(s, out),
				['υ', s @ ..] => super::t0::n422(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n77(s, out),
		['π', s @ ..] => super::t0::n78(s, out),
		['ρ', s @ ..] => super::t0::n79(s, out),
		['σ', s @ ..] => super::t0::n80(s, out),
		['τ', s @ ..] => super::t0::n81(s, out),
		['υ', s @ ..] => super::t0::n82(s, out),
		['φ', s @ ..] => super::t0::n83(s, out),
		['χ', s @ ..] => super::t0::n84(s, out),
		_ => false,
	}
}

/// Words ending -θ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 439);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['ο', 'μ', 'α', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 975);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 91);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 92);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 447);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 981);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ι.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 100);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1382);
									match s {
										[] => true,
										['ω', '\u{1}', s @ ..] => {
											super::push(out, 1621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κ.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 987);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1384);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1385);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 988);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 989);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 990);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1387);
									s.is_empty()
								}
								['ζ', s @ ..] => {
									super::push(out, 1388);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1622);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1623);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 1389);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['λ', 'ι', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 994);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 996);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1391);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 1392);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1624);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['δ', s @ ..] => {
									super::push(out, 1393);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1626);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1627);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 1628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1394);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1395);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1629);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1630);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 1714);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 1715);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 1631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 1396);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1632);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 1633);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1398);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1634);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 1635);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 1636);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1400);
									match s {
										[] => true,
										['κ', 'η', s @ ..] => {
											super::push(out, 1637);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => super::t0::n1453(s, out),
								['φ', 'ρ', s @ ..] => {
									super::push(out, 1402);
									s.is_empty()
								}
								['χ', s @ ..] => super::t0::n1455(s, out),
								_ => false,
							}
						}
						['ε', s @ ..] => super::t0::n1024(s, out),
						['η', s @ ..] => super::t0::n1025(s, out),
						['ι', s @ ..] => super::t0::n1026(s, out),
						['ο', s @ ..] => super::t0::n1027(s, out),
						_ => false,
					}
				}
				['υ', s @ ..] => super::t0::n486(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n111(s, out),
		['η', s @ ..] => super::t0::n112(s, out),
		['ι', s @ ..] => super::t0::n113(s, out),
		['λ', s @ ..] => super::t0::n114(s, out),
		['ν', s @ ..] => super::t0::n115(s, out),
		['ο', s @ ..] => super::t0::n116(s, out),
		['ρ', s @ ..] => super::t0::n117(s, out),
		['τ', s @ ..] => super::t0::n118(s, out),
		['υ', s @ ..] => super::t0::n119(s, out),
		['ω', s @ ..] => super::t0::n120(s, out),
		_ => false,
	}
}

/// Words ending -λ.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['ω', '\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1408);
									s.is_empty()
								}
								['ζ', s @ ..] => {
									super::push(out, 1409);
									s.is_empty()
								}
								['χ', 'ε', 'υ', 'ο', 'μ', 'α', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['τ', 'ρ', 'ε', 'υ', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1017);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1413);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1019);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1020);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1021);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1022);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1023);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['γ', 'ι', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μ.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['κ', 'α', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				['σ', 'τ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1027);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1028);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1030);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1415);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								['λ', 'α', 'γ', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1418);
									s.is_empty()
								}
								['π', 'ρ', s @ ..] => {
									super::push(out, 1419);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1420);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1421);
									s.is_empty()
								}
								['φ', 'ε', s @ ..] => {
									super::push(out, 1422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['α', 'ο', 'μ', 'α', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1645);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1646);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1424);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1425);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1647);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 1648);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1037);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 560);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['ω', '\u{1}', s @ ..] => {
							super::push(out, 1041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 137);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 138);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['σ', s @ ..] => {
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1042);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'τ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξ.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 143);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 1044);
							match s {
								[] => true,
								['θ', 'α', 'ι', 'ρ', 'ε', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1426);
									s.is_empty()
								}
								['τ', 'α', 'σ', 'κ', 'ε', 'υ', 'α', 'ζ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['λ', 'α', 'μ', 'β', 'α', 'ν', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['β', 'ο', 'υ', 'λ', 'ε', 'υ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1046);
							s.is_empty()
						}
						['μ', s @ ..] => {
							match s {
								[] => true,
								['α', 'χ', 'ε', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1428);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['ο', 'λ', 'ε', 'μ', 'ε', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1430);
									s.is_empty()
								}
								['ρ', 'ο', 'π', 'ε', 'μ', 'π', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', 'ε', 'ρ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1048);
							match s {
								[] => true,
								['δ', 'ι', 'κ', 'ε', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1432);
									s.is_empty()
								}
								['λ', 'λ', 'α', 'τ', 'τ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1433);
									s.is_empty()
								}
								['φ', 'ι', 'σ', 'τ', 'η', 'μ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', 'π', s @ ..] => {
							super::push(out, 1049);
							match s {
								[] => true,
								['ι', 'τ', 'ι', 'θ', 'η', 'μ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', 'α', 'π', 'τ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['ν', 'α', 'υ', 'μ', 'α', 'χ', 'ε', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						['τ', 'ι', 'θ', 'η', 'μ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ο.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['υ', 'ρ', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['γ', 'ο', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						['τ', 'τ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -π.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1436);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 1437);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 1649);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1650);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1651);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1652);
											s.is_empty()
										}
										_ => false,
									}
								}
								['δ', s @ ..] => {
									super::push(out, 1438);
									match s {
										[] => true,
										['ε', 'ι', 'γ', s @ ..] => {
											super::push(out, 1653);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1439);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1440);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1441);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 1654);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1442);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1443);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1444);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1061);
							match s {
								[] => true,
								['ι', 'μ', s @ ..] => {
									super::push(out, 1445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1062);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1064);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						['χ', s @ ..] => {
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1447);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1068);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1449);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1070);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1071);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1072);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1451);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1656);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1657);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 1452);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1453);
									s.is_empty()
								}
								['κ', 'λ', s @ ..] => {
									super::push(out, 1454);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1455);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1456);
									s.is_empty()
								}
								['σ', 'κ', s @ ..] => {
									super::push(out, 1457);
									s.is_empty()
								}
								['τ', 'ε', 'ι', s @ ..] => {
									super::push(out, 1458);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 1459);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 1460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1074);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['α', 'ι', s @ ..] => super::t0::n615(s, out),
				['δ', s @ ..] => super::t0::n616(s, out),
				['ε', s @ ..] => super::t0::n617(s, out),
				['λ', s @ ..] => super::t0::n618(s, out),
				['μ', s @ ..] => super::t0::n619(s, out),
				['ν', s @ ..] => super::t0::n620(s, out),
				['π', s @ ..] => super::t0::n621(s, out),
				['σ', s @ ..] => super::t0::n622(s, out),
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n157(s, out),
		['ν', s @ ..] => super::t0::n158(s, out),
		['ο', s @ ..] => super::t0::n159(s, out),
		['ρ', s @ ..] => super::t0::n160(s, out),
		['τ', s @ ..] => super::t0::n161(s, out),
		['υ', s @ ..] => super::t0::n162(s, out),
		['ω', s @ ..] => super::t0::n163(s, out),
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1103);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1104);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1105);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1107);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 1485);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1109);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1111);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1112);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1114);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1117);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1488);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1118);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1121);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => super::t0::n1158(s, out),
						['χ', s @ ..] => super::t0::n1159(s, out),
						_ => false,
					}
				}
				['ζ', s @ ..] => super::t0::n666(s, out),
				['λ', s @ ..] => super::t0::n667(s, out),
				['μ', s @ ..] => super::t0::n668(s, out),
				['ν', s @ ..] => super::t0::n669(s, out),
				['ρ', s @ ..] => super::t0::n670(s, out),
				['σ', s @ ..] => super::t0::n671(s, out),
				['χ', s @ ..] => super::t0::n672(s, out),
				_ => false,
			}
		}
		['φ', s @ ..] => super::t0::n175(s, out),
		['χ', s @ ..] => super::t0::n176(s, out),
		['ω', s @ ..] => super::t0::n177(s, out),
		_ => false,
	}
}

/// Words ending -τ.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1146);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 180);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 681);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1152);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1153);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1154);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υ.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 186);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 187);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 188);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1159);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['ρ', 'α', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1527);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', 'ε', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 1686);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 1687);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 1529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1163);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1530);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1531);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1532);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1164);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1166);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1535);
									s.is_empty()
								}
								['τ', s @ ..] => {
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 1688);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1168);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1536);
									s.is_empty()
								}
								['ε', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1690);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 1691);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1537);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1538);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1692);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1693);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1169);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1170);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1539);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φ.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['ρ', 'ω', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 703);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['υ', 'α', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['α', 'κ', 'ι', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χ.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['α', 'ω', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1541);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1177);
							match s {
								[] => true,
								['ζ', 'ο', s @ ..] => {
									super::push(out, 1543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['α', 'ο', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωαψ.
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 725);
	s.is_empty()
}

/// Words ending -εψ.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	s.is_empty()
}

/// Words ending -ηψ.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	s.is_empty()
}

/// Words ending -ιψ.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	s.is_empty()
}

/// Words ending -υψ.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	s.is_empty()
}

/// Words ending -ω.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['υ', 'ο', 'μ', 'α', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥ.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αμα.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['υ', 'σ', 'σ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -βμα.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	s.is_empty()
}

/// Words ending -εμα.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 765);
			s.is_empty()
		}
		['λ', s @ ..] => {
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ημα.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	s.is_empty()
}

/// Words ending -νμα.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	s.is_empty()
}

/// Words ending -ομα.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	s.is_empty()
}

/// Words ending -υμα.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 767);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φμα.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		['ι', s @ ..] => {
			match s {
				[] => true,
				['β', 'α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['κ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1196);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1197);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1199);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1200);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1202);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1205);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1208);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1209);
							match s {
								[] => true,
								['σ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1210);
							match s {
								[] => true,
								['ι', s @ ..] => {
									match s {
										[] => true,
										['ν', 'ο', 'ω', '\u{1}', s @ ..] => {
											super::push(out, 1694);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1695);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 1551);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1696);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 1697);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 1552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1211);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1553);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1212);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1213);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1214);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1220);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1221);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1556);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1223);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1558);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1226);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1560);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 1561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1698);
											s.is_empty()
										}
										['χ', s @ ..] => {
											super::push(out, 1699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['ρ', s @ ..] => super::t0::n801(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n274(s, out),
		['η', s @ ..] => super::t0::n275(s, out),
		['θ', s @ ..] => super::t0::n276(s, out),
		['ι', s @ ..] => super::t0::n277(s, out),
		['ο', s @ ..] => super::t0::n278(s, out),
		['τ', s @ ..] => super::t0::n279(s, out),
		['υ', s @ ..] => super::t0::n280(s, out),
		_ => false,
	}
}

/// The word ωοιξα.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	s.is_empty()
}

/// Words ending -οα.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	s.is_empty()
}

/// Words ending -πα.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1263);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 1706);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 1707);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['μ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 815);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['χ', s @ ..] => {
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['σ', 'τ', 'ε', s @ ..] => {
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						['ι', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1573);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 1574);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 1708);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 1709);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1281);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1575);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1286);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 825);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1289);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1580);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => super::t0::n844(s, out),
				['π', s @ ..] => super::t0::n845(s, out),
				['ρ', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				['σ', s @ ..] => super::t0::n847(s, out),
				['τ', s @ ..] => super::t0::n848(s, out),
				['φ', 'α', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['χ', 'ω', s @ ..] => super::t0::n850(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t0::n286(s, out),
		['ω', s @ ..] => super::t0::n287(s, out),
		_ => false,
	}
}

/// Words ending -ρα.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υα.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	s.is_empty()
}

/// Words ending -φα.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 850);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χα.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	s.is_empty()
}

/// Words ending -λπιδ.
pub(crate) fn n912(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1325);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιδ.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	s.is_empty()
}

/// Words ending -υιδ.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	s.is_empty()
}

/// Words ending -ψιδ.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	s.is_empty()
}

/// Words ending -ωιδ.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οδ.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['μ', 'α', 'τ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρδ.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υδ.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['ω', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωδ.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	s.is_empty()
}

/// Words ending -εξε.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	s.is_empty()
}

/// Words ending -ιξε.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 931);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 932);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οξε.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	s.is_empty()
}

/// Words ending -υξε.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	s.is_empty()
}

/// Words ending -οε.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	s.is_empty()
}

/// Words ending -πε.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['γ', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1348);
							match s {
								[] => true,
								['λ', 'υ', s @ ..] => {
									super::push(out, 1603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['α', 'λ', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['ι', 'γ', 'ν', 'ο', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1355);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1604);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1357);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1361);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1365);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1609);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1611);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1612);
									s.is_empty()
								}
								['τ', 'τ', 'ω', '\u{1}', s @ ..] => {
									super::push(out, 1613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1614);
									s.is_empty()
								}
								['ε', 'λ', s @ ..] => {
									super::push(out, 1615);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['ε', 'λ', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1617);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1375);
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1712);
											s.is_empty()
										}
										['χ', s @ ..] => {
											super::push(out, 1713);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1381);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρε.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 428);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['κ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 964);
					s.is_empty()
				}
				['ω', '\u{1}', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σε.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['β', 'ι', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['β', 'α', 'ζ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τε.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	s.is_empty()
}

/// Words ending -υε.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['χ', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 967);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φε.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χε.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	s.is_empty()
}

/// Words ending -τατακ.
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1401);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χατακ.
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1403);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1639);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ετακ.
pub(crate) fn n1024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	s.is_empty()
}

/// Words ending -ητακ.
pub(crate) fn n1025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 998);
	s.is_empty()
}

/// Words ending -ιτακ.
pub(crate) fn n1026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 999);
	s.is_empty()
}

/// Words ending -οτακ.
pub(crate) fn n1027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1000);
	match s {
		[] => true,
		['ι', 'κ', s @ ..] => {
			match s {
				[] => true,
				['ε', 'ω', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				['ο', 'ν', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υακ.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	s.is_empty()
}

/// Words ending -εκ.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['α', 'ν', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηκ.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	s.is_empty()
}

/// Words ending -ικ.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λκ.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	s.is_empty()
}

/// Words ending -νκ.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['α', 'π', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκ.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['α', 'κ', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκ.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1643);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['μ', 'α', 'ν', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['ν', 'υ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τκ.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υκ.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωκ.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	s.is_empty()
}

/// Words ending -ιαιπ.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	s.is_empty()
}

/// Words ending -διπ.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	s.is_empty()
}

/// Words ending -ειπ.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	s.is_empty()
}

/// Words ending -λιπ.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	s.is_empty()
}

/// Words ending -μιπ.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	s.is_empty()
}

/// Words ending -νιπ.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πιπ.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιπ.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	s.is_empty()
}

/// Words ending -λπ.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['ζ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 616);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 617);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νπ.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	s.is_empty()
}

/// Words ending -οπ.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρπ.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['γ', 'ω', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1084);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['ι', 'α', 'γ', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['ι', s @ ..] => {
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 1718);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 1719);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1469);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1658);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1659);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 1729);
													s.is_empty()
												}
												['λ', 'α', s @ ..] => {
													super::push(out, 1730);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1475);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1660);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['γ', 'ο', s @ ..] => {
									super::push(out, 1662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1480);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1663);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 1720);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 1721);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1481);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1664);
									match s {
										[] => true,
										['ρ', 'α', s @ ..] => {
											super::push(out, 1722);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 1665);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 1723);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['ι', 'μ', s @ ..] => {
							super::push(out, 1483);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1666);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τπ.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υπ.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 626);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωπ.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	s.is_empty()
}

/// Words ending -κγυσ.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1122);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1491);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1492);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1494);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χγυσ.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ζυσ.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	s.is_empty()
}

/// Words ending -λυσ.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1124);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μυσ.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['α', 'χ', 'ο', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['ρ', 'α', 'ζ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 1508);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νυσ.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1676);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1520);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1135);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυσ.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1138);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συσ.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['τ', 'ε', 'λ', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χυσ.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	s.is_empty()
}

/// Words ending -φσ.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χσ.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	s.is_empty()
}

/// Words ending -ωσ.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	s.is_empty()
}

/// Words ending -ρδνα.
pub(crate) fn n801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 787);
	s.is_empty()
}

/// Words ending -ενα.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 791);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 792);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηνα.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['κ', s @ ..] => {
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θνα.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 793);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ινα.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 795);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ονα.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['ι', s @ ..] => {
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τνα.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1700);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1702);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['θ', 'ι', s @ ..] => {
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 1727);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 1728);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1704);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 1731);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 1732);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 801);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνα.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	s.is_empty()
}

/// Words ending -νοπα.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποπα.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['ε', s @ ..] => {
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σοπα.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 829);
	match s {
		[] => true,
		['τ', 'ε', 'ρ', s @ ..] => {
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τοπα.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 830);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['ν', 'α', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωχοπα.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 832);
	s.is_empty()
}

/// Words ending -τπα.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	s.is_empty()
}

/// Words ending -ωπα.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	s.is_empty()
}

