use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['ι', 'ο', 'ν', 'υ', 'ξ', s @ ..] => {
							super::push(out, 49);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 13);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 3);
			s.is_empty()
		}
		['ι', s @ ..] => super::t0::n4(s, out),
		['ν', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 21);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 22);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 23);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 60);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 61);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 24);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 62);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 63);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 254);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 64);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 256);
									match s {
										[] => true,
										['ι', 'κ', 'ε', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 65);
							match s {
								[] => true,
								['ι', 'ε', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 798);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 1527);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 66);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 25);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['ω', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -ι.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n15(s, out),
		['ε', s @ ..] => {
			super::push(out, 16);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 245);
							match s {
								[] => true,
								['ι', 'τ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1487);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2082);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 2083);
													s.is_empty()
												}
												['ρ', s @ ..] => {
													super::push(out, 2084);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 2085);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 1488);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 1489);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 1490);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2086);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2087);
													match s {
														[] => true,
														['ξ', '\u{1}', s @ ..] => super::t0::n2503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 1491);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 1492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => super::t0::n247(s, out),
						['λ', s @ ..] => {
							super::push(out, 247);
							match s {
								[] => true,
								['π', 'μ', 'ι', 'π', s @ ..] => super::t0::n802(s, out),
								_ => false,
							}
						}
						['ν', s @ ..] => super::t0::n249(s, out),
						['ρ', s @ ..] => super::t0::n250(s, out),
						['τ', s @ ..] => super::t0::n251(s, out),
						['φ', s @ ..] => super::t0::n252(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n55(s, out),
				['υ', s @ ..] => super::t0::n56(s, out),
				['ω', s @ ..] => super::t0::n57(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n18(s, out),
		['σ', s @ ..] => super::t0::n19(s, out),
		['τ', s @ ..] => super::t0::n20(s, out),
		_ => false,
	}
}

/// Words ending -ω.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 67);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 257);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 259);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 260);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 261);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 262);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 264);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 265);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 266);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 267);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 800);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 268);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 269);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 72);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 73);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 271);
							match s {
								[] => true,
								['ι', 'ε', 'μ', 'α', s @ ..] => {
									super::push(out, 803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 272);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 273);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 274);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 275);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 276);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 277);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 804);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 1528);
											s.is_empty()
										}
										['χ', s @ ..] => {
											super::push(out, 1529);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 805);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t0::n74(s, out),
				['λ', s @ ..] => super::t0::n75(s, out),
				['μ', s @ ..] => super::t0::n76(s, out),
				['ν', s @ ..] => super::t0::n77(s, out),
				['ο', s @ ..] => super::t0::n78(s, out),
				['π', s @ ..] => super::t0::n79(s, out),
				['ρ', s @ ..] => super::t0::n80(s, out),
				['σ', s @ ..] => super::t0::n81(s, out),
				['τ', s @ ..] => super::t0::n82(s, out),
				['υ', s @ ..] => {
					super::push(out, 83);
					s.is_empty()
				}
				['φ', s @ ..] => super::t0::n84(s, out),
				['χ', s @ ..] => super::t0::n85(s, out),
				['ψ', s @ ..] => super::t0::n86(s, out),
				['ω', s @ ..] => {
					super::push(out, 87);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => super::t0::n27(s, out),
		['γ', s @ ..] => super::t0::n28(s, out),
		['δ', s @ ..] => super::t0::n29(s, out),
		['ε', s @ ..] => super::t0::n30(s, out),
		['ζ', s @ ..] => super::t0::n31(s, out),
		['η', s @ ..] => super::t0::n32(s, out),
		['θ', s @ ..] => super::t0::n33(s, out),
		['ι', s @ ..] => super::t0::n34(s, out),
		['κ', s @ ..] => super::t0::n35(s, out),
		['λ', s @ ..] => super::t0::n36(s, out),
		['μ', s @ ..] => super::t0::n37(s, out),
		['ν', s @ ..] => super::t0::n38(s, out),
		['ξ', s @ ..] => super::t0::n39(s, out),
		['ο', s @ ..] => super::t0::n40(s, out),
		['π', s @ ..] => super::t0::n41(s, out),
		['ρ', s @ ..] => super::t0::n42(s, out),
		['σ', s @ ..] => super::t0::n43(s, out),
		['τ', s @ ..] => super::t0::n44(s, out),
		['υ', s @ ..] => super::t0::n45(s, out),
		['φ', s @ ..] => super::t0::n46(s, out),
		['χ', s @ ..] => super::t0::n47(s, out),
		['ψ', s @ ..] => super::t0::n48(s, out),
		_ => false,
	}
}

