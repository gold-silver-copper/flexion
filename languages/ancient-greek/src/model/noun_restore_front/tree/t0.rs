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
					super::push(out, 47);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 442);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 443);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 444);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 445);
							match s {
								[] => true,
								['τ', s @ ..] => {
									match s {
										[] => true,
										['η', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 2731);
											s.is_empty()
										}
										['ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2732);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', 'ι', 'α', 'δ', 'α', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 446);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1604);
									match s {
										[] => true,
										['ν', s @ ..] => {
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3273);
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 3274);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 3275);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 2733);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 1605);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2734);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 447);
							match s {
								[] => true,
								['β', 'ρ', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 448);
							match s {
								[] => true,
								['τ', 'ι', 'σ', s @ ..] => {
									match s {
										[] => true,
										['μ', 'α', s @ ..] => {
											super::push(out, 2735);
											s.is_empty()
										}
										['τ', 'η', 'ρ', s @ ..] => {
											super::push(out, 2736);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 449);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1608);
									match s {
										[] => true,
										['κ', 'ο', s @ ..] => {
											super::push(out, 2737);
											s.is_empty()
										}
										_ => false,
									}
								}
								['β', s @ ..] => {
									super::push(out, 1609);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1611);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2738);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 450);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1612);
									match s {
										[] => true,
										['ν', 'ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => super::t0::n1656(s, out),
								['κ', s @ ..] => super::t0::n1657(s, out),
								['τ', s @ ..] => super::t0::n1658(s, out),
								['υ', s @ ..] => {
									super::push(out, 1616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => super::t0::n453(s, out),
						['υ', s @ ..] => super::t0::n454(s, out),
						['φ', s @ ..] => super::t0::n455(s, out),
						_ => false,
					}
				}
				['δ', s @ ..] => super::t0::n48(s, out),
				['ε', s @ ..] => super::t0::n49(s, out),
				['η', s @ ..] => super::t0::n50(s, out),
				['ι', s @ ..] => super::t0::n51(s, out),
				['λ', s @ ..] => super::t0::n52(s, out),
				['ο', s @ ..] => super::t0::n53(s, out),
				['ρ', s @ ..] => super::t0::n54(s, out),
				['υ', s @ ..] => super::t0::n55(s, out),
				['ω', s @ ..] => super::t0::n56(s, out),
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n3(s, out),
		['δ', s @ ..] => super::t0::n4(s, out),
		['ε', s @ ..] => super::t0::n5(s, out),
		['ζ', s @ ..] => super::t0::n6(s, out),
		['η', s @ ..] => super::t0::n7(s, out),
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
		['ψ', s @ ..] => super::t0::n22(s, out),
		['ω', s @ ..] => super::t0::n23(s, out),
		['ϝ', s @ ..] => super::t0::n24(s, out),
		['ῥ', s @ ..] => super::t0::n25(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 288);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2521);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 2522);
									match s {
										[] => true,
										['ι', 'δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3211);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2523);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2524);
									match s {
										[] => true,
										['ο', 'ν', 'ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3212);
											s.is_empty()
										}
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
					super::push(out, 293);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1329);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1331);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2525);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1332);
							match s {
								[] => true,
								['λ', 'ι', s @ ..] => {
									match s {
										[] => true,
										['α', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3213);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2527);
									match s {
										[] => true,
										['α', 'τ', 'ο', 'θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
											super::push(out, 3215);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'χ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1333);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['ρ', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							match s {
								[] => true,
								['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2530);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1337);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['ν', 'ν', s @ ..] => {
							super::push(out, 1338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 297);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1342);
							s.is_empty()
						}
						['υ', 'λ', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2533);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['α', 'ο', 'φ', 'ω', 'τ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1347);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1348);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 302);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1350);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 3217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1351);
							match s {
								[] => true,
								['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n303(s, out),
				['υ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['λ', s @ ..] => super::t0::n2645(s, out),
								_ => false,
							}
						}
						['ο', s @ ..] => super::t0::n1379(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => super::t0::n306(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n28(s, out),
		['ε', s @ ..] => super::t0::n29(s, out),
		['η', s @ ..] => super::t0::n30(s, out),
		['θ', s @ ..] => super::t0::n31(s, out),
		['ι', s @ ..] => super::t0::n32(s, out),
		['κ', s @ ..] => super::t0::n33(s, out),
		['λ', s @ ..] => super::t0::n34(s, out),
		['μ', s @ ..] => super::t0::n35(s, out),
		['ν', s @ ..] => super::t0::n36(s, out),
		['ξ', s @ ..] => super::t0::n37(s, out),
		['ο', s @ ..] => super::t0::n38(s, out),
		['π', s @ ..] => super::t0::n39(s, out),
		['ρ', s @ ..] => super::t0::n40(s, out),
		['σ', s @ ..] => super::t0::n41(s, out),
		['τ', s @ ..] => super::t0::n42(s, out),
		['υ', s @ ..] => super::t0::n43(s, out),
		['φ', s @ ..] => super::t0::n44(s, out),
		['χ', s @ ..] => super::t0::n45(s, out),
		['ψ', s @ ..] => super::t0::n46(s, out),
		_ => false,
	}
}

/// Words ending -ισαβ.
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1613);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2740);
			match s {
				[] => true,
				['ε', 'ι', 'δ', s @ ..] => {
					super::push(out, 3276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2741);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κσαβ.
pub(crate) fn n1657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1614);
	match s {
		[] => true,
		['α', 'ν', 'ο', s @ ..] => {
			super::push(out, 2742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τσαβ.
pub(crate) fn n1658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1615);
	match s {
		[] => true,
		['α', 'γ', 'μ', s @ ..] => {
			super::push(out, 2743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ταβ.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1617);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['α', 'χ', 'ι', 'τ', s @ ..] => {
					super::push(out, 2744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υαβ.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	s.is_empty()
}

/// Words ending -φαβ.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	s.is_empty()
}

/// Words ending -δβ.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -εβ.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	s.is_empty()
}

/// Words ending -ηβ.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['ρ', 'υ', 'τ', s @ ..] => {
			super::push(out, 454);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιβ.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['λ', 'ι', 'ο', 'ν', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 458);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['γ', 'ρ', 'α', 'φ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λβ.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['μ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				['σ', s @ ..] => {
					match s {
						[] => true,
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						['φ', 'η', 'μ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 466);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οβ.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['ν', 'θ', 'ε', 'μ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['ι', 'ν', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['α', 'σ', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 1632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρβ.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['χ', 'μ', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 473);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1636);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1637);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1638);
					match s {
						[] => true,
						['ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υβ.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['σ', s @ ..] => {
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1640);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωβ.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γ.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['γ', 'γ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['α', 'ρ', 'η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 485);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1645);
							match s {
								[] => true,
								['β', 'ρ', 'ι', 'ο', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 2748);
									s.is_empty()
								}
								['κ', 'τ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								['ξ', 'ι', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2750);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1647);
							match s {
								[] => true,
								['ν', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				['ρ', 'σ', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['α', 'σ', 'τ', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['ε', 'α', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2753);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							super::push(out, 1652);
							match s {
								[] => true,
								['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2755);
									s.is_empty()
								}
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
			super::push(out, 59);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['γ', 'α', 'ρ', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['β', 'ρ', 'η', 'ν', 'η', '\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						['ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['κ', 'υ', s @ ..] => {
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2756);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', 'η', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 499);
					match s {
						[] => true,
						['φ', 'α', 'λ', 'λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 63);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1659);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1661);
							match s {
								[] => true,
								['μ', 'α', 'τ', 'ε', s @ ..] => {
									super::push(out, 2758);
									match s {
										[] => true,
										['ι', 'δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3277);
											s.is_empty()
										}
										['υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3278);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1662);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1664);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2759);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
								['ι', 'ο', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['ν', 'α', 'σ', 'ι', 'α', s @ ..] => {
							super::push(out, 1667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['ρ', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['ρ', 'ο', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				['ψ', '\u{1}', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δ.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['τ', 'υ', 'λ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'ο', 'θ', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								['ο', 'π', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 515);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1672);
							match s {
								[] => true,
								['ι', 'τ', 'α', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 2768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['υ', 'π', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2769);
									s.is_empty()
								}
								['σ', 'τ', s @ ..] => {
									super::push(out, 2770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['α', 'σ', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['μ', 'α', 'τ', s @ ..] => {
							super::push(out, 1676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['τ', 'ε', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['ο', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
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
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1683);
							match s {
								[] => true,
								['ρ', 'α', 'μ', s @ ..] => {
									super::push(out, 2771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1684);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ζ', 'ω', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1686);
							match s {
								[] => true,
								['τ', 'ο', s @ ..] => {
									super::push(out, 2773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1687);
							match s {
								[] => true,
								['ο', 'ν', s @ ..] => {
									super::push(out, 2774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1688);
							match s {
								[] => true,
								['υ', 'τ', 'ρ', s @ ..] => {
									super::push(out, 2775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2776);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2777);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3279);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2778);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2779);
									match s {
										[] => true,
										['α', 'μ', 'β', 'ο', s @ ..] => {
											match s {
												[] => true,
												['γ', 'ρ', 'α', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 3438);
													s.is_empty()
												}
												['σ', '\u{1}', s @ ..] => {
													super::push(out, 3439);
													s.is_empty()
												}
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
				['κ', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['α', s @ ..] => super::t0::n1743(s, out),
						['ε', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						['τ', 'υ', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => super::t0::n542(s, out),
				['ν', s @ ..] => super::t0::n543(s, out),
				['ο', s @ ..] => super::t0::n544(s, out),
				['π', s @ ..] => super::t0::n545(s, out),
				['ρ', s @ ..] => super::t0::n546(s, out),
				['σ', s @ ..] => super::t0::n547(s, out),
				['φ', s @ ..] => super::t0::n548(s, out),
				['χ', s @ ..] => super::t0::n549(s, out),
				['ψ', s @ ..] => super::t0::n550(s, out),
				['ω', s @ ..] => super::t0::n551(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n71(s, out),
		['ο', s @ ..] => super::t0::n72(s, out),
		['ρ', s @ ..] => super::t0::n73(s, out),
		['υ', s @ ..] => super::t0::n74(s, out),
		['ω', s @ ..] => super::t0::n75(s, out),
		_ => false,
	}
}

/// Words ending -ε.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 558);
					match s {
						[] => true,
						['υ', 'θ', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'ρ', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							super::push(out, 1721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['ε', 'σ', 'μ', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['α', 'τ', 'ο', 'θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['ν', 'α', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1724);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2788);
									match s {
										[] => true,
										['ι', 'ν', 'η', s @ ..] => {
											super::push(out, 3282);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['α', 'τ', 'α', 'ν', 'ω', 'π', 'ε', 'ρ', 'ι', 'β', 'α', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						['υ', 'τ', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1735);
							s.is_empty()
						}
						['σ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['η', 'ρ', s @ ..] => {
									super::push(out, 2790);
									s.is_empty()
								}
								['ι', 'ν', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1738);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3283);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 3284);
											s.is_empty()
										}
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
					super::push(out, 581);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 583);
					match s {
						[] => true,
						['ι', 'ν', 'θ', 'ο', 'β', 'ο', 'τ', 'α', 'ν', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['δ', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['η', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2792);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1745);
							match s {
								[] => true,
								['ν', 'ι', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2794);
									s.is_empty()
								}
								['σ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1746);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2796);
									s.is_empty()
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['υ', 'μ', 'η', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'α', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1750);
							match s {
								[] => true,
								['β', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2797);
									s.is_empty()
								}
								['π', 'ω', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['κ', s @ ..] => super::t0::n1803(s, out),
						['λ', s @ ..] => super::t0::n1804(s, out),
						_ => false,
					}
				}
				['η', s @ ..] => super::t0::n601(s, out),
				['ι', s @ ..] => super::t0::n602(s, out),
				['ο', s @ ..] => super::t0::n603(s, out),
				['υ', s @ ..] => super::t0::n604(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n87(s, out),
		['π', s @ ..] => super::t0::n88(s, out),
		['ρ', s @ ..] => super::t0::n89(s, out),
		['σ', s @ ..] => super::t0::n90(s, out),
		['τ', s @ ..] => super::t0::n91(s, out),
		['υ', s @ ..] => super::t0::n92(s, out),
		['φ', s @ ..] => super::t0::n93(s, out),
		['χ', s @ ..] => super::t0::n94(s, out),
		_ => false,
	}
}

/// Words ending -ζ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['ι', 'σ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				['χ', 'α', 'ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['γ', 'ι', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['α', 'ν', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1794);
							match s {
								[] => true,
								['σ', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'σ', s @ ..] => {
							super::push(out, 1795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['γ', 'ρ', 'α', 'φ', 'ε', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -η.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						['σ', 'α', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 103);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1801);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2815);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 642);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['υ', 'κ', 'λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1804);
							match s {
								[] => true,
								['ι', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						['ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['θ', 'θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2819);
									s.is_empty()
								}
								['μ', 'ο', s @ ..] => {
									super::push(out, 2820);
									s.is_empty()
								}
								['σ', 'σ', s @ ..] => {
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3289);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3290);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'τ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['ω', 'ρ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['μ', 'α', 'τ', 'ο', 'υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1819);
							match s {
								[] => true,
								['π', 'ω', s @ ..] => {
									super::push(out, 2823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['σ', 'α', 'λ', 'ο', 'ν', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['ι', 'ο', s @ ..] => {
							super::push(out, 1824);
							match s {
								[] => true,
								['ν', '\u{1}', s @ ..] => {
									super::push(out, 2824);
									s.is_empty()
								}
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
			super::push(out, 115);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['ν', 'ι', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['α', 'λ', 'λ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1831);
							match s {
								[] => true,
								['ι', 'γ', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 681);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
								['λ', 'ω', s @ ..] => {
									super::push(out, 2828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1836);
							match s {
								[] => true,
								['α', 'τ', 'η', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1837);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['ι', 'α', 'σ', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 120);
			s.is_empty()
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
			super::push(out, 121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['β', s @ ..] => {
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['γ', 'ρ', 'α', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3291);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3292);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', 'κ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['ι', 'σ', 'κ', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['δ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 698);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2832);
									match s {
										[] => true,
										['υ', 'γ', 'κ', 'ρ', 'α', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3293);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2834);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 2836);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2837);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2838);
									match s {
										[] => true,
										['χ', s @ ..] => {
											match s {
												[] => true,
												['η', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 3440);
													s.is_empty()
												}
												['ι', 'α', '\u{1}', s @ ..] => {
													super::push(out, 3441);
													s.is_empty()
												}
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
							super::push(out, 1852);
							match s {
								[] => true,
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1853);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2840);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2842);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2843);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3294);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 2844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['μ', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						['φ', 'α', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['ν', 'ο', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						['ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2845);
									s.is_empty()
								}
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['ι', 'ω', 'τ', 'η', 'ρ', '\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['ι', 'σ', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n130(s, out),
		['ν', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['δ', s @ ..] => super::t0::n729(s, out),
				['ι', s @ ..] => super::t0::n730(s, out),
				_ => false,
			}
		}
		['ξ', s @ ..] => super::t0::n132(s, out),
		['ο', s @ ..] => super::t0::n133(s, out),
		['π', s @ ..] => super::t0::n134(s, out),
		['ρ', s @ ..] => super::t0::n135(s, out),
		['σ', s @ ..] => super::t0::n136(s, out),
		['τ', s @ ..] => super::t0::n137(s, out),
		['υ', s @ ..] => super::t0::n138(s, out),
		['φ', s @ ..] => super::t0::n139(s, out),
		['χ', s @ ..] => super::t0::n140(s, out),
		['ψ', s @ ..] => super::t0::n141(s, out),
		['ω', s @ ..] => super::t0::n142(s, out),
		_ => false,
	}
}

/// Words ending -κ.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1879);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1880);
							match s {
								[] => true,
								['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', 'χ', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2867);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['κ', s @ ..] => {
							match s {
								[] => true,
								['α', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2869);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 2870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1881);
							match s {
								[] => true,
								['θ', 'ι', 'σ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2871);
									s.is_empty()
								}
								['μ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3306);
											s.is_empty()
										}
										['ι', 'ν', 'θ', 'η', '\u{1}', s @ ..] => {
											super::push(out, 3307);
											s.is_empty()
										}
										['ο', s @ ..] => {
											match s {
												[] => true,
												['γ', 'ρ', 'α', 'φ', 'ι', 'α', '\u{1}', s @ ..] => {
													super::push(out, 3445);
													s.is_empty()
												}
												['θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
													super::push(out, 3446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', 'δ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2872);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2873);
									s.is_empty()
								}
								['σ', 'ι', 'ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1882);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2875);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 2876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2877);
									match s {
										[] => true,
										['α', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3308);
											s.is_empty()
										}
										['θ', 'ρ', 'ι', 'ξ', '\u{1}', s @ ..] => {
											super::push(out, 3309);
											s.is_empty()
										}
										['κ', 'λ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3310);
											s.is_empty()
										}
										['ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3311);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 3312);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2879);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2880);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1886);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2882);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => super::t0::n770(s, out),
				['π', s @ ..] => super::t0::n771(s, out),
				['ρ', s @ ..] => super::t0::n772(s, out),
				['σ', s @ ..] => super::t0::n773(s, out),
				['τ', s @ ..] => super::t0::n774(s, out),
				['υ', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['χ', 'λ', 'α', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				['ψ', s @ ..] => super::t0::n777(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n144(s, out),
		['η', s @ ..] => super::t0::n145(s, out),
		['ι', s @ ..] => super::t0::n146(s, out),
		['λ', s @ ..] => super::t0::n147(s, out),
		['ν', s @ ..] => super::t0::n148(s, out),
		['ο', s @ ..] => super::t0::n149(s, out),
		['ρ', s @ ..] => super::t0::n150(s, out),
		['τ', s @ ..] => super::t0::n151(s, out),
		['υ', s @ ..] => super::t0::n152(s, out),
		['ω', s @ ..] => super::t0::n153(s, out),
		_ => false,
	}
}

/// Words ending -λ.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 838);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1987);
							match s {
								[] => true,
								['α', 'κ', 'ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1988);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', 'α', 'ν', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['η', 'μ', s @ ..] => {
							super::push(out, 1995);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1997);
							match s {
								[] => true,
								['β', 'α', 'ρ', 'υ', 'π', 'α', 'ρ', 'α', 'μ', 'ε', 'λ', 'ο', 'ρ', 'υ', 'θ', 'μ', 'ο', 'β', 'α', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1998);
							match s {
								[] => true,
								['δ', 'α', 'κ', 'ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1999);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2000);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2947);
									match s {
										[] => true,
										['δ', s @ ..] => {
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3451);
													match s {
														[] => true,
														['ρ', 'ι', s @ ..] => super::t0::n3637(s, out),
														_ => false,
													}
												}
												['ε', 'ι', 'α', s @ ..] => {
													super::push(out, 3452);
													s.is_empty()
												}
												['η', s @ ..] => {
													super::push(out, 3453);
													match s {
														[] => true,
														['φ', 'ο', 'ρ', 'ι', s @ ..] => super::t0::n3638(s, out),
														_ => false,
													}
												}
												['ι', s @ ..] => {
													super::push(out, 3454);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3639(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 3455);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n3640(s, out),
														['υ', s @ ..] => super::t0::n3641(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 3339);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 2948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ψ', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 848);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 2002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['α', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						['θ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2949);
									s.is_empty()
								}
								['ο', 'θ', s @ ..] => {
									super::push(out, 2950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['α', 'δ', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2008);
							match s {
								[] => true,
								['δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 855);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 2009);
							match s {
								[] => true,
								['κ', 'α', 'ν', 'θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2952);
									s.is_empty()
								}
								['ν', 'θ', 'ε', 'μ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2953);
									s.is_empty()
								}
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
			super::push(out, 156);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'τ', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n157(s, out),
		['ο', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['γ', 'ι', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n159(s, out),
		['ω', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 161);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						['ν', 'η', 'σ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2956);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['α', 'ρ', 'ω', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', 'α', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['δ', 'ο', s @ ..] => {
									super::push(out, 2958);
									s.is_empty()
								}
								['λ', 'λ', 'α', s @ ..] => {
									super::push(out, 2959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						['ρ', 'ω', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2960);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 2961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2044);
							match s {
								[] => true,
								['α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2962);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 2963);
									s.is_empty()
								}
								['ι', 'λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2964);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2965);
									s.is_empty()
								}
								['υ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3340);
											s.is_empty()
										}
										['λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3341);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2045);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2966);
									match s {
										[] => true,
										['κ', 'ι', s @ ..] => {
											super::push(out, 3342);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 2967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2046);
							match s {
								[] => true,
								['ι', 'λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2047);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2969);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 2048);
							match s {
								[] => true,
								['α', 'ρ', 'ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2049);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2975);
									match s {
										[] => true,
										['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2050);
							match s {
								[] => true,
								['α', 'ρ', 'ο', 'υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2052);
							match s {
								[] => true,
								['ι', 'π', 'π', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3344);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3345);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2053);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['γ', 'ι', 'δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						['η', s @ ..] => super::t0::n2126(s, out),
						['ο', 'υ', 'χ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						['σ', s @ ..] => super::t0::n2128(s, out),
						['τ', s @ ..] => super::t0::n2129(s, out),
						['χ', 'α', 'λ', 'η', '\u{1}', s @ ..] => super::t0::n2130(s, out),
						_ => false,
					}
				}
				['τ', s @ ..] => super::t0::n898(s, out),
				['υ', s @ ..] => super::t0::n899(s, out),
				['φ', s @ ..] => super::t0::n900(s, out),
				['χ', s @ ..] => super::t0::n901(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n162(s, out),
		['η', s @ ..] => super::t0::n163(s, out),
		['ι', s @ ..] => super::t0::n164(s, out),
		['ν', s @ ..] => super::t0::n165(s, out),
		['ο', s @ ..] => super::t0::n166(s, out),
		['υ', s @ ..] => super::t0::n167(s, out),
		['ω', s @ ..] => super::t0::n168(s, out),
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['ζ', 'ε', 'ι', 'ρ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['μ', 'ε', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				['π', 'η', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2104);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						['ν', 'α', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2108);
							match s {
								[] => true,
								['γ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3015);
									s.is_empty()
								}
								['ρ', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['ν', 'ι', s @ ..] => {
							match s {
								[] => true,
								['α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3018);
									s.is_empty()
								}
								['σ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2111);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3020);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2112);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3022);
									s.is_empty()
								}
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
			super::push(out, 171);
			match s {
				[] => true,
				['σ', 'ι', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['ω', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['μ', 'α', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2116);
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
				['κ', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['τ', 'ι', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'φ', 'ι', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
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
			super::push(out, 176);
			match s {
				[] => true,
				['ν', 'θ', 'η', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 177);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['ρ', 'α', 'φ', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 179);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 180);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['γ', 'γ', 'ε', 'ν', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['ν', 'τ', 'υ', 'χ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
								['ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
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

/// Words ending -ο.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['ε', 'λ', 'ι', 'σ', 'κ', 'ο', 'λ', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 184);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2118);
							match s {
								[] => true,
								['σ', 'κ', s @ ..] => {
									super::push(out, 3025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'τ', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['ν', 'ι', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['τ', 'α', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['γ', 'α', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3026);
									s.is_empty()
								}
								['ρ', 'χ', 'ι', s @ ..] => {
									super::push(out, 3027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['ι', 'σ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['ν', 'θ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['γ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['σ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2130);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2131);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2133);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2134);
							match s {
								[] => true,
								['κ', 'α', 'ν', 'θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 2135);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2136);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2137);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2138);
							match s {
								[] => true,
								['υ', 'ρ', 'σ', 'ι', 'ν', 'η', '\u{1}', s @ ..] => {
									super::push(out, 3031);
									s.is_empty()
								}
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
			super::push(out, 192);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['λ', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 978);
					match s {
						[] => true,
						['σ', 'θ', s @ ..] => {
							super::push(out, 2140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['ι', 'λ', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 2146);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2147);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2149);
							match s {
								[] => true,
								['ν', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['ι', 'θ', s @ ..] => {
							super::push(out, 2150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['υ', 'γ', s @ ..] => {
							super::push(out, 2151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2152);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								['ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 2153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t0::n194(s, out),
		['υ', s @ ..] => super::t0::n195(s, out),
		['φ', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['ι', s @ ..] => super::t0::n1009(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => super::t0::n197(s, out),
		['ψ', s @ ..] => super::t0::n198(s, out),
		_ => false,
	}
}

/// Words ending -π.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						['κ', 'ρ', 'α', s @ ..] => {
							super::push(out, 2169);
							match s {
								[] => true,
								['τ', 'ι', 'α', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['δ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3040);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3352);
											s.is_empty()
										}
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
					super::push(out, 1007);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2170);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								['ι', 'σ', 'τ', 'ι', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2171);
							match s {
								[] => true,
								['γ', 'κ', 'α', 'π', 'η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2172);
							match s {
								[] => true,
								['α', 'κ', 'ι', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['β', 'α', 'σ', 'ι', 'λ', 'ε', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2175);
							match s {
								[] => true,
								['ο', 'χ', 'ε', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 2176);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2177);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3047);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2178);
							match s {
								[] => true,
								['ι', 'μ', 'α', 'θ', 'ο', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 3048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2180);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3049);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3050);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3353);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 3051);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3052);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3355);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', 'ε', s @ ..] => {
									super::push(out, 3053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2183);
							match s {
								[] => true,
								['ε', 'ν', s @ ..] => {
									match s {
										[] => true,
										['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3356);
											s.is_empty()
										}
										['ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						['τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['α', 'γ', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3054);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3055);
									match s {
										[] => true,
										['α', 'ρ', s @ ..] => {
											super::push(out, 3358);
											s.is_empty()
										}
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
					super::push(out, 1015);
					s.is_empty()
				}
				['χ', 'ν', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['λ', s @ ..] => super::t0::n1032(s, out),
				['μ', s @ ..] => super::t0::n1033(s, out),
				['ν', s @ ..] => super::t0::n1034(s, out),
				['π', s @ ..] => super::t0::n1035(s, out),
				['ρ', s @ ..] => super::t0::n1036(s, out),
				['τ', s @ ..] => super::t0::n1037(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n201(s, out),
		['ι', s @ ..] => super::t0::n202(s, out),
		['λ', s @ ..] => super::t0::n203(s, out),
		['ν', s @ ..] => super::t0::n204(s, out),
		['ο', s @ ..] => super::t0::n205(s, out),
		['ρ', s @ ..] => super::t0::n206(s, out),
		['τ', s @ ..] => super::t0::n207(s, out),
		['υ', s @ ..] => super::t0::n208(s, out),
		['ω', s @ ..] => super::t0::n209(s, out),
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['α', 'τ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3370);
											s.is_empty()
										}
										['ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3371);
											s.is_empty()
										}
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
					super::push(out, 1074);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							super::push(out, 2261);
							match s {
								[] => true,
								['τ', 'ι', 'ο', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 3088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2262);
							match s {
								[] => true,
								['α', 'τ', 'ο', 'π', 'α', 'σ', 'μ', 'α', 'γ', 'α', 'δ', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['χ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3090);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['α', 'μ', 'α', s @ ..] => {
							super::push(out, 2264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									match s {
										[] => true,
										['ε', 'ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3372);
											s.is_empty()
										}
										['ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2267);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3374);
											match s {
												[] => true,
												['ι', 'σ', s @ ..] => {
													super::push(out, 3458);
													s.is_empty()
												}
												['ο', 'θ', s @ ..] => {
													super::push(out, 3459);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						['ν', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						['τ', 'α', 'λ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2271);
							s.is_empty()
						}
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2273);
							match s {
								[] => true,
								['β', 'ο', s @ ..] => {
									super::push(out, 3093);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', 'α', s @ ..] => {
							super::push(out, 2274);
							match s {
								[] => true,
								['ν', 'η', '\u{1}', s @ ..] => {
									super::push(out, 3095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2276);
							match s {
								[] => true,
								['α', 'π', 'η', s @ ..] => {
									super::push(out, 3096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2277);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2278);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3376);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3377);
											s.is_empty()
										}
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
					super::push(out, 1084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['ι', 'ν', s @ ..] => {
							super::push(out, 2279);
							match s {
								[] => true,
								['ι', 'τ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['ι', 'δ', 'α', 'λ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2281);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['α', 'μ', 'ο', 'ε', s @ ..] => {
							super::push(out, 2283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['α', 'ν', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n213(s, out),
		['κ', s @ ..] => super::t1::n214(s, out),
		['μ', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['μ', 'α', 'τ', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ρ', 'ο', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['ι', 'σ', s @ ..] => super::t1::n2410(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n217(s, out),
		['τ', s @ ..] => super::t1::n218(s, out),
		['υ', s @ ..] => super::t1::n219(s, out),
		['φ', s @ ..] => super::t1::n220(s, out),
		['χ', s @ ..] => super::t1::n221(s, out),
		['ω', s @ ..] => super::t1::n222(s, out),
		_ => false,
	}
}

/// Words ending -τ.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['ι', 'ε', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['χ', 'ο', 'π', 'ω', 'λ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						['σ', 'ε', s @ ..] => {
							super::push(out, 2386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2387);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['υ', 'κ', 'α', 'ν', 'ι', 'σ', 'τ', 'η', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1162);
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
				['α', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1164);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2388);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['υ', 'μ', 'α', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2391);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2394);
							match s {
								[] => true,
								['ν', 'ο', s @ ..] => {
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3392);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 3393);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 3154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['υ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							super::push(out, 2399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2400);
							match s {
								[] => true,
								['α', 'κ', s @ ..] => {
									super::push(out, 3155);
									s.is_empty()
								}
								['ω', 'δ', 'ι', 'ο', s @ ..] => {
									super::push(out, 3156);
									match s {
										[] => true,
										['γ', 'ρ', 'α', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2401);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2402);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2403);
							match s {
								[] => true,
								['η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['ι', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3159);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 2405);
							match s {
								[] => true,
								['λ', 'η', s @ ..] => {
									super::push(out, 3161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'μ', s @ ..] => {
							super::push(out, 2406);
							s.is_empty()
						}
						['π', 'α', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 2408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2409);
							match s {
								[] => true,
								['γ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3162);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ι', 'π', 'ο', 'σ', '\u{1}', s @ ..] => super::t1::n3565(s, out),
										['ν', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => super::t1::n3566(s, out),
										_ => false,
									}
								}
								['ω', 'ν', '\u{1}', s @ ..] => super::t1::n3310(s, out),
								_ => false,
							}
						}
						['π', s @ ..] => super::t1::n2502(s, out),
						['τ', s @ ..] => super::t1::n2503(s, out),
						['φ', s @ ..] => super::t1::n2504(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => super::t1::n1189(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n232(s, out),
		['ω', s @ ..] => super::t1::n233(s, out),
		_ => false,
	}
}

/// Words ending -υ.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', 'ο', s @ ..] => {
			super::push(out, 235);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['ι', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2422);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3167);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2423);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3169);
									s.is_empty()
								}
								['μ', s @ ..] => {
									match s {
										[] => true,
										['α', 'ν', 'τ', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3397);
											s.is_empty()
										}
										['ε', 'λ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 3398);
											s.is_empty()
										}
										['υ', 'λ', 'η', '\u{1}', s @ ..] => {
											super::push(out, 3399);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', 'ε', 'π', 'ε', 'ρ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 3170);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3171);
									s.is_empty()
								}
								['φ', 'ο', 'β', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', 'ψ', '\u{1}', s @ ..] => {
							super::push(out, 2424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 239);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3173);
									s.is_empty()
								}
								['τ', 'η', 'ρ', '\u{1}', s @ ..] => {
									super::push(out, 3174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'σ', 'ε', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2425);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2426);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'ο', s @ ..] => {
							super::push(out, 2427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3176);
									s.is_empty()
								}
								['β', s @ ..] => {
									super::push(out, 3177);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3400);
											s.is_empty()
										}
										['ψ', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3401);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'α', 'ξ', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3178);
									s.is_empty()
								}
								['ω', s @ ..] => {
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3402);
											s.is_empty()
										}
										['ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3403);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2428);
							s.is_empty()
						}
						['ρ', 'ε', 'σ', s @ ..] => {
							super::push(out, 2429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2430);
							s.is_empty()
						}
						['γ', 'λ', s @ ..] => {
							super::push(out, 2431);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						['κ', 'ο', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						['λ', 'η', 'ν', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3179);
									s.is_empty()
								}
								['τ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3404);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3405);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['γ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['σ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['τ', 'ε', 'ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['σ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'α', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['ο', 'λ', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => super::t1::n247(s, out),
		_ => false,
	}
}

/// Words ending -φ.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3180);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3406);
											s.is_empty()
										}
										['ξ', '\u{1}', s @ ..] => {
											super::push(out, 3407);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['λ', 'α', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2448);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2449);
							match s {
								[] => true,
								['γ', 'γ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', 'ο', s @ ..] => {
							super::push(out, 2450);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'σ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2452);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2453);
							match s {
								[] => true,
								['α', 'κ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 3408);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 3409);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 3410);
											s.is_empty()
										}
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
					super::push(out, 1217);
					match s {
						[] => true,
						['ι', 'α', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2455);
							match s {
								[] => true,
								['ω', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'τ', 'α', 'γ', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['ν', 'α', 'κ', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['ν', 'α', s @ ..] => {
							super::push(out, 2457);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['η', '\u{1}', s @ ..] => {
									super::push(out, 3185);
									s.is_empty()
								}
								['ο', 'μ', 'α', 'ν', 'τ', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['π', 'π', 'η', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2460);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2461);
							match s {
								[] => true,
								['ρ', 'α', s @ ..] => {
									super::push(out, 3188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
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
		['λ', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['ν', 'ι', 'κ', s @ ..] => {
							super::push(out, 2462);
							match s {
								[] => true,
								['ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1236);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ρ', '\u{1}', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
								['τ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3411);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3412);
											s.is_empty()
										}
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
					super::push(out, 1237);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2465);
							match s {
								[] => true,
								['α', 'ν', 'ι', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 2468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n256(s, out),
		['ω', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
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
			super::push(out, 258);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2480);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								['ζ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3413);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3414);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', 'φ', s @ ..] => {
							super::push(out, 2481);
							s.is_empty()
						}
						['κ', 'ο', 'θ', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['α', 'ι', 'κ', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3199);
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
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2484);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3200);
									match s {
										[] => true,
										['ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3415);
											s.is_empty()
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2485);
							match s {
								[] => true,
								['σ', 'τ', s @ ..] => {
									match s {
										[] => true,
										['ε', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3416);
											s.is_empty()
										}
										['η', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 3417);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2487);
							match s {
								[] => true,
								['ο', 'υ', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['μ', 'α', s @ ..] => {
							super::push(out, 2489);
							match s {
								[] => true,
								['σ', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['ι', 'δ', 'ο', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						['υ', 'δ', s @ ..] => {
							super::push(out, 2491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['α', 'λ', 'ω', 'π', 'η', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['ι', 'ο', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1260);
					match s {
						[] => true,
						['α', 'ρ', 'α', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2495);
							match s {
								[] => true,
								['υ', s @ ..] => {
									match s {
										[] => true,
										['δ', 'ο', 'υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3418);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3419);
											s.is_empty()
										}
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
		['ν', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'α', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['σ', 'μ', 'ι', 'ο', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 2497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3205);
									s.is_empty()
								}
								['μ', s @ ..] => {
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3420);
											s.is_empty()
										}
										['ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3421);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3206);
									match s {
										[] => true,
										['ι', 'α', 'ν', s @ ..] => {
											match s {
												[] => true,
												['ι', 'σ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 3464);
													s.is_empty()
												}
												['ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 3465);
													s.is_empty()
												}
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
					super::push(out, 1269);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['σ', 'α', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1272);
					s.is_empty()
				}
				['μ', 'ο', s @ ..] => {
					super::push(out, 1273);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψ.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['ι', 'ο', s @ ..] => {
							super::push(out, 2499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'μ', 'ο', 'δ', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1279);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['υ', 'δ', 'α', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['θ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2500);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['υ', 'θ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
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
		['ο', s @ ..] => {
			super::push(out, 272);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1289);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1290);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ω.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['γ', 'γ', 'ε', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ϝ.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	s.is_empty()
}

/// Words ending -ῥ.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3208);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1301);
					match s {
						[] => true,
						['α', 'μ', 'ι', 'γ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'σ', 'τ', 'η', 'ρ', '\u{1}', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				['μ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['ν', 'τ', 'ι', 'σ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				['π', 'ι', 'σ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1309);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1310);
					match s {
						[] => true,
						['α', 'ν', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						['η', '\u{1}', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1311);
					match s {
						[] => true,
						['ι', 'σ', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', 'ω', 'δ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['υ', 'μ', 'α', 'τ', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['τ', 'ι', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['ν', 'ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1315);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1316);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['δ', 'ι', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['α', 'ν', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ι', 'ο', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1326);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ργα.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['ι', 'φ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2537);
					s.is_empty()
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2539);
					match s {
						[] => true,
						['κ', 'ρ', s @ ..] => {
							super::push(out, 3218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1357);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιχγα.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2542);
	s.is_empty()
}

/// Words ending -οχγα.
pub(crate) fn n1379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1360);
	match s {
		[] => true,
		['υ', 'σ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωγα.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['ν', 'ι', s @ ..] => {
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				['σ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δα.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1361);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1362);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1364);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['μ', 'ο', 'ν', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1367);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['υ', 'α', 'σ', s @ ..] => {
					super::push(out, 1370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εα.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηα.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θα.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['ν', 'α', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['η', 'τ', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				['ρ', 'μ', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ια.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 323);
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
				['ι', s @ ..] => {
					super::push(out, 1379);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['λ', 'ο', 'φ', 'υ', 'λ', 'α', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 3219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 2548);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1380);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3220);
									s.is_empty()
								}
								['ψ', s @ ..] => {
									super::push(out, 3221);
									s.is_empty()
								}
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
			super::push(out, 325);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['ι', 'ο', 'λ', s @ ..] => {
					super::push(out, 1383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['μ', 'α', 'λ', 'ω', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κα.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1386);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1387);
					match s {
						[] => true,
						['θ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3222);
									s.is_empty()
								}
								['ι', 'σ', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 329);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1391);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1394);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1397);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2551);
							match s {
								[] => true,
								['ε', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								['ο', 'φ', 'ο', 'ρ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['ν', '\u{1}', s @ ..] => {
											super::push(out, 3422);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3423);
											s.is_empty()
										}
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
					super::push(out, 1398);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2553);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1400);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2555);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3225);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		['ω', 'κ', s @ ..] => {
			super::push(out, 337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λα.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['α', 'σ', 'τ', 'ρ', s @ ..] => {
							super::push(out, 2557);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ν', '\u{1}', s @ ..] => {
											super::push(out, 3424);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3425);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['η', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1406);
					s.is_empty()
				}
				['ρ', 'ο', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 339);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['η', s @ ..] => {
					match s {
						[] => true,
						['δ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						['μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						['σ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1410);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						['ι', 'φ', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['α', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						['ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						['ι', 'ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3229);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['κ', 'κ', 'α', 'β', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3231);
									s.is_empty()
								}
								['ρ', 'ν', 'α', 'σ', 'σ', 'ε', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['υ', 'ο', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['α', 'γ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['σ', 'ι', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['β', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2572);
							s.is_empty()
						}
						['δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['ε', 'κ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3233);
									s.is_empty()
								}
								['ο', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μα.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['θ', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2579);
							s.is_empty()
						}
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['α', 'ξ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						['η', 'λ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						['η', 'λ', 'α', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3235);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3236);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2586);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3238);
									s.is_empty()
								}
								['ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['η', 'σ', 'τ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['γ', 'ι', s @ ..] => {
							super::push(out, 2590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['ε', 'λ', 'ο', s @ ..] => {
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1462);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1465);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 2594);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2599);
							match s {
								[] => true,
								['β', 'α', 'ι', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3241);
									s.is_empty()
								}
								['σ', 'ε', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['α', 'θ', 'μ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2604);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3243);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 3244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['ι', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['μ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						['ρ', 'ε', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						['σ', 'θ', 'η', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						['τ', 'ο', 'ρ', 'ι', 'ο', 'σ', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						['η', 'ψ', s @ ..] => {
							super::push(out, 2615);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2617);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 2618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1475);
					match s {
						[] => true,
						['υ', 'ρ', 'ι', 'δ', 'ε', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				['χ', 'ω', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['ρ', 'η', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3246);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 3247);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2625);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2626);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2627);
							match s {
								[] => true,
								['α', 'ν', 'τ', 'ο', 'θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2628);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
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
			super::push(out, 368);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['θ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['ρ', '\u{1}', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['η', s @ ..] => {
					match s {
						[] => true,
						['δ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						['λ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['ο', s @ ..] => {
					match s {
						[] => true,
						['λ', 'ο', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2632);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2634);
							match s {
								[] => true,
								['κ', 'ι', 'τ', s @ ..] => {
									super::push(out, 3253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 2635);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2636);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1489);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2638);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['σ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['α', s @ ..] => super::t1::n1521(s, out),
				['ι', s @ ..] => super::t1::n1522(s, out),
				['λ', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['ο', s @ ..] => super::t1::n2764(s, out),
						_ => false,
					}
				}
				['ο', s @ ..] => super::t1::n1524(s, out),
				['ρ', 'ο', 'ν', '\u{1}', s @ ..] => super::t1::n1525(s, out),
				['υ', s @ ..] => super::t1::n1526(s, out),
				['ω', s @ ..] => super::t1::n1527(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n375(s, out),
		_ => false,
	}
}

/// Words ending -ξα.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['ι', 'ω', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οα.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	s.is_empty()
}

/// Words ending -πα.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				['σ', 'χ', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['ι', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 3256);
									s.is_empty()
								}
								['σ', 'ι', s @ ..] => {
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3428);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 3429);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2655);
							s.is_empty()
						}
						['ο', 'σ', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'ε', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['ι', 'ν', s @ ..] => {
					super::push(out, 1508);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['ο', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2663);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ι', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['π', 'τ', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['σ', 'ι', 'α', s @ ..] => {
											super::push(out, 3430);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 3431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'λ', 'η', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2667);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'γ', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρα.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['β', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['κ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1520);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						['λ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2674);
							s.is_empty()
						}
						['μ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						['σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2677);
							s.is_empty()
						}
						['λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['ρ', 'ο', 'σ', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', 'τ', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 1532);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3259);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3432);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 3433);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 3434);
											s.is_empty()
										}
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
		['κ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'υ', 'θ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				['η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1537);
					s.is_empty()
				}
				['ο', 'φ', 'θ', 'α', 'λ', 'μ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['τ', 'ρ', 'ι', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['η', 'τ', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['ε', 'μ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					match s {
						[] => true,
						['μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2689);
							s.is_empty()
						}
						['ρ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'α', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1553);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['υ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['β', 'α', 'λ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						['λ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1556);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1558);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2693);
							match s {
								[] => true,
								['γ', 'ε', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3260);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['τ', 'υ', 'π', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => super::t1::n1597(s, out),
				['ι', s @ ..] => super::t1::n1598(s, out),
				['ο', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['ω', s @ ..] => super::t1::n1600(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n402(s, out),
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		['ε', 'β', s @ ..] => {
			super::push(out, 405);
			s.is_empty()
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
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2708);
							match s {
								[] => true,
								['χ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3266);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['λ', 'η', s @ ..] => {
							super::push(out, 2709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						['ν', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						['φ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						['χ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'ρ', 'ι', 'σ', 'μ', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2718);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 3267);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 3268);
									match s {
										[] => true,
										['α', 'λ', 'ο', s @ ..] => {
											super::push(out, 3437);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2719);
							match s {
								[] => true,
								['ν', '\u{1}', s @ ..] => {
									super::push(out, 3270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['γ', 'ε', 'ι', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						['ν', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3271);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 420);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 421);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['κ', 'τ', 'ο', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['φ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['γ', 'η', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υα.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['λ', 'ο', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φα.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['ρ', 'ε', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'κ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				['σ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'δ', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['θ', 'α', 'ρ', s @ ..] => {
			super::push(out, 428);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['ξ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['σ', 'γ', 'ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χα.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'ρ', 'δ', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 435);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		['ρ', 'η', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψα.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['ι', 'ν', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['θ', 'ι', s @ ..] => {
					match s {
						[] => true,
						['ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ακιδ.
pub(crate) fn n1743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1693);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2780);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιδ.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	s.is_empty()
}

/// Words ending -νιδ.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	s.is_empty()
}

/// Words ending -οιδ.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 1697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πιδ.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	s.is_empty()
}

/// Words ending -ριδ.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	s.is_empty()
}

/// Words ending -σιδ.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	s.is_empty()
}

/// Words ending -φιδ.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χιδ.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	s.is_empty()
}

/// Words ending -ψιδ.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 1699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωιδ.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1700);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μδ.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	s.is_empty()
}

/// Words ending -οδ.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['ι', 'μ', 'α', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['τ', 'ο', 'θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρδ.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['λ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3280);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['σ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['λ', 'α', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 2785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υδ.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['η', '\u{1}', s @ ..] => {
			super::push(out, 554);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['μ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2787);
							s.is_empty()
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
				['μ', 'α', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωδ.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	s.is_empty()
}

/// Words ending -καξε.
pub(crate) fn n1803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1751);
	s.is_empty()
}

/// Words ending -λαξε.
pub(crate) fn n1804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1752);
	s.is_empty()
}

/// Words ending -ηξε.
pub(crate) fn n601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	s.is_empty()
}

/// Words ending -ιξε.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 596);
	match s {
		[] => true,
		['σ', 'ω', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οξε.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	s.is_empty()
}

/// Words ending -υξε.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	s.is_empty()
}

/// Words ending -οε.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	s.is_empty()
}

/// Words ending -πε.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['γ', 'γ', 'ε', 'λ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1755);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 600);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['γ', 'ο', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2801);
							s.is_empty()
						}
						['η', 'κ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						['υ', 'μ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['λ', 'ο', 'υ', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['μ', 'α', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2806);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 603);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 604);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρε.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 606);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3285);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 608);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['ν', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 2810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'ν', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['ο', 'ν', 'ε', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['π', 'ε', 'λ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3287);
									s.is_empty()
								}
								['σ', 'κ', 'η', 'π', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σε.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['χ', 'α', 'ρ', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['ε', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τε.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 616);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υε.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['γ', 'γ', 'ε', 'λ', 'ι', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['υ', 'ν', 'τ', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['ι', 'ν', 'η', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['α', 'τ', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'υ', 'χ', 'η', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φε.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χε.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['ι', s @ ..] => {
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['ο', 'π', 'ο', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μι.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1865);
					match s {
						[] => true,
						['ι', 'ο', 'ν', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δνι.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 719);
	match s {
		[] => true,
		['ο', 'λ', s @ ..] => {
			super::push(out, 1866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ινι.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 720);
	s.is_empty()
}

/// Words ending -ξι.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οι.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['ν', '\u{1}', s @ ..] => {
			super::push(out, 723);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['δ', 'α', 'ι', 'α', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πι.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 726);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 727);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['κ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								['ο', 'ρ', 'υ', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2849);
							s.is_empty()
						}
						['μ', 'α', 'ρ', 'α', 'θ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						['σ', 'ε', 'λ', 'ι', 'ν', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2851);
							s.is_empty()
						}
						['τ', 'ι', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						['φ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ι', 'σ', 'τ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3297);
									s.is_empty()
								}
								['λ', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3298);
									s.is_empty()
								}
								['ο', 'ρ', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3299);
									s.is_empty()
								}
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

/// Words ending -ρι.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	s.is_empty()
}

/// Words ending -σι.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 729);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['τ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 731);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 732);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 733);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['κ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 735);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				['ν', 'δ', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 737);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['δ', 'ο', 'κ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['κ', 'ε', 'ρ', 'α', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						['π', 'ε', 'δ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ο', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τι.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 744);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υι.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['η', '\u{1}', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φι.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	s.is_empty()
}

/// Words ending -χι.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2860);
							match s {
								[] => true,
								['μ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3442);
											s.is_empty()
										}
										['τ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3443);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2862);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2863);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 3300);
									match s {
										[] => true,
										['η', 'ρ', 'α', 'σ', s @ ..] => {
											super::push(out, 3444);
											s.is_empty()
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 3301);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 3302);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 3303);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2864);
							match s {
								[] => true,
								['ι', 'λ', 'η', 'ι', 'σ', 'τ', 'η', 'ρ', '\u{1}', s @ ..] => {
									super::push(out, 3305);
									s.is_empty()
								}
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
			super::push(out, 749);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψι.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωι.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	s.is_empty()
}

/// Words ending -νακ.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 760);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2884);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', 'α', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['θ', s @ ..] => {
			match s {
				[] => true,
				['α', 'ρ', 'ο', s @ ..] => {
					super::push(out, 2886);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2887);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'α', 'β', s @ ..] => {
			super::push(out, 1890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πακ.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['η', 'λ', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρακ.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 762);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2889);
					match s {
						[] => true,
						['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3313);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1896);
			s.is_empty()
		}
		['κ', 'ι', 'ν', 'ο', s @ ..] => {
			super::push(out, 1897);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1898);
			s.is_empty()
		}
		['π', 'α', s @ ..] => {
			super::push(out, 1899);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['α', 'ζ', 'ω', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2891);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['α', 'τ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2893);
					s.is_empty()
				}
				['ο', 'φ', 'υ', 'λ', 'λ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1902);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σακ.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 763);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2895);
					match s {
						[] => true,
						['τ', 'ε', 'ρ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3447);
									s.is_empty()
								}
								['υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', 'ω', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τακ.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 764);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['β', 'ο', s @ ..] => {
					super::push(out, 2896);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 3315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', 'λ', s @ ..] => {
					super::push(out, 2897);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2898);
					match s {
						[] => true,
						['γ', 'ι', 'σ', 'μ', s @ ..] => {
							super::push(out, 3316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['υ', 'τ', 'η', 'ρ', s @ ..] => {
							super::push(out, 3317);
							match s {
								[] => true,
								['ι', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2901);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 3318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2902);
					s.is_empty()
				}
				['ρ', 'ρ', 'ο', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 3319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2904);
					s.is_empty()
				}
				['χ', 'υ', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['σ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1907);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1908);
			match s {
				[] => true,
				['ι', 'κ', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3321);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 3322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1909);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1910);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψακ.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 767);
	s.is_empty()
}

/// Words ending -εκ.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1911);
					match s {
						[] => true,
						['δ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1912);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2908);
							match s {
								[] => true,
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3323);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3325);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2909);
							match s {
								[] => true,
								['ε', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3327);
									s.is_empty()
								}
								['ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 774);
			match s {
				[] => true,
				['α', 'λ', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['ο', 'σ', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηκ.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['ι', 'ν', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['υ', 'κ', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ικ.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['ω', 'τ', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['ι', 'ν', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 780);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['μ', 'ε', 'ρ', 'ι', 'α', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['β', 'α', 'ρ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 3329);
									s.is_empty()
								}
								['μ', 'ω', 'μ', 'ο', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['κ', 'ι', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				['σ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['μ', 'π', 'ε', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'σ', s @ ..] => {
							super::push(out, 2915);
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
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 787);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λκ.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				['ν', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 791);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1932);
					match s {
						[] => true,
						['α', 'κ', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1933);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 793);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νκ.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 795);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οκ.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 798);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['κ', 'υ', 'μ', s @ ..] => {
					super::push(out, 1936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1937);
					match s {
						[] => true,
						['σ', 'τ', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1939);
					match s {
						[] => true,
						['β', 'δ', 'α', 'ι', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 801);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1940);
					match s {
						[] => true,
						['ψ', 'η', 'ν', 'ο', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['α', 'δ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1942);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1943);
					match s {
						[] => true,
						['α', 'ν', 'ν', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1944);
					match s {
						[] => true,
						['ν', 'θ', s @ ..] => {
							super::push(out, 2924);
							s.is_empty()
						}
						['φ', 'α', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 805);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['η', '\u{1}', s @ ..] => {
									super::push(out, 3332);
									s.is_empty()
								}
								['ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 3333);
									s.is_empty()
								}
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

/// Words ending -ρκ.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 2927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ι', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['η', 'ρ', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 808);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 809);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['γ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['θ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['π', 'τ', s @ ..] => {
					match s {
						[] => true,
						['ε', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						['η', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				['ψ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τκ.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 814);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 815);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 816);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υκ.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['ε', 'ρ', 'τ', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 821);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 823);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['ι', 'ν', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['τ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['λ', 'ι', s @ ..] => {
									super::push(out, 3334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1970);
					match s {
						[] => true,
						['ν', s @ ..] => {
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3335);
									s.is_empty()
								}
								['ο', 'ν', s @ ..] => {
									super::push(out, 3336);
									s.is_empty()
								}
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
			super::push(out, 827);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1972);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3337);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1973);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1977);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				['ι', s @ ..] => {
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
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
		['σ', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['ι', '\u{1}', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωκ.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['ο', 'β', 'α', 'θ', 'ρ', 'ι', s @ ..] => {
					super::push(out, 1984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['ω', 'δ', 'ι', 'ο', s @ ..] => {
					super::push(out, 1985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1986);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιραδαπμαλ.
pub(crate) fn n3637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3466);
	s.is_empty()
}

/// Words ending -ιροφηδαπμαλ.
pub(crate) fn n3638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3467);
	s.is_empty()
}

/// Words ending -σιδαπμαλ.
pub(crate) fn n3639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3468);
	s.is_empty()
}

/// Words ending -δοδαπμαλ.
pub(crate) fn n3640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3469);
	s.is_empty()
}

/// Words ending -υοδαπμαλ.
pub(crate) fn n3641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3470);
	s.is_empty()
}

/// Words ending -ιλ.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2012);
					match s {
						[] => true,
						['ν', 'ο', 'σ', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 859);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2014);
					s.is_empty()
				}
				['ο', 'κ', s @ ..] => {
					super::push(out, 2015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['ε', 'ν', 'α', s @ ..] => {
					super::push(out, 2016);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2017);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['α', 'ρ', 'ε', s @ ..] => {
					super::push(out, 2019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2020);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλ.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['δ', 'η', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['α', 'ψ', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				['η', '\u{1}', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['ι', 'σ', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['ε', 'ρ', 'ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				['ι', 'μ', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				['ρ', 'ω', 'τ', s @ ..] => {
					super::push(out, 2033);
					s.is_empty()
				}
				['ω', 'ρ', '\u{1}', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ησαμ.
pub(crate) fn n2126(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2979);
			s.is_empty()
		}
		['σ', 'ι', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2980);
			s.is_empty()
		}
		['τ', 'η', 'ρ', '\u{1}', s @ ..] => {
			super::push(out, 2981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σσαμ.
pub(crate) fn n2128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2056);
	s.is_empty()
}

/// Words ending -τσαμ.
pub(crate) fn n2129(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', 'ξ', '\u{1}', s @ ..] => {
			super::push(out, 2982);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['γ', 'ο', s @ ..] => {
					super::push(out, 3346);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 3347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2984);
			s.is_empty()
		}
		['ρ', 'ο', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2985);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ηλαχσαμ.
pub(crate) fn n2130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2057);
	s.is_empty()
}

/// Words ending -ταμ.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 888);
	match s {
		[] => true,
		['α', 'ι', 'ο', 'λ', 'ο', 'γ', 'ο', s @ ..] => {
			super::push(out, 2058);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υαμ.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 889);
	s.is_empty()
}

/// Words ending -φαμ.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 890);
	s.is_empty()
}

/// Words ending -χαμ.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	s.is_empty()
}

/// Words ending -εμ.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						['ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2060);
					match s {
						[] => true,
						['γ', 'χ', 'ο', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2062);
					match s {
						[] => true,
						['κ', 'ρ', 'α', 'τ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						['μ', 'η', 'λ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['ι', 'σ', 'μ', 'ο', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['γ', 'κ', 'υ', 'λ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2065);
					match s {
						[] => true,
						['β', 'ο', 'λ', 'ε', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 2998);
							s.is_empty()
						}
						['π', 'τ', s @ ..] => {
							super::push(out, 2999);
							s.is_empty()
						}
						['φ', 'ρ', 'α', s @ ..] => {
							super::push(out, 3000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2067);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ημ.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['ο', 'κ', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['ι', s @ ..] => {
					match s {
						[] => true,
						['γ', 'γ', s @ ..] => {
							super::push(out, 3001);
							s.is_empty()
						}
						['σ', 'κ', s @ ..] => {
							super::push(out, 3002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['α', 'ν', 'η', 'μ', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιμ.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['λ', s @ ..] => {
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				['τ', 'ε', s @ ..] => {
					super::push(out, 2075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2076);
					match s {
						[] => true,
						['ι', 'κ', 'υ', 'λ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3003);
							s.is_empty()
						}
						['ρ', 'κ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['θ', 'η', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 907);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2083);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νμ.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['σ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομ.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['σ', 'υ', 'ν', 'ο', s @ ..] => {
					super::push(out, 2090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμ.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 918);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3006);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 3007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 920);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 922);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 923);
			s.is_empty()
		}
		['ξ', s @ ..] => {
			super::push(out, 924);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2093);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2094);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3008);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['β', 'λ', 'υ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2097);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2098);
					match s {
						[] => true,
						['ι', 'ν', 'ι', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2100);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 3013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2102);
					match s {
						[] => true,
						['α', 'γ', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', 'λ', s @ ..] => {
			super::push(out, 927);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωμ.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σο.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['ο', 'τ', s @ ..] => {
					super::push(out, 2155);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['ρ', 'ι', 'ο', 'θ', 'η', 'κ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2159);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['α', 'κ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['σ', 'μ', s @ ..] => {
									super::push(out, 3348);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3349);
									s.is_empty()
								}
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
			super::push(out, 994);
			match s {
				[] => true,
				['υ', 'α', 'λ', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υο.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2162);
					match s {
						[] => true,
						['ν', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3350);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3456);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'σ', 'κ', 'ο', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιφο.
pub(crate) fn n1009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χο.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 998);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['ρ', 'ω', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψο.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['ι', 'μ', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λεπ.
pub(crate) fn n1032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1020);
	match s {
		[] => true,
		['α', 'ρ', s @ ..] => {
			super::push(out, 2191);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2192);
			match s {
				[] => true,
				['γ', 'ν', 'ο', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μεπ.
pub(crate) fn n1033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	s.is_empty()
}

/// Words ending -νεπ.
pub(crate) fn n1034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 2193);
			s.is_empty()
		}
		['τ', 'α', s @ ..] => {
			super::push(out, 2194);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3057);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3058);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 3059);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πεπ.
pub(crate) fn n1035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1023);
	s.is_empty()
}

/// Words ending -ρεπ.
pub(crate) fn n1036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1024);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2195);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2196);
			match s {
				[] => true,
				['ι', 'κ', 'ο', s @ ..] => {
					super::push(out, 3061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2197);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3062);
					s.is_empty()
				}
				['ν', 'ε', 'ω', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				['ο', 'χ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 3064);
					s.is_empty()
				}
				['σ', s @ ..] => {
					match s {
						[] => true,
						['π', 'ω', s @ ..] => {
							super::push(out, 3359);
							s.is_empty()
						}
						['τ', 'υ', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ο', 'ν', s @ ..] => {
					super::push(out, 3065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2198);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2199);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2200);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2201);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεπ.
pub(crate) fn n1037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1025);
	match s {
		[] => true,
		['α', 'σ', s @ ..] => {
			super::push(out, 2203);
			match s {
				[] => true,
				['ι', 'τ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3066);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηπ.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιπ.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1032);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1033);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3068);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2213);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λπ.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2215);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['ν', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 3072);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3073);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3074);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3361);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 3362);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 3363);
									s.is_empty()
								}
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
			super::push(out, 1040);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1042);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1043);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νπ.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['ι', 'γ', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οπ.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['α', 'ν', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3075);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3076);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3077);
							match s {
								[] => true,
								['η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['ο', 'υ', s @ ..] => {
									super::push(out, 3366);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3367);
									match s {
										[] => true,
										['α', 'γ', 'μ', 'ο', 'σ', 'υ', 'ν', 'η', '\u{1}', s @ ..] => {
											super::push(out, 3457);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['φ', 'ο', 'λ', 'υ', 'γ', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['μ', 'ο', 'γ', 'ε', 'ι', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρπ.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['γ', 'μ', 'α', 'τ', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['ι', 'σ', s @ ..] => {
							super::push(out, 3081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['η', 'ν', s @ ..] => {
							super::push(out, 3082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['σ', 'β', 'υ', 'τ', 'ε', 'ρ', 'ι', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['τ', 'α', 'ν', 'ε', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['τ', 'α', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['γ', 'ω', 'ν', 'ι', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3084);
							s.is_empty()
						}
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
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['ρ', 'μ', 'ι', 'κ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['γ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['ξ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				['σ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['χ', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υπ.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1064);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'ι', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['τ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['ω', 'ρ', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 1069);
			s.is_empty()
		}
		['ο', 'σ', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2256);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3085);
							match s {
								[] => true,
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3368);
									s.is_empty()
								}
								['ο', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2257);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 3086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['μ', 'α', 'ν', 'τ', 'ε', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωπ.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

