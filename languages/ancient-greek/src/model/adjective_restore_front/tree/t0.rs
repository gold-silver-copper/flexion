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
					super::push(out, 48);
					match s {
						[] => true,
						['β', 'υ', 'λ', 'ω', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 419);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 420);
							match s {
								[] => true,
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 421);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 422);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 423);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1270);
									match s {
										[] => true,
										['υ', 'σ', 'τ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 1271);
									match s {
										[] => true,
										['ω', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 1921);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', 'β', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 424);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 425);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1272);
									match s {
										[] => true,
										['α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 1273);
									match s {
										[] => true,
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 1923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 426);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['β', 'ι', 'α', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							super::push(out, 429);
							match s {
								[] => true,
								['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 430);
							match s {
								[] => true,
								['ι', 'σ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1276);
									s.is_empty()
								}
								['ξ', '\u{1}', s @ ..] => {
									super::push(out, 1277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['μ', 'β', 'υ', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 432);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 433);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 434);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1278);
									match s {
										[] => true,
										['υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1924);
											s.is_empty()
										}
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
					super::push(out, 54);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 435);
							match s {
								[] => true,
								['χ', 'υ', s @ ..] => {
									match s {
										[] => true,
										['δ', 'α', 'κ', 'τ', 'υ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1925);
											s.is_empty()
										}
										['τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', 'σ', 'α', 'ρ', 'μ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 436);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 437);
							s.is_empty()
						}
						['υ', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['ζ', 'α', 'ν', 'τ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 439);
							s.is_empty()
						}
						['σ', 'σ', 'ο', 'δ', 'ο', 'μ', 'ε', 'υ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 56);
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
					super::push(out, 57);
					match s {
						[] => true,
						['ι', 'η', 'ο', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 441);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 442);
							match s {
								[] => true,
								['α', 'κ', 'τ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1927);
											s.is_empty()
										}
										['ο', 'φ', 'α', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1928);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', 'λ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 443);
							match s {
								[] => true,
								['ψ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', 'τ', 'ρ', 'ι', 'μ', 'α', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 444);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['ν', 'ε', s @ ..] => {
							super::push(out, 446);
							match s {
								[] => true,
								['α', 'λ', 'ο', 'γ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['γ', 'γ', 'ρ', 'α', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 448);
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
							super::push(out, 449);
							match s {
								[] => true,
								['φ', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1320(s, out),
								_ => false,
							}
						}
						['ι', 'σ', 'χ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n453(s, out),
						['υ', s @ ..] => super::t0::n454(s, out),
						_ => false,
					}
				}
				['ν', s @ ..] => super::t0::n62(s, out),
				['ο', s @ ..] => super::t0::n63(s, out),
				['ρ', s @ ..] => super::t0::n64(s, out),
				['υ', s @ ..] => super::t0::n65(s, out),
				_ => false,
			}
		}
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
		['ῥ', s @ ..] => super::t0::n24(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 263);
					match s {
						[] => true,
						['θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'α', 'β', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				['ο', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 266);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 267);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['δ', 'ι', 'α', 'ι', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1775);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 1776);
									s.is_empty()
								}
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
			super::push(out, 27);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 268);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1038);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['π', 'ο', 'ι', s @ ..] => {
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2273);
													match s {
														[] => true,
														['ω', 'ν', '\u{1}', s @ ..] => super::t0::n2408(s, out),
														_ => false,
													}
												}
												['η', s @ ..] => {
													super::push(out, 2274);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n2409(s, out),
														_ => false,
													}
												}
												['ο', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2275);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						['μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1040);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1041);
							match s {
								[] => true,
								['α', 'κ', 'τ', 'ε', s @ ..] => {
									super::push(out, 1777);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1042);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1779);
									match s {
										[] => true,
										['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2162);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 1780);
									match s {
										[] => true,
										['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2163);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 1781);
									match s {
										[] => true,
										['μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 269);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1044);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['λ', 'ι', s @ ..] => {
									super::push(out, 1782);
									match s {
										[] => true,
										['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2165);
											s.is_empty()
										}
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
					super::push(out, 270);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1046);
							match s {
								[] => true,
								['ε', 'α', 'λ', 'ο', 'γ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', 'ν', 'ω', 'ρ', '\u{1}', s @ ..] => {
					super::push(out, 271);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 272);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1047);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'α', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 274);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1049);
							match s {
								[] => true,
								['φ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1051);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 276);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1054);
							match s {
								[] => true,
								['μ', 'μ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => super::t0::n1074(s, out),
						['ο', s @ ..] => {
							super::push(out, 1056);
							match s {
								[] => true,
								['ν', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1865(s, out),
								_ => false,
							}
						}
						['υ', 'π', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1076(s, out),
						_ => false,
					}
				}
				['χ', s @ ..] => super::t0::n277(s, out),
				['ω', s @ ..] => super::t0::n278(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n28(s, out),
		['ε', s @ ..] => super::t0::n29(s, out),
		['ζ', s @ ..] => super::t0::n30(s, out),
		['η', s @ ..] => super::t0::n31(s, out),
		['θ', s @ ..] => super::t0::n32(s, out),
		['ι', s @ ..] => super::t0::n33(s, out),
		['κ', s @ ..] => super::t0::n34(s, out),
		['λ', s @ ..] => super::t0::n35(s, out),
		['μ', s @ ..] => super::t0::n36(s, out),
		['ν', s @ ..] => super::t0::n37(s, out),
		['ξ', s @ ..] => super::t0::n38(s, out),
		['ο', s @ ..] => super::t0::n39(s, out),
		['π', s @ ..] => super::t0::n40(s, out),
		['ρ', s @ ..] => super::t0::n41(s, out),
		['σ', s @ ..] => super::t0::n42(s, out),
		['τ', s @ ..] => super::t0::n43(s, out),
		['υ', s @ ..] => super::t0::n44(s, out),
		['φ', s @ ..] => super::t0::n45(s, out),
		['χ', s @ ..] => super::t0::n46(s, out),
		['ψ', s @ ..] => super::t0::n47(s, out),
		_ => false,
	}
}

/// The word σορυφαλγ.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	s.is_empty()
}

/// The word σορχσιλγ.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	s.is_empty()
}

/// Words ending -υλγ.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['κ', 'υ', s @ ..] => {
			match s {
				[] => true,
				['π', 'ι', 'κ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['φ', 'θ', 'ο', 'γ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νγ.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	s.is_empty()
}

/// Words ending -ογ.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['ν', 'υ', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ργ.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 454);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υγ.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 455);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['ν', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 458);
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
			super::push(out, 66);
			match s {
				[] => true,
				['δ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						['ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						['ν', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						['φ', 'ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['τ', 'υ', 'λ', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 462);
					match s {
						[] => true,
						['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1293);
							match s {
								[] => true,
								['τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['α', 'ι', 'γ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['α', 'γ', 'ω', 'γ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1297);
							match s {
								[] => true,
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 470);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1300);
							match s {
								[] => true,
								['ι', 'γ', 'ν', 'ω', 'σ', 'κ', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['ο', 'σ', 'ι', 'ο', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1303);
							match s {
								[] => true,
								['υ', s @ ..] => {
									match s {
										[] => true,
										['σ', 'ι', 'φ', 'ι', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2200);
											s.is_empty()
										}
										['τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2201);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1304);
							match s {
								[] => true,
								['τ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						['σ', 'π', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1936);
									s.is_empty()
								}
								['σ', 'κ', 'α', 'λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['φ', 'θ', 'α', 'ρ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				['ι', 'π', 'ε', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						['ρ', 'ο', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						['τ', 'α', 'μ', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'η', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['γ', 'ε', 'ν', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['λ', 'α', 'σ', 'ι', 'ο', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						['τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['υ', 'λ', 'λ', 'α', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['γ', 'μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ο', 'τ', 'ρ', 'ο', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1938);
									s.is_empty()
								}
								['χ', s @ ..] => {
									match s {
										[] => true,
										['α', 'υ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2202);
											s.is_empty()
										}
										['ο', s @ ..] => {
											match s {
												[] => true,
												['δ', 'ρ', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2282);
													s.is_empty()
												}
												['σ', 'κ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2283);
													s.is_empty()
												}
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
				['ν', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['ι', 'κ', 'ο', 'φ', 'α', 'ν', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						['μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						['σ', s @ ..] => super::t0::n1362(s, out),
						_ => false,
					}
				}
				['ι', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => super::t0::n492(s, out),
				['ο', s @ ..] => super::t0::n493(s, out),
				['υ', s @ ..] => super::t0::n494(s, out),
				['ω', s @ ..] => super::t0::n495(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n72(s, out),
		['ω', s @ ..] => super::t0::n73(s, out),
		_ => false,
	}
}

/// Words ending -ε.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				['ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				['σ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['δ', 'ο', 'μ', 'η', 'κ', 'ο', 'ν', 'τ', 'α', 'κ', 'α', 'ι', 'ε', 'κ', 'α', 'τ', 'ο', 'ν', 'τ', 'α', 'π', 'λ', 'α', 'σ', 'ι', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['τ', 'α', 'σ', 'τ', 'η', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1326);
							match s {
								[] => true,
								['ι', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1942);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1943);
									s.is_empty()
								}
								['ρ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['ε', 'κ', 'υ', 'δ', 'ο', 'ι', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['ι', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				['θ', 'ι', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['γ', 'α', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1329);
							s.is_empty()
						}
						['η', 'ν', 'α', 'ρ', 'χ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['ε', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1331);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['λ', 'η', 'σ', 'ι', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 512);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1335);
							match s {
								[] => true,
								['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['α', 'γ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1336);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1337);
							s.is_empty()
						}
						['ι', 'π', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1338);
							s.is_empty()
						}
						['υ', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'τ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 515);
					match s {
						[] => true,
						['α', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1946);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1947);
									s.is_empty()
								}
								['τ', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 1948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['ι', 'ξ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ζ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1949);
									s.is_empty()
								}
								['σ', 'θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1950);
									s.is_empty()
								}
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
			super::push(out, 82);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
						['ρ', 'α', 'κ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						['υ', 'ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						['υ', 'τ', 'ε', 'υ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1348);
							s.is_empty()
						}
						['κ', 'ο', 'σ', 'ι', 'ο', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						['ν', 'τ', 'ι', 'ο', s @ ..] => {
							super::push(out, 1350);
							match s {
								[] => true,
								['υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'γ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						['σ', 'τ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['ε', 'κ', s @ ..] => {
							super::push(out, 1354);
							match s {
								[] => true,
								['α', 'σ', 'υ', 'λ', 'λ', 'α', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['υ', 'μ', 'η', 'θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['α', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['δ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['α', s @ ..] => super::t0::n534(s, out),
				['ε', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				['ο', s @ ..] => super::t0::n537(s, out),
				['υ', 'π', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n538(s, out),
				['ω', s @ ..] => super::t0::n539(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t0::n85(s, out),
		['ρ', s @ ..] => super::t0::n86(s, out),
		['σ', s @ ..] => super::t0::n87(s, out),
		['τ', s @ ..] => super::t0::n88(s, out),
		['υ', s @ ..] => super::t0::n89(s, out),
		['φ', s @ ..] => super::t0::n90(s, out),
		['χ', s @ ..] => super::t0::n91(s, out),
		['ψ', s @ ..] => super::t0::n92(s, out),
		_ => false,
	}
}

/// Words ending -ζ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['κ', 'υ', 'ν', 'θ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 94);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['γ', 'ρ', 'α', 'φ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['δ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 577);
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
			super::push(out, 98);
			match s {
				[] => true,
				['α', 'π', 'η', 'κ', 'ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['η', 'σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['υ', 'π', 'ν', 'ο', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['κ', 'υ', 'κ', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['λ', 'ε', 'π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', 'ι', 'ω', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['ι', 'ο', 'δ', 'ε', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 108);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						['π', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['ψ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['σ', 'α', 'λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['ι', 'κ', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['ρ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'β', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['σ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'δ', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['λ', 'α', 'κ', 'ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['μ', 'ι', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['σ', 'α', 'ν', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['ρ', 'α', 'κ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
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
			super::push(out, 117);
			match s {
				[] => true,
				['λ', 'ε', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1415);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1416);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 1976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 120);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 121);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				['λ', 'υ', 'ρ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['α', 'ν', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1420);
							s.is_empty()
						}
						['ο', 'γ', 'ε', 'ν', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['δ', 'ν', 'ε', 'φ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', 'γ', 'ω', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1422);
							s.is_empty()
						}
						['ι', 'α', 'τ', 'ρ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1423);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1424);
							match s {
								[] => true,
								['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
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
		['ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1426);
							match s {
								[] => true,
								['ι', 'α', 'ι', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1978);
									s.is_empty()
								}
								['υ', 'ν', 'α', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1427);
							s.is_empty()
						}
						['μ', 'ε', 'τ', 'ρ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1428);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1429);
							s.is_empty()
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1980);
									s.is_empty()
								}
								['κ', 'ε', 'λ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1430);
							s.is_empty()
						}
						['ψ', 'η', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'α', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['ι', 'α', 'δ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1433);
							s.is_empty()
						}
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1434);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['ρ', 'ο', 'τ', s @ ..] => {
									super::push(out, 1982);
									match s {
										[] => true,
										['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2206);
											s.is_empty()
										}
										['ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2207);
											s.is_empty()
										}
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
					super::push(out, 633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['α', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['θ', 'υ', s @ ..] => {
					match s {
						[] => true,
						['ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1436);
							s.is_empty()
						}
						['π', 'α', 'γ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						['σ', 'τ', 'ε', 'φ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						['ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['β', 'η', 'λ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 636);
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
			super::push(out, 133);
			match s {
				[] => true,
				['γ', 'κ', 'α', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1440);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1983);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									match s {
										[] => true,
										['τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2208);
											s.is_empty()
										}
										['ω', s @ ..] => {
											super::push(out, 2209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1442);
							match s {
								[] => true,
								['σ', 'τ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 639);
					match s {
						[] => true,
						['ρ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1985);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 1986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1447);
							match s {
								[] => true,
								['ι', s @ ..] => {
									match s {
										[] => true,
										['γ', 'ρ', 'α', 'φ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2210);
											s.is_empty()
										}
										['π', 'υ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2211);
											s.is_empty()
										}
										['ω', 'ν', 'υ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1449);
							match s {
								[] => true,
								['φ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1450);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						['ψ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['α', 'β', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1453);
							s.is_empty()
						}
						['ν', 'α', 'β', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1454);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1455);
							match s {
								[] => true,
								['ν', 'ι', s @ ..] => {
									match s {
										[] => true,
										['ζ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2213);
											s.is_empty()
										}
										['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
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
				['ρ', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1457);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1989);
									match s {
										[] => true,
										['λ', 'ι', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2215);
											s.is_empty()
										}
										['σ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1459);
							match s {
								[] => true,
								['κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1990);
									s.is_empty()
								}
								['σ', 'τ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', 'η', 'δ', 'ο', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['σ', 'ι', 'τ', 'ε', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['α', s @ ..] => super::t0::n1506(s, out),
						['ε', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1466);
							match s {
								[] => true,
								['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n134(s, out),
		['η', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['ρ', s @ ..] => super::t0::n659(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n136(s, out),
		['λ', s @ ..] => super::t0::n137(s, out),
		['ν', s @ ..] => super::t0::n138(s, out),
		['ο', s @ ..] => super::t0::n139(s, out),
		['ρ', s @ ..] => super::t0::n140(s, out),
		['τ', s @ ..] => super::t0::n141(s, out),
		['υ', s @ ..] => super::t0::n142(s, out),
		['ω', s @ ..] => super::t0::n143(s, out),
		_ => false,
	}
}

/// Words ending -λ.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['ι', 'φ', 'ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1506);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						['ψ', 'η', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1511);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2007);
									match s {
										[] => true,
										['δ', s @ ..] => {
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2289);
													match s {
														[] => true,
														['ο', 'σ', '\u{1}', s @ ..] => super::t0::n2412(s, out),
														_ => false,
													}
												}
												['ο', 'δ', 'ρ', 'ο', 'μ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2290);
													s.is_empty()
												}
												['ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2291);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'σ', 'σ', 'ο', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				['χ', 'α', 'ν', 'η', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['α', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						['υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['ι', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['θ', 'α', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['σ', 'τ', 'ι', 'κ', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2009);
									s.is_empty()
								}
								['φ', 'θ', 'ο', 'γ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['α', 'ρ', 'γ', 'υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							match s {
								[] => true,
								['η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2011);
									s.is_empty()
								}
								['ο', 'τ', 'ρ', 'ο', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['σ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ζ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2013);
									s.is_empty()
								}
								['σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2014);
									s.is_empty()
								}
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
			super::push(out, 149);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1519);
							match s {
								[] => true,
								['λ', 'γ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['μ', s @ ..] => {
									match s {
										[] => true,
										['ε', 'λ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2233);
											s.is_empty()
										}
										['ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2234);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'ε', 'λ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1521);
							match s {
								[] => true,
								['ω', s @ ..] => {
									match s {
										[] => true,
										['σ', 'ι', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2235);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 2236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 716);
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
			super::push(out, 150);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['α', 'ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['β', 'α', 'θ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2018);
									match s {
										[] => true,
										['ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2237);
											s.is_empty()
										}
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
					super::push(out, 721);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								['χ', 'α', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
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
				['ρ', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['α', 'ν', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1526);
							match s {
								[] => true,
								['α', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['τ', 'ι', s @ ..] => {
							super::push(out, 1528);
							match s {
								[] => true,
								['γ', 'ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2022);
									s.is_empty()
								}
								['χ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1529);
							match s {
								[] => true,
								['θ', 'υ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2025);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1531);
							match s {
								[] => true,
								['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2027);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1532);
							match s {
								[] => true,
								['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2029);
									s.is_empty()
								}
								['λ', 'ω', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2030);
									s.is_empty()
								}
								['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1534);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2032);
									match s {
										[] => true,
										['α', 'λ', 'λ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2238);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', 'τ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2033);
									s.is_empty()
								}
								['φ', 'υ', 'σ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						['υ', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'ρ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['ν', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['ξ', 'ο', 'β', 'α', 'ρ', 'β', 'α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['α', 'ν', 'θ', 'ρ', 'ω', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						['η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1544);
							match s {
								[] => true,
								['ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'γ', 'υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['η', 'σ', 'ι', 'π', 'η', 'μ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['υ', 'β', 'δ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ο', 'σ', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['α', 'κ', 'α', 'ν', 'θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 747);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['ι', 'ο', 'σ', 'ο', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 749);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				['ρ', s @ ..] => super::t0::n759(s, out),
				['σ', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['τ', 'ι', 'λ', 'η', 'ν', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 157);
			s.is_empty()
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
			super::push(out, 158);
			match s {
				[] => true,
				['ζ', 'ε', 'ι', 'ρ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['ξ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				['ρ', 'θ', 'η', 'κ', 'ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['μ', 'α', 'χ', 'η', 'σ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						['σ', 'ι', 'κ', 'λ', 'ε', 'ι', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['υ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2038);
									s.is_empty()
								}
								['κ', 'η', 'κ', 'ω', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['δ', 'α', 'μ', 'ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['ι', 'ω', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', 'α', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['κ', 'τ', 'ι', 'λ', 'α', 'θ', 'ρ', 'α', 'ι', 'ο', 'φ', 'α', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['ι', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 767);
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
			super::push(out, 165);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['β', s @ ..] => {
							match s {
								[] => true,
								['α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2040);
									s.is_empty()
								}
								['ο', 'λ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['α', 'χ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'α', 'σ', '\u{1}', s @ ..] => {
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
				['ν', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['γ', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1562);
							s.is_empty()
						}
						['θ', 'η', 'κ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						['τ', 'ε', 'θ', 'α', 'μ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1564);
							s.is_empty()
						}
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
		['β', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'μ', 'ο', 'θ', 'υ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						['υ', 'ζ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['ν', 'η', 'φ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1567);
							s.is_empty()
						}
						['ρ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['ι', 'δ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['σ', 'υ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['ι', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['γ', 'α', 'ρ', 'χ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['λ', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1573);
							match s {
								[] => true,
								['υ', 'ρ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						['ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['α', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['λ', 'η', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1576);
							s.is_empty()
						}
						['χ', 'λ', 'ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['α', 'γ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['μ', 'α', s @ ..] => {
							match s {
								[] => true,
								['σ', 'τ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2241);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'ο', 'θ', 'ε', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'χ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['γ', 'α', 'λ', 'α', 'κ', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1579);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['σ', 'θ', 'ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'ι', 'τ', 'α', 'γ', 'ω', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				['ο', 'β', 'α', 'λ', 'σ', 'α', 'μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['ι', 'λ', s @ ..] => {
							match s {
								[] => true,
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2046);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['ρ', 'ο', 'φ', 'ο', 'ι', 'τ', 'ο', 'σ', 'υ', 'κ', 'ο', 'φ', 'α', 'ν', 'τ', 'ο', 'δ', 'ι', 'κ', 'ο', 'τ', 'α', 'λ', 'α', 'ι', 'π', 'ω', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['τ', 'ρ', 'α', 'κ', 'ο', 'δ', 'ε', 'ρ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['ρ', 'υ', 'ν', 'ε', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2048);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', 'ι', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['τ', 'ι', 'δ', 'α', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['α', 'λ', 'μ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
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
			super::push(out, 184);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['ο', 'γ', 'ν', 'ω', 'μ', 'ο', 'ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['δ', 'α', 'γ', 'ω', 'γ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1586);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1587);
							match s {
								[] => true,
								['ο', 'υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'υ', 'ρ', 'η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['μ', s @ ..] => {
							match s {
								[] => true,
								['α', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2051);
									s.is_empty()
								}
								['ε', 'γ', 'ε', 'θ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'ο', 'ι', 'κ', 'ι', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['φ', 'υ', 'λ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						['ψ', 'υ', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1592);
							match s {
								[] => true,
								['ι', 'ο', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2053);
									s.is_empty()
								}
								['κ', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1594);
							match s {
								[] => true,
								['α', 'θ', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ι', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2056);
									s.is_empty()
								}
								['η', 'ν', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1596);
							match s {
								[] => true,
								['γ', 'ι', 'γ', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2058);
									s.is_empty()
								}
								['θ', 'α', 'λ', 'α', 'σ', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2059);
									s.is_empty()
								}
								['ι', 'τ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						['θ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1599);
							match s {
								[] => true,
								['τ', 'η', 'τ', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['τ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2062);
									s.is_empty()
								}
								['φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							match s {
								[] => true,
								['α', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2064);
									s.is_empty()
								}
								['ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['ρ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['α', 'ρ', 'χ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2066);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 815);
					match s {
						[] => true,
						['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2068);
									match s {
										[] => true,
										['α', 'κ', 'τ', 'υ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2243);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									match s {
										[] => true,
										['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2244);
											s.is_empty()
										}
										['ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2245);
											s.is_empty()
										}
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
			super::push(out, 185);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1603);
							match s {
								[] => true,
								['σ', 'γ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'τ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2070);
									match s {
										[] => true,
										['σ', 'τ', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2246);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n833(s, out),
				['ρ', s @ ..] => super::t0::n834(s, out),
				['τ', s @ ..] => super::t0::n835(s, out),
				['υ', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				['φ', s @ ..] => super::t0::n837(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n186(s, out),
		['ι', s @ ..] => super::t0::n187(s, out),
		['λ', s @ ..] => super::t0::n188(s, out),
		['ν', s @ ..] => super::t0::n189(s, out),
		['ο', s @ ..] => super::t0::n190(s, out),
		['ρ', s @ ..] => super::t0::n191(s, out),
		['τ', s @ ..] => super::t0::n192(s, out),
		['υ', s @ ..] => super::t0::n193(s, out),
		['ω', s @ ..] => super::t0::n194(s, out),
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				['θ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				['κ', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 861);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 862);
					match s {
						[] => true,
						['δ', 'α', 'ρ', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						['τ', 'α', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 863);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['δ', 'α', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['β', 'α', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['ι', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['η', 'μ', 'α', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['μ', 'α', 'ν', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 870);
					match s {
						[] => true,
						['α', 'μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						_ => false,
					}
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
				['γ', 'μ', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 871);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['υ', 'ω', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['ν', 'α', 'π', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['μ', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['δ', 'α', 'σ', 'θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						['ψ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['λ', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['θ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						['τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['λ', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['ρ', 'ν', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['φ', 'ι', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['ν', 'ι', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
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
					super::push(out, 889);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['ρ', 'α', 'δ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 892);
					match s {
						[] => true,
						['ι', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1659);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['α', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1662);
							match s {
								[] => true,
								['α', 'χ', 'ι', 'ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2099);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1663);
							match s {
								[] => true,
								['ι', 'σ', 'κ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', 'α', 'ν', 'η', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['β', 'α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['χ', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1668);
							match s {
								[] => true,
								['β', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2101);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						['ι', 'φ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1671);
							match s {
								[] => true,
								['γ', 'γ', 'υ', 'λ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2263);
											s.is_empty()
										}
										['κ', 'α', 'υ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', 'φ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['ε', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n2376(s, out),
										['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n2377(s, out),
										_ => false,
									}
								}
								['τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n2206(s, out),
								_ => false,
							}
						}
						['ρ', 'α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1740(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => super::t0::n912(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n205(s, out),
		['φ', s @ ..] => super::t0::n206(s, out),
		['χ', s @ ..] => super::t0::n207(s, out),
		['ω', s @ ..] => super::t0::n208(s, out),
		_ => false,
	}
}

/// Words ending -τ.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['υ', 'γ', 'λ', 'ω', 'σ', 'σ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['α', 'χ', 'ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1698);
							s.is_empty()
						}
						['β', 'η', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						['υ', 'θ', 'α', 'ν', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 927);
					match s {
						[] => true,
						['ε', 'ρ', 'α', 'π', 'ε', 'υ', 'κ', 'ω', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['ρ', 'ι', 'π', 'π', 'ο', 'τ', 'ρ', 'ο', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['ε', 'σ', 'ι', 'ο', 'υ', 'ρ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['ψ', 'ι', 'μ', 'β', 'ρ', 'ο', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'σ', 'α', 'ρ', 'α', 'κ', 'ο', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1707);
							match s {
								[] => true,
								['ρ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1708);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2122);
									match s {
										[] => true,
										['π', 'ο', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2267);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', 'μ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2123);
									s.is_empty()
								}
								['υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['λ', 'ι', 'κ', 'ο', s @ ..] => {
					match s {
						[] => true,
						['σ', 'δ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1709);
							s.is_empty()
						}
						['υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['ι', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'θ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				['φ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['γ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						['ν', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1713);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						['χ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['ψ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', 'μ', 'α', 'τ', 'ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1718);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1720);
							match s {
								[] => true,
								['ι', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1721);
							match s {
								[] => true,
								['ι', 'θ', 'η', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1722);
							match s {
								[] => true,
								['α', 'θ', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2128);
									match s {
										[] => true,
										['α', 'κ', 'ο', 'δ', 'α', 'ι', 'μ', 'ω', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 2268);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['χ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
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
			super::push(out, 215);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 949);
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
			super::push(out, 216);
			match s {
				[] => true,
				['κ', 'ι', 'ν', 'θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['λ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['ι', 'ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['ε', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						['η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['α', 'υ', 'λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['σ', 'κ', 'υ', 'α', 'μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								['τ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'λ', 'η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['ξ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								['χ', s @ ..] => {
									match s {
										[] => true,
										['ο', s @ ..] => {
											match s {
												[] => true,
												['μ', 'ε', 'ν', s @ ..] => {
													match s {
														[] => true,
														['η', '\u{1}', s @ ..] => super::t0::n2413(s, out),
														['ο', s @ ..] => super::t0::n2414(s, out),
														_ => false,
													}
												}
												['ν', '\u{1}', s @ ..] => {
													super::push(out, 2294);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ω', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 2269);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', 'τ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								['β', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								['ν', 'ε', 'φ', 'ε', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								['ο', 'υ', 'ρ', 'α', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								['υ', 'θ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['ν', 'η', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1735);
							match s {
								[] => true,
								['α', 'ρ', 'β', 'α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						['δ', 'ε', s @ ..] => {
							match s {
								[] => true,
								['ε', 'σ', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2138);
									s.is_empty()
								}
								['ι', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							match s {
								[] => true,
								['ε', 'ι', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2140);
									s.is_empty()
								}
								['ο', 'ρ', 'ι', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'ε', 'ι', 'π', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						['π', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1741);
							match s {
								[] => true,
								['α', 'κ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2142);
									s.is_empty()
								}
								['ρ', 'ο', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ρ', 'γ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['γ', 'ι', 'ν', 'ο', 'β', 'α', 'φ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['ν', 'ω', 'ρ', '\u{1}', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['κ', 'α', 'ρ', 'η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 226);
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
			super::push(out, 227);
			match s {
				[] => true,
				['γ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				['ε', 'θ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				['κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['κ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
								['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'ο', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						['τ', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						['κ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1756);
							s.is_empty()
						}
						['σ', 'ο', 'φ', 'ο', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['ρ', 'α', 'σ', 'π', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1758);
							match s {
								[] => true,
								['ρ', 'γ', 'υ', 'ρ', s @ ..] => {
									match s {
										[] => true,
										['ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2270);
											s.is_empty()
										}
										['ω', 'τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1759);
							match s {
								[] => true,
								['π', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1760);
							match s {
								[] => true,
								['β', 'α', 'ρ', 'β', 'α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2148);
									match s {
										[] => true,
										['ι', 'ν', 'δ', 'υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2272);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', 'ο', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2149);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['ν', 'ι', 'κ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								['ο', 'π', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
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
			super::push(out, 234);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				['ι', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['γ', 'α', 'ν', 'ω', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['λ', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['α', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 236);
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
			super::push(out, 237);
			match s {
				[] => true,
				['ι', 'ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['κ', 'ε', s @ ..] => {
							super::push(out, 1766);
							match s {
								[] => true,
								['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2152);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				['ν', 'α', 'ν', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['α', 'κ', 'τ', 'η', 'ρ', 'ι', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						['ι', 'σ', s @ ..] => {
							super::push(out, 1768);
							match s {
								[] => true,
								['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								['τ', 'η', 'ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['ι', 'ρ', 'ι', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['δ', 'ω', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['ι', 'δ', 'ο', 'ν', s @ ..] => {
							match s {
								[] => true,
								['ε', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
								['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2157);
									s.is_empty()
								}
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
			super::push(out, 239);
			match s {
				[] => true,
				['α', 'μ', 'α', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['ι', 'ζ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['λ', 'ι', 'ο', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['ο', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['ι', 'α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['ρ', 'ι', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['μ', 'α', 'τ', 'ι', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1771);
							match s {
								[] => true,
								['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['σ', 'τ', s @ ..] => {
							match s {
								[] => true,
								['ε', 'π', 'ω', 'ν', 'υ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2159);
									s.is_empty()
								}
								['ι', 'α', 'ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['μ', 'α', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['τ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
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
			super::push(out, 246);
			match s {
				[] => true,
				['ι', 'ν', 'υ', 'θ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['λ', 'ι', 'δ', 'ο', 'σ', 'τ', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				['υ', 'κ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['λ', 'α', 'φ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
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
		['κ', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 253);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['ε', 'λ', 'ι', 'μ', 'ω', s @ ..] => {
					super::push(out, 1022);
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
			super::push(out, 255);
			match s {
				[] => true,
				['δ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				['μ', 'μ', 'α', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['ν', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['φ', 'α', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['υ', 'μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['τ', 'ο', 'ρ', 'ι', 'κ', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['κ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['α', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['μ', 'α', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word νωειοποθαγα.
pub(crate) fn n2408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2295);
	s.is_empty()
}

/// Words ending -σηιοποθαγα.
pub(crate) fn n2409(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2300);
			s.is_empty()
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 2301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιργα.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1055);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1789);
			match s {
				[] => true,
				['θ', 'υ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				['μ', 'υ', 'ρ', 'ι', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1790);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word σομονοργα.
pub(crate) fn n1865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1791);
	s.is_empty()
}

/// The word σονπυργα.
pub(crate) fn n1076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1057);
	s.is_empty()
}

/// Words ending -χγα.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 1058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωγα.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['ν', 'ι', s @ ..] => {
			super::push(out, 1059);
			s.is_empty()
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
			super::push(out, 279);
			match s {
				[] => true,
				['μ', 'α', 'ν', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				['υ', 'κ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['κ', 'ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['β', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['α', 'τ', s @ ..] => {
							match s {
								[] => true,
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2169);
									s.is_empty()
								}
								['ω', 'τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εα.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['θ', 'λ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['ι', s @ ..] => {
			match s {
				[] => true,
				['ζ', 'ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['α', 'κ', 'α', 'ρ', 'ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ζα.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['γ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηα.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	s.is_empty()
}

/// Words ending -θα.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['α', 'ν', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['ο', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ια.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['ι', 'ο', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1077);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				['ο', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1084);
					match s {
						[] => true,
						['λ', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['ο', 'π', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['α', 'τ', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 299);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['α', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['υ', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', 'μ', 'α', 'λ', 'ω', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κα.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['δ', 'η', 'μ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				['ρ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						['ω', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					match s {
						[] => true,
						['δ', 'υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						['η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['ι', 'τ', s @ ..] => {
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['β', 'ε', 'σ', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						['τ', 'ο', 'μ', 'υ', 'θ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						['λ', 'ι', 'θ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						['τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['η', 'μ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λα.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['β', 'α', 'σ', 'τ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				['π', 'α', 'δ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['ξ', 'α', 'ν', 'δ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['θ', 'η', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						['σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				['δ', 'ι', 'ν', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				['π', 'ο', 'ρ', 'φ', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['σ', 'κ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['τ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2171);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'ο', 'ν', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['δ', 'α', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						['ι', 'ω', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						['φ', 'υ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['π', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μα.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2173);
									s.is_empty()
								}
								['ν', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['ω', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				['χ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['ρ', 'ο', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['υ', 'σ', 'τ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2176);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'β', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				['ρ', 'ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['α', 'ν', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'η', 'σ', 'τ', 'ο', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						['γ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						['φ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['ε', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				['ρ', 'α', 'κ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['δ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				['λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				['μ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1830);
							match s {
								[] => true,
								['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2178);
									s.is_empty()
								}
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
			super::push(out, 336);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'σ', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['κ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						['ρ', 'ε', 'φ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['κ', 'υ', 'π', 'ε', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						['π', 'ο', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['φ', 'α', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['β', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['α', 'γ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['ι', 'γ', 'ν', 'ω', 'σ', 'κ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1839);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 2276);
											match s {
												[] => true,
												['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2296);
													s.is_empty()
												}
												['ω', 'ν', '\u{1}', s @ ..] => {
													super::push(out, 2297);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2277);
											s.is_empty()
										}
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
					super::push(out, 1139);
					match s {
						[] => true,
						['ο', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['μ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						['ξ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['ρ', 'ε', 'θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						['τ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['γ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1846);
							match s {
								[] => true,
								['τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', 'φ', 'η', 'ρ', 'ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['τ', 'α', 'π', 'ο', 'δ', 'ο', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['π', 'α', 'ξ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						['σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1147);
					match s {
						[] => true,
						['κ', 'ε', 'υ', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['σ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1851);
							s.is_empty()
						}
						['τ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['φ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['α', 'σ', 'σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1856);
							match s {
								[] => true,
								['γ', 'υ', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2180);
									s.is_empty()
								}
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
			super::push(out, 339);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1151);
					match s {
						[] => true,
						['μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['π', 'ι', 'φ', 'θ', 'ο', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['σ', 'κ', 'α', 'μ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['μ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['σ', 'υ', 'χ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1859);
							match s {
								[] => true,
								['κ', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1860);
							match s {
								[] => true,
								['π', 'ο', s @ ..] => {
									match s {
										[] => true,
										['λ', 'ο', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2278);
											s.is_empty()
										}
										['φ', 'α', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2279);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'π', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['κ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['σ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						['ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['ν', 'α', 'γ', 'α', 'γ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						['π', 'ο', 'κ', 'α', 'τ', 'α', 'σ', 'τ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['θ', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1868);
							match s {
								[] => true,
								['α', 'θ', 'ε', 'σ', 'τ', 'η', 'κ', 'ω', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1871);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2183);
									s.is_empty()
								}
								['ε', 'π', 'ο', 'ν', 'θ', 'ω', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2184);
									s.is_empty()
								}
								['ο', 'λ', 'ε', 'μ', 'ω', 'ν', '\u{1}', s @ ..] => super::t0::n2292(s, out),
								_ => false,
							}
						}
						['σ', s @ ..] => super::t0::n1949(s, out),
						['φ', 'ω', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1950(s, out),
						_ => false,
					}
				}
				['ο', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n1197(s, out),
				_ => false,
			}
		}
		['υ', 'π', 'ο', 'δ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => super::t0::n346(s, out),
		['ω', s @ ..] => super::t0::n347(s, out),
		_ => false,
	}
}

/// Words ending -ξα.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['ι', s @ ..] => {
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1874);
							match s {
								[] => true,
								['ο', 'γ', 'ω', 'τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1875);
							match s {
								[] => true,
								['ρ', 'ε', 'π', 'η', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						['χ', 'ρ', 'ε', 'ω', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'μ', 'φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οα.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πα.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['γ', 'ε', 'λ', 'λ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'ε', 'ι', 'β', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['ξ', 'ι', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ι', 'τ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2189);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['δ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['λ', 'η', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['ι', 'ο', 'π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'α', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['μ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['τ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
								['τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2192);
									s.is_empty()
								}
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
			super::push(out, 356);
			match s {
				[] => true,
				['η', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'ο', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['β', 'α', 'ι', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['δ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1884);
							s.is_empty()
						}
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['ε', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'χ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1888);
							match s {
								[] => true,
								['τ', 'α', 'σ', 'τ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1889);
							match s {
								[] => true,
								['ι', 'ν', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2194);
									s.is_empty()
								}
								['υ', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['ε', 'ι', 'π', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'υ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['ρ', 'ω', 'τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['λ', 'λ', 'ω', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 2280);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2281);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', 'ο', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						['ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						['ρ', 'α', 'π', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['γ', 'μ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						['κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		['υ', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρα.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['β', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['α', 'λ', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				['η', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				['ι', 'ν', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['ω', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1899);
							s.is_empty()
						}
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['δ', 'ε', 'ι', 'κ', 'ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				['θ', 'μ', 'η', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				['σ', 'τ', 'ο', 'φ', 'α', 'ν', 'ε', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['τ', 'ε', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		['ξ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['ο', 'υ', 'ρ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['α', 'γ', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['η', s @ ..] => {
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 1901);
							s.is_empty()
						}
						['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'θ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ε', 'ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'τ', 'α', 'ι', 'ν', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2197);
									s.is_empty()
								}
								['κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2198);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['τ', 'ε', 'κ', 'τ', 'ο', 'ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['β', 'ε', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['ε', 'β', 'η', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['η', 'μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['ε', 'ν', s @ ..] => {
					match s {
						[] => true,
						['ε', 'σ', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1904);
							s.is_empty()
						}
						['η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['ε', 'π', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['ο', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['α', 'σ', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['σ', 'ι', 'α', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['ρ', 'ο', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['φ', 'ο', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					match s {
						[] => true,
						['δ', 'ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1909);
							s.is_empty()
						}
						['ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', 'η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['λ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['α', 'ι', 'π', 'ω', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'ι', 'ε', 'υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['ι', 'ρ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'ν', 'α', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['λ', 'α', 'ν', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		['μ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['λ', 'μ', 'ο', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['ε', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['ο', 'π', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['ι', 'κ', 'ι', 'ζ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['ρ', 'α', 'ν', 'ν', 'ε', 'υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υα.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['ο', 'δ', 'ι', 'δ', 'α', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φα.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				['ρ', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['σ', 'τ', 'ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['α', 'ρ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1262);
					match s {
						[] => true,
						['ν', 'ο', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1913);
							s.is_empty()
						}
						['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 409);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['δ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['δ', 'ι', 'σ', 'ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χα.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['ρ', 'ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'ρ', 'ο', 'υ', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['δ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						['ν', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψα.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['ι', 'ν', 'θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['ο', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σαρδ.
pub(crate) fn n1362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	s.is_empty()
}

/// The word συμιρδ.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	s.is_empty()
}

/// Words ending -ορδ.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	s.is_empty()
}

/// Words ending -υρδ.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	s.is_empty()
}

/// Words ending -ωρδ.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	s.is_empty()
}

/// Words ending -υδ.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['δ', 'ι', 'α', 'λ', 'υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['ο', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωδ.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	s.is_empty()
}

/// Words ending -αξε.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['α', 'γ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'ρ', 'ε', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1359);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['ι', 'σ', 'μ', 'υ', 'ρ', 'ι', 'ο', 'τ', 'ε', 'τ', 'ρ', 'α', 'κ', 'ι', 'σ', 'χ', 'ι', 'λ', 'ι', 'ο', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['α', 'ν', 'ι', 'σ', s @ ..] => {
					match s {
						[] => true,
						['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						['θ', 'ε', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οξε.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	s.is_empty()
}

/// The word σονπυξε.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	s.is_empty()
}

/// Words ending -ωξε.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	s.is_empty()
}

/// Words ending -πε.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1363);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['ξ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['σ', 'σ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1364);
					match s {
						[] => true,
						['α', 'χ', 'ω', 'ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 538);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1366);
					match s {
						[] => true,
						['ι', 'γ', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['ι', 'ν', 'η', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['α', 'φ', 'ε', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['ι', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', 'ο', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1370);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['υ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				['π', 'λ', 'ε', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				['ρ', 'ρ', 'ε', 'π', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['κ', 'ε', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						['π', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						['υ', 'ν', 'θ', 'ε', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ρ', 'ε', 'ψ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1377);
					match s {
						[] => true,
						['α', 'ν', 'ε', 'σ', 'τ', 'α', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1378);
					match s {
						[] => true,
						['θ', 'ο', 'ν', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['α', 'μ', 'ε', 'ρ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρε.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['α', 'σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 545);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['σ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'υ', 'λ', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 548);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['τ', 'ι', 'κ', 'ω', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σε.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['ι', 'β', 'α', 'σ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τε.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 554);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υε.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['α', 'φ', 'η', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['ι', 'α', 'λ', 'υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'χ', 'θ', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 558);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['α', 'τ', 'ο', 'ι', 'κ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				['ι', 'ν', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['υ', 'κ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['ι', 'π', 'ι', 'δ', 'ε', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1391);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['π', 'ρ', 'ω', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 565);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'σ', 'τ', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φε.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['λ', 'κ', 'υ', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['α', 'ρ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['α', 'λ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χε.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψε.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['η', 'φ', 'ι', 'σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ατακ.
pub(crate) fn n1506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1462);
	match s {
		[] => true,
		['β', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1992);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['λ', 'ω', 'τ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1994);
			s.is_empty()
		}
		['ι', 'ρ', 'ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 1995);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1996);
			match s {
				[] => true,
				['ε', 'χ', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['α', 'μ', 'β', 'α', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', 'ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 1998);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['ε', 'λ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			match s {
				[] => true,
				['κ', 'ε', 'υ', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			match s {
				[] => true,
				['α', 'ρ', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['α', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2287);
							s.is_empty()
						}
						['ο', 'ν', 'η', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εκ.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['α', 'λ', 'υ', 'μ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2001);
							match s {
								[] => true,
								['ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'υ', 'ρ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', 'α', 'λ', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 650);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['ρ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['σ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2227);
									s.is_empty()
								}
								['τ', 'ω', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2228);
									s.is_empty()
								}
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

/// Words ending -ρηκ.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1470);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικ.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['β', 'δ', 'η', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		['δ', 'α', 'φ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 653);
			s.is_empty()
		}
		['κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 654);
			s.is_empty()
		}
		['λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 655);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['δ', 'υ', 'ν', 'ε', 'υ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1474);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['β', 'α', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2229);
									s.is_empty()
								}
								['μ', 'ω', 'μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['σ', 'σ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['τ', 'ρ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λκ.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['ι', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['μ', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['μ', 'α', 'κ', 'τ', 'η', 'ρ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νκ.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	s.is_empty()
}

/// Words ending -οκ.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['κ', 'υ', 'γ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['ο', 'κ', 'υ', 'ν', 'θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['α', 'κ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['ι', 'ν', 'θ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκ.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['ι', 'π', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['α', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ε', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['θ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['κ', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				['ν', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2005);
							s.is_empty()
						}
						['ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['σ', 'τ', 'α', 'λ', 'λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τκ.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['η', 'σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υκ.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 675);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['ι', 'σ', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['ι', 'σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['λ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['ι', 'ν', 'δ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['α', 'ι', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['ν', 'ο', 'π', 'ρ', 'ι', 'σ', 'τ', 'ο', 'κ', 'α', 'ρ', 'δ', 'α', 'μ', 'ο', 'γ', 'λ', 'υ', 'φ', s @ ..] => {
							match s {
								[] => true,
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2231);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2232);
									s.is_empty()
								}
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
			super::push(out, 681);
			match s {
				[] => true,
				['θ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['α', 'ρ', 'ι', 'σ', 'σ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['ι', 'α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				['τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωκ.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['λ', 'υ', 'τ', 'η', 'ρ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word σοιδαπμαλ.
pub(crate) fn n2412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2298);
	s.is_empty()
}

/// Words ending -ρυμ.
pub(crate) fn n759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2036);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['φ', 'θ', 'α', 'λ', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['β', 'α', 'λ', 'α', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ι', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πεπ.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['ν', 'υ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρεπ.
pub(crate) fn n834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['γ', 'α', 'μ', 'η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1606);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['ι', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2074);
					match s {
						[] => true,
						['α', 'τ', 'η', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					match s {
						[] => true,
						['κ', 'ο', 'π', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						['τ', 'α', 'λ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2075);
					match s {
						[] => true,
						['υ', 'χ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					match s {
						[] => true,
						['ε', 'ρ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						['ρ', 'ο', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεπ.
pub(crate) fn n835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	match s {
		[] => true,
		['ρ', 'ο', 'κ', 'α', 'τ', 'ο', 'ι', 'κ', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φεπ.
pub(crate) fn n837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['ρ', 'α', 'γ', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1610);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηπ.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	s.is_empty()
}

/// Words ending -ιπ.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 827);
			s.is_empty()
		}
		['λ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['υ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'τ', 'ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				['σ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1613);
					s.is_empty()
				}
				['τ', 'ε', 'υ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'υ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		['ω', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λπ.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['α', 'ν', 'ι', 'σ', 'τ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						['υ', s @ ..] => {
							match s {
								[] => true,
								['π', 'ο', 'ρ', 'φ', 'υ', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2253);
									s.is_empty()
								}
								['φ', 'υ', 'λ', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
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
			super::push(out, 835);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['υ', 'σ', 'ι', 'ω', s @ ..] => {
					super::push(out, 1616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νπ.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['γ', 'η', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1617);
					s.is_empty()
				}
				['κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οπ.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 842);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['ι', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2079);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['δ', 'ε', 'ι', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						['π', 'ο', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2084);
							match s {
								[] => true,
								['α', 'γ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', 'ρ', 'ο', 'ν', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['φ', 'υ', 'ρ', 'ο', s @ ..] => {
					match s {
						[] => true,
						['γ', 'ε', 'ν', 'ν', 'η', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						['ζ', 'ω', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['α', 'π', 'λ', 'α', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρπ.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['κ', 'τ', 'ε', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				['ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2257);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['ι', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['α', 'γ', 'ο', 'ρ', 'ε', 'υ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['α', 'λ', 'λ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1633);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['τ', 'α', 'λ', 'α', 'μ', 'β', 'α', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['α', 'ρ', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2093);
							match s {
								[] => true,
								['γ', 'ω', s @ ..] => {
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2292);
											s.is_empty()
										}
										['ν', '\u{1}', s @ ..] => {
											super::push(out, 2293);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2094);
							match s {
								[] => true,
								['α', 'θ', 'ε', 'ζ', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['α', 'ρ', s @ ..] => {
									super::push(out, 2261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['υ', 'γ', 'χ', 'α', 'ν', 'ω', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 2262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['υ', 'λ', 'α', 'κ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τπ.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['ε', 'ρ', 'υ', 'γ', 'ο', 'ε', 'ι', 'δ', 'η', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υπ.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 855);
			s.is_empty()
		}
		['ν', 'θ', 'α', 'ν', 'ο', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 856);
			s.is_empty()
		}
		['ξ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 857);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['η', 'ν', 'α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1637);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1638);
					match s {
						[] => true,
						['φ', 'λ', 'ε', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
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

/// Words ending -ωπ.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	s.is_empty()
}

/// The word σονιεππυτσ.
pub(crate) fn n2376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2265);
	s.is_empty()
}

/// The word σονιππυτσ.
pub(crate) fn n2377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2266);
	s.is_empty()
}

/// The word σοκιτπυτσ.
pub(crate) fn n2206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	s.is_empty()
}

/// The word σονικαρυτσ.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1676);
	s.is_empty()
}

/// Words ending -ωτσ.
pub(crate) fn n912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 899);
	s.is_empty()
}

/// Words ending -υσ.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['κ', s @ ..] => {
					match s {
						[] => true,
						['α', 'τ', 'α', 'θ', 'ε', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						['ε', 'ι', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['ρ', 'ο', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 901);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['α', 'μ', 'ι', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['ο', 'φ', 'α', 'ν', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['η', 'σ', 'ω', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['α', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['β', 'ο', 'λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['α', 'χ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2109);
							s.is_empty()
						}
						['ε', 'τ', 'ρ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['ρ', 'α', 'κ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['ω', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['γ', 'α', 'γ', 'ω', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						['φ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['ν', 'α', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['α', 'κ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						['ε', 'λ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['α', 'κ', 'ο', 'σ', 'ι', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 907);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φσ.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 908);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['ν', 'δ', 'α', 'μ', 'ν', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['ι', 'γ', 'κ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 912);
			s.is_empty()
		}
		['υ', 'γ', 'μ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		['ω', 'ι', 'τ', 'ε', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χσ.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['σ', 'α', 'μ', 'ε', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['σ', 'τ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['λ', 'α', 'σ', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσ.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['κ', 'ρ', 'α', 'τ', 'ι', 'κ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 918);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 919);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 920);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ηνεμοχραπυ.
pub(crate) fn n2413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2299);
	s.is_empty()
}

/// Words ending -ονεμοχραπυ.
pub(crate) fn n2414(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ν', '\u{1}', s @ ..] => {
			super::push(out, 2302);
			s.is_empty()
		}
		['σ', '\u{1}', s @ ..] => {
			super::push(out, 2303);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word νωμελοπιτνα.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2185);
	s.is_empty()
}

/// Words ending -σιτνα.
pub(crate) fn n1949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1872);
	match s {
		[] => true,
		['τ', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2186);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word σονωφιτνα.
pub(crate) fn n1950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1873);
	s.is_empty()
}

/// The word σοκιοτνα.
pub(crate) fn n1197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1168);
	s.is_empty()
}

/// The word σοτηδοπυνα.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	s.is_empty()
}

/// Words ending -ωνα.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['μ', 'α', 'λ', 'ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1169);
			s.is_empty()
		}
		_ => false,
	}
}