/// Words ending -αι.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['μ', s @ ..] => super::t0::n50(s, out),
		['ν', s @ ..] => {
			super::push(out, 51);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 52);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 53);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ξυντιθημι.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2386);
	s.is_empty()
}

/// Words ending -ιημι.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 777);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πιμπλημι.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1496);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νημι.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	s.is_empty()
}

/// Words ending -ρημι.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['π', 'μ', 'ι', 'π', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τημι.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	match s {
		[] => true,
		['σ', 'ι', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['κ', 'ι', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2098);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						['υ', s @ ..] => {
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

/// Words ending -φημι.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	s.is_empty()
}

/// Words ending -ιμι.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['ε', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['α', 'π', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2102);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμι.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['λ', 'ο', 'π', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1515);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2104);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1516);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2106);
									match s {
										[] => true,
										['μ', s @ ..] => {
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2516);
													s.is_empty()
												}
												['μ', s @ ..] => {
													super::push(out, 2517);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 2518);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2107);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2390);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2108);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2391);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['ι', 'ε', 'δ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2112);
									match s {
										[] => true,
										['π', s @ ..] => {
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2519);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2520);
													s.is_empty()
												}
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
				['μ', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['ο', 'ν', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2113);
									match s {
										[] => true,
										['ε', 'κ', 'σ', 'α', s @ ..] => {
											super::push(out, 2392);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1522);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωμι.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['δ', 'ι', 'δ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 796);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1525);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2393);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1526);
							match s {
								[] => true,
								['π', 'α', s @ ..] => {
									super::push(out, 2116);
									s.is_empty()
								}
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

/// Words ending -οι.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	s.is_empty()
}

/// Words ending -σι.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	s.is_empty()
}

/// Words ending -τι.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 58);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 59);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -καω.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 280);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λαω.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['γ', 'α', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μαω.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ναω.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 299);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1534);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οαω.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['β', 'α', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παω.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1538);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ραω.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σαω.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['φ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταω.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 329);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 838);
					match s {
						[] => true,
						['φ', 'ι', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1545);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2395);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2396);
											s.is_empty()
										}
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
		['π', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φαω.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['α', s @ ..] => {
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

/// Words ending -χαω.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 339);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 340);
			s.is_empty()
		}
		['σ', '\u{1}', s @ ..] => {
			super::push(out, 341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψαω.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -βω.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 88);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 842);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 344);
					match s {
						[] => true,
						['θ', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1548);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1550);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1551);
									s.is_empty()
								}
								['ο', s @ ..] => {
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
		['λ', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 91);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 92);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γω.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 845);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1553);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 355);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1555);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1556);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1557);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1558);
									match s {
										[] => true,
										['α', 'κ', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2397);
													s.is_empty()
												}
												['γ', s @ ..] => {
													super::push(out, 2398);
													s.is_empty()
												}
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
							super::push(out, 852);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1559);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 1560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 359);
					match s {
						[] => true,
						['π', 'ε', 'τ', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 361);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 362);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['ι', 'μ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['φ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1561);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2117);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δω.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['α', 'ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['ε', 'θ', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εω.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 108);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 865);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 375);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 870);
							match s {
								[] => true,
								['ο', 'χ', 'ι', s @ ..] => {
									super::push(out, 1565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 377);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1566);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2119);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2120);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1567);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2121);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 2399);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2400);
													s.is_empty()
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 2122);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2123);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2401);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 2124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 872);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 874);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2125);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 2126);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2127);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2128);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2129);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2402);
													s.is_empty()
												}
												['η', s @ ..] => {
													super::push(out, 2403);
													s.is_empty()
												}
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
				['υ', s @ ..] => {
					super::push(out, 380);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 875);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 876);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 878);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['μ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 882);
							match s {
								[] => true,
								['α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['θ', s @ ..] => super::t0::n113(s, out),
		['ι', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['ο', 'π', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 895);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 896);
							match s {
								[] => true,
								['θ', s @ ..] => super::t0::n1647(s, out),
								['λ', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
								['τ', s @ ..] => super::t0::n1649(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n115(s, out),
		['λ', s @ ..] => super::t0::n116(s, out),
		['μ', s @ ..] => super::t0::n117(s, out),
		['ν', s @ ..] => super::t0::n118(s, out),
		['ξ', s @ ..] => super::t0::n119(s, out),
		['ο', s @ ..] => super::t0::n120(s, out),
		['π', s @ ..] => super::t0::n121(s, out),
		['ρ', s @ ..] => super::t0::n122(s, out),
		['σ', s @ ..] => super::t0::n123(s, out),
		['τ', s @ ..] => super::t0::n124(s, out),
		['υ', s @ ..] => super::t0::n125(s, out),
		['φ', s @ ..] => super::t0::n126(s, out),
		['χ', s @ ..] => super::t0::n127(s, out),
		['ψ', s @ ..] => super::t0::n128(s, out),
		['ω', s @ ..] => super::t0::n129(s, out),
		['ῥ', s @ ..] => super::t0::n130(s, out),
		_ => false,
	}
}

/// Words ending -ζω.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 999);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1000);
							match s {
								[] => true,
								['β', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2174);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2430);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 2431);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2175);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 2176);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 2177);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1001);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1003);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1010);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1659);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 1660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1012);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1661);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1662);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1013);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1663);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 1664);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1015);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1016);
							match s {
								[] => true,
								['α', 'ν', 'α', 'τ', s @ ..] => {
									super::push(out, 1667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1017);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1668);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 1669);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2179);
											match s {
												[] => true,
												['ι', s @ ..] => super::t0::n2551(s, out),
												['τ', s @ ..] => super::t0::n2552(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ε', s @ ..] => super::t0::n1749(s, out),
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t0::n1058(s, out),
						['σ', s @ ..] => super::t0::n1059(s, out),
						_ => false,
					}
				}
				['λ', s @ ..] => super::t0::n491(s, out),
				['μ', s @ ..] => super::t0::n492(s, out),
				['ν', s @ ..] => super::t0::n493(s, out),
				['ξ', s @ ..] => super::t0::n494(s, out),
				['ο', s @ ..] => super::t0::n495(s, out),
				['π', s @ ..] => super::t0::n496(s, out),
				['ρ', s @ ..] => super::t0::n497(s, out),
				['τ', s @ ..] => super::t0::n498(s, out),
				['υ', s @ ..] => super::t0::n499(s, out),
				['φ', s @ ..] => super::t0::n500(s, out),
				['χ', s @ ..] => super::t0::n501(s, out),
				['ω', s @ ..] => super::t0::n502(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n132(s, out),
		['η', s @ ..] => super::t0::n133(s, out),
		['ι', s @ ..] => super::t0::n134(s, out),
		['ο', s @ ..] => super::t0::n135(s, out),
		['υ', s @ ..] => super::t0::n136(s, out),
		['ω', s @ ..] => super::t0::n137(s, out),
		_ => false,
	}
}

/// Words ending -ηω.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	s.is_empty()
}

/// Words ending -θω.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['ε', 'π', 'α', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['ε', 'π', 'α', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['λ', 'κ', 'ι', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιω.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1123);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1751);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['α', 'γ', 'υ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['τ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['χ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1127);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1129);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1753);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κω.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['λ', 'π', 'ι', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 1131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1132);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1137);
							match s {
								[] => true,
								['ι', 'δ', 'α', s @ ..] => {
									super::push(out, 1755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1139);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1756);
									match s {
										[] => true,
										['ι', 'δ', 'ο', s @ ..] => {
											super::push(out, 2210);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 1757);
									match s {
										[] => true,
										['γ', 'α', s @ ..] => {
											super::push(out, 2211);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['ρ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1759);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 558);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						['ν', s @ ..] => {
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1761);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2212);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2213);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 1762);
									match s {
										[] => true,
										['ι', 'μ', 'α', s @ ..] => {
											super::push(out, 2214);
											s.is_empty()
										}
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
					super::push(out, 559);
					match s {
						[] => true,
						['α', 'ν', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1146);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1148);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1763);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1149);
							match s {
								[] => true,
								['γ', 'ι', 'γ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2215);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2216);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2450);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 2451);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 2452);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n2650(s, out),
														['ε', s @ ..] => super::t0::n2651(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2217);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1765);
									match s {
										[] => true,
										['γ', '\u{1}', s @ ..] => {
											super::push(out, 2218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1766);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 1767);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['ι', 'δ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λω.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1153);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1769);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2220);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2221);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2222);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2223);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2453);
													s.is_empty()
												}
												['ε', s @ ..] => {
													super::push(out, 2454);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1770);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2224);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2225);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 2226);
											s.is_empty()
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 1771);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 1772);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2227);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2228);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1773);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2229);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2230);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1774);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1154);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1155);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1777);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['γ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2231);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2232);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2233);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2455);
													s.is_empty()
												}
												['ε', s @ ..] => {
													super::push(out, 2456);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1163);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1164);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1165);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1780);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1781);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μω.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	s.is_empty()
}

/// Words ending -νω.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['μ', 'α', 'λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1782);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2236);
											match s {
												[] => true,
												['α', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n2652(s, out),
														['π', s @ ..] => super::t0::n2653(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2237);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2238);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2457);
													s.is_empty()
												}
												['ε', s @ ..] => {
													super::push(out, 2458);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1783);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1785);
									match s {
										[] => true,
										['π', s @ ..] => {
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2459);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2460);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2239);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['ν', 'α', 'χ', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1175);
							match s {
								[] => true,
								['α', 'μ', 'α', 'φ', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['α', 'λ', 'β', 'α', s @ ..] => {
									super::push(out, 1789);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1791);
									match s {
										[] => true,
										['λ', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2461);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 2462);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 1792);
									match s {
										[] => true,
										['τ', s @ ..] => {
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2463);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 2464);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 2465);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2654(s, out),
														['υ', s @ ..] => super::t0::n2655(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 2466);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 2467);
													s.is_empty()
												}
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
		['ε', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1177);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						['μ', s @ ..] => super::t0::n1224(s, out),
						['ο', s @ ..] => super::t0::n1225(s, out),
						_ => false,
					}
				}
				['τ', s @ ..] => super::t0::n593(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n169(s, out),
		['κ', s @ ..] => super::t0::n170(s, out),
		['μ', s @ ..] => super::t0::n171(s, out),
		['υ', s @ ..] => super::t0::n172(s, out),
		_ => false,
	}
}

/// Words ending -ξω.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['ε', 'λ', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οω.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['ρ', 'κ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1853);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['ι', 'ν', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['θ', 'ο', s @ ..] => {
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 179);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['κ', 'σ', 'α', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['α', 'κ', 'α', s @ ..] => {
							super::push(out, 1235);
							match s {
								[] => true,
								['ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1241);
							match s {
								[] => true,
								['υ', 'ε', 'λ', 'ε', 'π', s @ ..] => {
									super::push(out, 1856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1242);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['λ', 'π', 'μ', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['ρ', 'ο', 'μ', 'α', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πω.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['ρ', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['β', 'α', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1857);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1859);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1860);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1861);
									match s {
										[] => true,
										['ε', 'ρ', s @ ..] => {
											super::push(out, 2263);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['ε', 'λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1252);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1863);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1864);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['θ', 'ι', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1866);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2264);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2265);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1867);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2266);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2267);
											s.is_empty()
										}
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
		['ρ', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρω.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['φ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1255);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1868);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1869);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1871);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1872);
									s.is_empty()
								}
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
			super::push(out, 200);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1258);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1259);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1873);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1262);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1875);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1264);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1265);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1878);
									s.is_empty()
								}
								['ε', s @ ..] => {
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2268);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2269);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 1266);
							match s {
								[] => true,
								['φ', 'α', s @ ..] => {
									super::push(out, 1879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1268);
							match s {
								[] => true,
								['σ', 'α', s @ ..] => {
									super::push(out, 1880);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2270);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σω.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['σ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1273);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1881);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1882);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2468);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 2469);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 2470);
													s.is_empty()
												}
												['ρ', s @ ..] => {
													super::push(out, 2471);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 2472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1883);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1274);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1885);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1886);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1275);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1888);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1889);
									match s {
										[] => true,
										['τ', 'α', s @ ..] => {
											super::push(out, 2272);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1276);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1890);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1277);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1892);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2273);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2274);
											s.is_empty()
										}
										['χ', s @ ..] => {
											super::push(out, 2275);
											s.is_empty()
										}
										_ => false,
									}
								}
								['β', s @ ..] => {
									super::push(out, 1893);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 1894);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1895);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2276);
											s.is_empty()
										}
										['μ', s @ ..] => {
											super::push(out, 2277);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 1896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1278);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1897);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2278);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2279);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2280);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2281);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1898);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1899);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1900);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1279);
							match s {
								[] => true,
								['ι', 'α', s @ ..] => {
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2282);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2283);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => super::t0::n1333(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n670(s, out),
				['η', s @ ..] => super::t0::n671(s, out),
				['ι', s @ ..] => super::t0::n672(s, out),
				['υ', s @ ..] => super::t0::n673(s, out),
				['ω', s @ ..] => super::t0::n674(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τω.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1299);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['σ', 'α', s @ ..] => {
									super::push(out, 1918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1304);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1919);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1920);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1921);
									match s {
										[] => true,
										['σ', 'ι', s @ ..] => {
											super::push(out, 2289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1306);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1922);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1308);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1926);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2290);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2291);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1928);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 1929);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2292);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2293);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1309);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['κ', 'α', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['κ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 1936);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1937);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1938);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2294);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2295);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['α', 'κ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2296);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2297);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t0::n1371(s, out),
						['τ', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t0::n208(s, out),
		_ => false,
	}
}

/// Words ending -υω.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['θ', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1942);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1329);
							match s {
								[] => true,
								['κ', 'α', s @ ..] => {
									super::push(out, 1944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1330);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1945);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1331);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1332);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1947);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1333);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1949);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 1950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1335);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1337);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1339);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1951);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1952);
									match s {
										[] => true,
										['σ', 'α', 'π', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 2303);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1342);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1954);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1955);
									match s {
										[] => true,
										['β', s @ ..] => super::t0::n2416(s, out),
										['δ', s @ ..] => super::t0::n2417(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => super::t0::n1402(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t0::n705(s, out),
				['ν', s @ ..] => super::t0::n706(s, out),
				['ξ', s @ ..] => super::t0::n707(s, out),
				['π', s @ ..] => super::t0::n708(s, out),
				['ρ', s @ ..] => super::t0::n709(s, out),
				['σ', s @ ..] => super::t0::n710(s, out),
				['τ', s @ ..] => super::t0::n711(s, out),
				['φ', s @ ..] => super::t0::n712(s, out),
				['χ', s @ ..] => super::t0::n713(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n213(s, out),
		['κ', s @ ..] => super::t0::n214(s, out),
		['λ', s @ ..] => super::t0::n215(s, out),
		['μ', s @ ..] => super::t0::n216(s, out),
		['ν', s @ ..] => super::t0::n217(s, out),
		['ξ', s @ ..] => super::t0::n218(s, out),
		['ο', s @ ..] => super::t0::n219(s, out),
		['ρ', s @ ..] => super::t0::n220(s, out),
		['σ', s @ ..] => super::t0::n221(s, out),
		['τ', s @ ..] => super::t0::n222(s, out),
		['φ', s @ ..] => super::t0::n223(s, out),
		['χ', s @ ..] => super::t0::n224(s, out),
		_ => false,
	}
}

/// Words ending -φω.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1991);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2321);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2322);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2323);
											s.is_empty()
										}
										_ => false,
									}
								}
								['γ', s @ ..] => {
									super::push(out, 1992);
									match s {
										[] => true,
										['υ', 'σ', s @ ..] => {
											super::push(out, 2324);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2483);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1993);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2325);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1994);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2327);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2328);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1995);
									s.is_empty()
								}
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
			super::push(out, 225);
			match s {
				[] => true,
				['ρ', 'τ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1398);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1996);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2329);
											match s {
												[] => true,
												['α', 'ν', s @ ..] => {
													super::push(out, 2484);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2330);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χω.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['ν', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1404);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1998);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2331);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 1999);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2333);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2334);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['ε', 'τ', 'σ', 'υ', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1409);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2000);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['π', 'ι', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['β', '\u{1}', s @ ..] => {
							super::push(out, 1415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψω.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -μαι.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1418);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 241);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['ε', 'κ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1421);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2002);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1422);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1423);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1424);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2004);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n244(s, out),
		['υ', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['ν', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2384);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2385);
											s.is_empty()
										}
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

/// Words ending -θεω.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 890);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['ο', 'β', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 394);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['λ', 'ο', 'κ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θοποιεω.
pub(crate) fn n1647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1577);
	match s {
		[] => true,
		['α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τοποιεω.
pub(crate) fn n1649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1579);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κεω.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['φ', 'ο', 'ι', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1582);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1585);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1589);
							match s {
								[] => true,
								['α', 'κ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2404);
											s.is_empty()
										}
										['γ', s @ ..] => {
											super::push(out, 2405);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2406);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2407);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 2408);
											s.is_empty()
										}
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
		['ο', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 902);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λεω.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1591);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2137);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2138);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2409);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 2410);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1592);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2139);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 908);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1599);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1601);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2411);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2412);
											s.is_empty()
										}
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
					super::push(out, 909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 406);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 409);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['σ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2143);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 918);
					match s {
						[] => true,
						['ε', 'ι', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				['ο', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεω.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['λ', 'ο', 'π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1611);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 420);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['ο', 'κ', 'ι', 'ο', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2145);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2146);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2413);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2414);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['ο', s @ ..] => {
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
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['ο', 'κ', 'α', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['θ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -νεω.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 428);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['η', 'δ', 'α', 'ν', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['φ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2148);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2149);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['κ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2152);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2415);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2416);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2153);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1625);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξεω.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	s.is_empty()
}

/// Words ending -οεω.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['ν', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2155);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πεω.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['κ', 'σ', 'α', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρεω.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1627);
							s.is_empty()
						}
						['ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1630);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2157);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1634);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2159);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['χ', 'ι', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['η', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2161);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2417);
											s.is_empty()
										}
										['β', s @ ..] => {
											super::push(out, 2418);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 2419);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['φ', 'ο', 'κ', 'ι', 'ο', 'τ', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2164);
									match s {
										[] => true,
										['η', 'δ', s @ ..] => {
											super::push(out, 2420);
											s.is_empty()
										}
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
		['ρ', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['ν', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2421);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2422);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1646);
							match s {
								[] => true,
								['π', 'α', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σεω.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 449);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεω.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['κ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2166);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['σ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2168);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2169);
									s.is_empty()
								}
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
			super::push(out, 453);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['π', 'υ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2423);
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 2424);
											s.is_empty()
										}
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
		['η', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['α', 'π', 'α', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 458);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υεω.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	s.is_empty()
}

/// Words ending -φεω.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1653);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2170);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2171);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2425);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 2426);
											match s {
												[] => true,
												['ι', 'ψ', '\u{1}', s @ ..] => {
													super::push(out, 2521);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2427);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2428);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2429);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 2172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χεω.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1655);
							match s {
								[] => true,
								['υ', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 2173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 468);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['α', 'ι', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψεω.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['ε', 'δ', 'ο', s @ ..] => {
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωεω.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	s.is_empty()
}

/// Words ending -ῥεω.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	s.is_empty()
}

/// Words ending -ιαδικαζω.
pub(crate) fn n2551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2432);
	s.is_empty()
}

/// Words ending -ταδικαζω.
pub(crate) fn n2552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	s.is_empty()
}

/// Words ending -εικαζω.
pub(crate) fn n1749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1670);
	s.is_empty()
}

/// Words ending -ρκαζω.
pub(crate) fn n1058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1018);
	s.is_empty()
}

/// Words ending -σκαζω.
pub(crate) fn n1059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	s.is_empty()
}

/// Words ending -λαζω.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 480);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μαζω.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1671);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ναζω.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξαζω.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	s.is_empty()
}

/// Words ending -οαζω.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	s.is_empty()
}

/// Words ending -παζω.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1032);
			s.is_empty()
		}
		['ρ', 'α', s @ ..] => {
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ραζω.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1033);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταζω.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1042);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υαζω.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2434);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2522);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2523);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2524);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2525);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 2435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φαζω.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	s.is_empty()
}

/// Words ending -χαζω.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωαζω.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	s.is_empty()
}

/// Words ending -εζω.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηζω.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	s.is_empty()
}

/// Words ending -ιζω.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1050);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1693);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2184);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2436);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2437);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1697);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2186);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1700);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2189);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2190);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2438);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2439);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['τ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1704);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1707);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2194);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1710);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['γ', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1713);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2196);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2440);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2441);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n515(s, out),
		['ο', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['ε', 'τ', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t0::n518(s, out),
		['σ', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['τ', s @ ..] => super::t0::n520(s, out),
		['φ', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['χ', s @ ..] => super::t0::n522(s, out),
		['ω', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οζω.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 513);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υζω.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['γ', 'γ', 'ο', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωζω.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 521);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αταγιγνωσκω.
pub(crate) fn n2650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2528);
	s.is_empty()
}

/// Words ending -εταγιγνωσκω.
pub(crate) fn n2651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2529);
	s.is_empty()
}

/// The word αναλαμβανω.
pub(crate) fn n2652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2530);
	s.is_empty()
}

/// Words ending -παναλαμβανω.
pub(crate) fn n2653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2531);
	s.is_empty()
}

/// Words ending -εντυγχανω.
pub(crate) fn n2654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2532);
	s.is_empty()
}

/// Words ending -υντυγχανω.
pub(crate) fn n2655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2533);
	s.is_empty()
}

/// Words ending -μμενω.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1796);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ομενω.
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	s.is_empty()
}

/// Words ending -τενω.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	s.is_empty()
}

/// Words ending -ινω.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2240);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2241);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2534);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 2535);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2242);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2243);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1799);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2244);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 2245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1801);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2246);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2247);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1182);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['λ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2249);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['υ', 'ε', 'λ', 'ι', s @ ..] => {
							super::push(out, 1811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2251);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1833);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2253);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1834);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1835);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2256);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1197);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t0::n598(s, out),
		['τ', s @ ..] => {
			super::push(out, 584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κνω.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	s.is_empty()
}

/// Words ending -μνω.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['τ', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνω.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2261);
									s.is_empty()
								}
								['τ', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 592);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 594);
			match s {
				[] => true,
				['ο', 'ρ', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 598);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1851);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥασσω.
pub(crate) fn n1333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	s.is_empty()
}

/// Words ending -εσσω.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1281);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ησσω.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['π', 'α', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['π', 'α', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ισσω.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1909);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1912);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υσσω.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1288);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1289);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1290);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1291);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1915);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2287);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['α', 'λ', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσσω.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1295);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρυπτω.
pub(crate) fn n1371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1314);
	match s {
		[] => true,
		['κ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2301);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['π', 'υ', s @ ..] => {
							super::push(out, 2480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ττω.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['λ', 'α', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 1939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['ρ', 'υ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['ρ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['ρ', 'η', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['μ', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -βουλευω.
pub(crate) fn n2416(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2481);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2482);
			match s {
				[] => true,
				['υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 2536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δουλευω.
pub(crate) fn n2417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2304);
	s.is_empty()
}

/// Words ending -ωλευω.
pub(crate) fn n1402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1344);
	s.is_empty()
}

/// Words ending -μευω.
pub(crate) fn n705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1345);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νευω.
pub(crate) fn n706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2305);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1348);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1350);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['μ', 'η', 'ν', 'μ', 'α', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['δ', 'ν', 'ι', 'κ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2307);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξευω.
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1355);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πευω.
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1357);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1358);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρευω.
pub(crate) fn n709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1360);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1963);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2311);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σευω.
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 692);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τευω.
pub(crate) fn n711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['τ', 'σ', 'υ', s @ ..] => {
							super::push(out, 2313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1370);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1371);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1372);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1374);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				['ι', 'π', s @ ..] => {
					super::push(out, 1975);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1375);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φευω.
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	s.is_empty()
}

/// Words ending -χευω.
pub(crate) fn n713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1377);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θυω.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κυω.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λυω.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['δ', 'ι', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2315);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 2316);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2317);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1382);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['α', 'κ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2319);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2320);
									s.is_empty()
								}
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
			super::push(out, 700);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['π', 'α', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['κ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μυω.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	s.is_empty()
}

/// Words ending -νυω.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['ι', 'ε', 'δ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξυω.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	s.is_empty()
}

/// Words ending -ουω.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 712);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['κ', 'ο', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυω.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['ι', 'θ', s @ ..] => {
					super::push(out, 1392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συω.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	s.is_empty()
}

/// Words ending -τυω.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φυω.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['μ', s @ ..] => {
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1393);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυω.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['ι', 'τ', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομαι.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2010);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2012);
							match s {
								[] => true,
								['ε', 'τ', 'ι', s @ ..] => {
									super::push(out, 2335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2014);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2015);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 753);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2017);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2019);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2020);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2021);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2024);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2026);
							match s {
								[] => true,
								['ι', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2485);
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 2486);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 2487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2032);
							match s {
								[] => true,
								['ρ', 'ε', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2488);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2489);
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 2490);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2491);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2033);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2338);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 2339);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => super::t0::n2128(s, out),
						['π', s @ ..] => super::t0::n2129(s, out),
						['υ', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						['χ', s @ ..] => super::t0::n2131(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n1516(s, out),
				['η', s @ ..] => super::t0::n1517(s, out),
				['ι', s @ ..] => super::t0::n1518(s, out),
				['υ', s @ ..] => super::t0::n1519(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n779(s, out),
		['ι', s @ ..] => super::t0::n780(s, out),
		['κ', s @ ..] => super::t0::n781(s, out),
		['λ', s @ ..] => super::t0::n782(s, out),
		['μ', s @ ..] => super::t0::n783(s, out),
		['ν', s @ ..] => super::t0::n784(s, out),
		['ο', s @ ..] => super::t0::n785(s, out),
		['π', s @ ..] => super::t0::n786(s, out),
		['ρ', s @ ..] => super::t0::n787(s, out),
		['σ', s @ ..] => super::t0::n788(s, out),
		['τ', s @ ..] => super::t0::n789(s, out),
		['υ', s @ ..] => super::t0::n790(s, out),
		['φ', s @ ..] => super::t0::n791(s, out),
		['χ', s @ ..] => super::t0::n792(s, out),
		['ω', s @ ..] => super::t0::n793(s, out),
		_ => false,
	}
}

/// Words ending -νιζω.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['α', 'ξ', s @ ..] => {
							super::push(out, 2198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1722);
					match s {
						[] => true,
						['χ', 'γ', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1725);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριζω.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['τ', 'α', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2204);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2442);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2205);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2444);
									s.is_empty()
								}
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
			super::push(out, 1095);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1097);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1099);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['ν', 'γ', 'α', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιζω.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2206);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2445);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2526);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2527);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2447);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2448);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 2449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1102);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1103);
			s.is_empty()
		}
		['ι', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1104);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2208);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1107);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1108);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1109);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1111);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χιζω.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 511);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1113);
			match s {
				[] => true,
				['ε', 'τ', 'ν', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρινω.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	match s {
		[] => true,
		['κ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1837);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λαζομαι.
pub(crate) fn n2128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2034);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παζομαι.
pub(crate) fn n2129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2035);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2342);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χαζομαι.
pub(crate) fn n2131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2037);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2344);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εζομαι.
pub(crate) fn n1516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1452);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 2038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηζομαι.
pub(crate) fn n1517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1453);
	s.is_empty()
}

/// Words ending -ιζομαι.
pub(crate) fn n1518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1454);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2039);
			match s {
				[] => true,
				['ο', 'λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2040);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 2041);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2042);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['γ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2537);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2538);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2044);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υζομαι.
pub(crate) fn n1519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1455);
	s.is_empty()
}

/// Words ending -θομαι.
pub(crate) fn n779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 758);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1456);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιομαι.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 759);
	s.is_empty()
}

/// Words ending -κομαι.
pub(crate) fn n781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 760);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['σ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2048);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λομαι.
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2050);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1460);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2353);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μομαι.
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 762);
	s.is_empty()
}

/// Words ending -νομαι.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 763);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['θ', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2355);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2496);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2497);
									match s {
										[] => true,
										['π', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2545);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 2546);
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 2547);
													s.is_empty()
												}
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
							super::push(out, 2356);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['ι', 'γ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2358);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2498);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2499);
									match s {
										[] => true,
										['α', 'π', 'μ', s @ ..] => {
											super::push(out, 2540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 2359);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2501);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2360);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2361);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2362);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2503);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2541);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2542);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2053);
					match s {
						[] => true,
						['κ', 'ο', 'π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2505);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οομαι.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 764);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['κ', 'ι', 'ο', 'τ', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2056);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2057);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1469);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2059);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πομαι.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 765);
	s.is_empty()
}

/// Words ending -ρομαι.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 766);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1472);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1473);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σομαι.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 767);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1475);
			s.is_empty()
		}
		['σ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2066);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2365);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2068);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2369);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2507);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τομαι.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 768);
	match s {
		[] => true,
		['π', 'ε', 'κ', 'σ', 'α', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υομαι.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 769);
	match s {
		[] => true,
		['ε', s @ ..] => {
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2075);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2076);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['ω', '\u{1}', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φομαι.
pub(crate) fn n791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 770);
	s.is_empty()
}

/// Words ending -χομαι.
pub(crate) fn n792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 771);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['μ', 'μ', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['δ', 'α', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1480);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				['ε', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2377);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2509);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2543);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2544);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2378);
							match s {
								[] => true,
								['ε', 'ν', s @ ..] => {
									super::push(out, 2511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2380);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2512);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2514);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1482);
			match s {
				[] => true,
				['ε', 'π', 'α', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωομαι.
pub(crate) fn n793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	s.is_empty()
}

