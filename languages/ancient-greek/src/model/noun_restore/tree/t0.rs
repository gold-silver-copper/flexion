use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => super::t0::n1(s, out),
		['ε', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['η', s @ ..] => super::t0::n3(s, out),
		['ι', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 369);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 370);
							match s {
								[] => true,
								['γ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2754);
											match s {
												[] => true,
												['ρ', 'α', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 4542);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2755);
											match s {
												[] => true,
												['λ', 'α', 'κ', '\u{1}', s @ ..] => {
													super::push(out, 4543);
													s.is_empty()
												}
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
							super::push(out, 371);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1220);
									match s {
										[] => true,
										['ι', 'μ', 'ρ', 'υ', 'κ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2756);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 1221);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1222);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1223);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2757);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 1224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 372);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1225);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 1226);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 373);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 374);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 375);
							match s {
								[] => true,
								['ρ', 'α', 'τ', 'σ', 'α', 'β', '\u{1}', s @ ..] => {
									super::push(out, 1230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 376);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 377);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 378);
							match s {
								[] => true,
								['α', s @ ..] => super::t0::n1256(s, out),
								['ε', s @ ..] => {
									super::push(out, 1232);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2761);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => super::t0::n1258(s, out),
								['ι', s @ ..] => super::t0::n1259(s, out),
								['υ', s @ ..] => super::t0::n1260(s, out),
								_ => false,
							}
						}
						['ψ', s @ ..] => super::t0::n380(s, out),
						_ => false,
					}
				}
				['β', s @ ..] => super::t0::n54(s, out),
				['ε', s @ ..] => super::t0::n55(s, out),
				['κ', s @ ..] => super::t0::n56(s, out),
				['λ', s @ ..] => super::t0::n57(s, out),
				['μ', s @ ..] => super::t0::n58(s, out),
				['ν', s @ ..] => super::t0::n59(s, out),
				['ο', s @ ..] => super::t0::n60(s, out),
				['π', s @ ..] => super::t0::n61(s, out),
				['ρ', s @ ..] => super::t0::n62(s, out),
				['σ', s @ ..] => super::t0::n63(s, out),
				['φ', s @ ..] => super::t0::n64(s, out),
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n5(s, out),
		['ξ', s @ ..] => super::t0::n6(s, out),
		['ρ', s @ ..] => super::t0::n7(s, out),
		['σ', s @ ..] => super::t0::n8(s, out),
		['υ', s @ ..] => super::t0::n9(s, out),
		['ψ', s @ ..] => super::t0::n10(s, out),
		['ω', s @ ..] => super::t0::n11(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 13);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 111);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 114);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 662);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2122);
									s.is_empty()
								}
								['φ', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 118);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 119);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 664);
							match s {
								[] => true,
								['γ', '\u{1}', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 121);
					match s {
						[] => true,
						['η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 122);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 666);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 667);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 668);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 669);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 670);
							match s {
								[] => true,
								['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['α', 'ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 672);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 674);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 125);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 676);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 677);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 126);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 127);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 128);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 17);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 130);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 131);
					match s {
						[] => true,
						['α', 'κ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 132);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								['ν', '\u{1}', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n19(s, out),
		['κ', s @ ..] => {
			super::push(out, 20);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 155);
					match s {
						[] => true,
						['ι', 'χ', 'ι', 'ν', 'ε', 'μ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 156);
					match s {
						[] => true,
						['τ', 'ι', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 21);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 157);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 832);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 833);
							match s {
								[] => true,
								['υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 158);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 159);
					match s {
						[] => true,
						['σ', 'υ', s @ ..] => {
							super::push(out, 835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 160);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 161);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 836);
							s.is_empty()
						}
						['ε', s @ ..] => super::t0::n844(s, out),
						['ι', s @ ..] => super::t0::n845(s, out),
						['ο', s @ ..] => super::t0::n846(s, out),
						['υ', s @ ..] => super::t0::n847(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n162(s, out),
				['τ', s @ ..] => super::t0::n163(s, out),
				['υ', s @ ..] => super::t0::n164(s, out),
				['χ', s @ ..] => super::t0::n165(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n22(s, out),
		['ν', s @ ..] => super::t0::n23(s, out),
		['ξ', s @ ..] => super::t0::n24(s, out),
		['ο', s @ ..] => super::t0::n25(s, out),
		['π', s @ ..] => super::t0::n26(s, out),
		['ρ', s @ ..] => super::t0::n27(s, out),
		['σ', s @ ..] => super::t0::n28(s, out),
		['τ', s @ ..] => super::t0::n29(s, out),
		['υ', s @ ..] => super::t0::n30(s, out),
		['χ', s @ ..] => super::t0::n31(s, out),
		['ψ', s @ ..] => super::t0::n32(s, out),
		['ω', s @ ..] => super::t0::n33(s, out),
		_ => false,
	}
}

/// Words ending -η.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1002);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1003);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1004);
							match s {
								[] => true,
								['τ', 'σ', 'ο', s @ ..] => {
									super::push(out, 2622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 234);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1006);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1007);
							match s {
								[] => true,
								['τ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 237);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1010);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2625);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2626);
									match s {
										[] => true,
										['κ', 'ο', s @ ..] => {
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 5789);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 5790);
													s.is_empty()
												}
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
							super::push(out, 1011);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1012);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 238);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 239);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 242);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1013);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2628);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2629);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['π', 'υ', s @ ..] => {
													super::push(out, 5791);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 5792);
													s.is_empty()
												}
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
							super::push(out, 1014);
							match s {
								[] => true,
								['ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1015);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2631);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 245);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2633);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2634);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 2635);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['ο', 'μ', s @ ..] => {
							super::push(out, 1018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 250);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1019);
							s.is_empty()
						}
						['λ', 'η', s @ ..] => {
							super::push(out, 1020);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 251);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1022);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4481);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 4482);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 4483);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 4484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1023);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1024);
							s.is_empty()
						}
						['ῥ', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['ι', s @ ..] => super::t0::n254(s, out),
				['υ', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['π', s @ ..] => super::t0::n1044(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n37(s, out),
		['θ', s @ ..] => super::t0::n38(s, out),
		['ι', s @ ..] => super::t0::n39(s, out),
		['κ', s @ ..] => super::t0::n40(s, out),
		['λ', s @ ..] => super::t0::n41(s, out),
		['μ', s @ ..] => super::t0::n42(s, out),
		['ν', s @ ..] => super::t0::n43(s, out),
		['ο', s @ ..] => super::t0::n44(s, out),
		['π', s @ ..] => super::t0::n45(s, out),
		['ρ', s @ ..] => super::t0::n46(s, out),
		['σ', s @ ..] => super::t0::n47(s, out),
		['τ', s @ ..] => super::t0::n48(s, out),
		['υ', s @ ..] => super::t0::n49(s, out),
		['φ', s @ ..] => super::t0::n50(s, out),
		['χ', s @ ..] => super::t0::n51(s, out),
		['ω', s @ ..] => super::t0::n52(s, out),
		_ => false,
	}
}

/// Words ending -αται.
pub(crate) fn n1256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2758);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2759);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηται.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1233);
	s.is_empty()
}

/// Words ending -ιται.
pub(crate) fn n1259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1234);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2762);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2763);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2764);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υται.
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1235);
	s.is_empty()
}

/// Words ending -ψαι.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	s.is_empty()
}

/// Words ending -βι.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// Words ending -ει.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	s.is_empty()
}

/// Words ending -κι.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	s.is_empty()
}

/// Words ending -λι.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['ε', 'μ', 'ο', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μι.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['ι', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νι.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	s.is_empty()
}

/// Words ending -οι.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['ν', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['ρ', 'υ', 'ο', 'ι', 'μ', 'η', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2767);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['α', 'ι', 'θ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2770);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4546);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4547);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 4548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2771);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2772);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 2774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2775);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2776);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['ε', 'ρ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2781);
							s.is_empty()
						}
						['μ', '\u{1}', s @ ..] => {
							super::push(out, 2782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['λ', 'ο', 'β', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2785);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2787);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['α', 'ζ', 'ι', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['ο', 'ι', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πι.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	s.is_empty()
}

/// Words ending -ρι.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 398);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['π', 'ε', 'π', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2791);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σι.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	s.is_empty()
}

/// Words ending -φι.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	s.is_empty()
}

/// Words ending -ν.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['φ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['α', 'τ', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 409);
					match s {
						[] => true,
						['λ', 'α', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 410);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1272);
							match s {
								[] => true,
								['λ', '\u{1}', s @ ..] => {
									super::push(out, 2793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['α', 'μ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['χ', 'ω', 'λ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n69(s, out),
		['υ', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['ο', 'ν', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 442);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1415);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1416);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 444);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3116);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3117);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 3118);
									match s {
										[] => true,
										['φ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 4882);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1420);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3119);
									match s {
										[] => true,
										['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 4883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => super::t0::n3187(s, out),
								_ => false,
							}
						}
						['ω', s @ ..] => super::t0::n1447(s, out),
						_ => false,
					}
				}
				['δ', s @ ..] => super::t0::n446(s, out),
				['ε', s @ ..] => super::t0::n447(s, out),
				['ζ', s @ ..] => super::t0::n448(s, out),
				['θ', s @ ..] => super::t0::n449(s, out),
				['ι', s @ ..] => super::t0::n450(s, out),
				['κ', s @ ..] => super::t0::n451(s, out),
				['λ', s @ ..] => super::t0::n452(s, out),
				['μ', s @ ..] => super::t0::n453(s, out),
				['ν', s @ ..] => super::t0::n454(s, out),
				['ξ', s @ ..] => super::t0::n455(s, out),
				['π', s @ ..] => super::t0::n456(s, out),
				['ρ', s @ ..] => super::t0::n457(s, out),
				['σ', s @ ..] => super::t0::n458(s, out),
				['τ', s @ ..] => super::t0::n459(s, out),
				['υ', s @ ..] => super::t0::n460(s, out),
				['φ', s @ ..] => super::t0::n461(s, out),
				['χ', s @ ..] => super::t0::n462(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 462);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3157);
									s.is_empty()
								}
								['β', '\u{1}', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1490);
							match s {
								[] => true,
								['μ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3160);
									s.is_empty()
								}
								['ρ', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 3161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1494);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', s @ ..] => super::t0::n3231(s, out),
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 468);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3163);
									s.is_empty()
								}
								['π', 'ο', 'π', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 3164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1499);
							match s {
								[] => true,
								['ρ', 'θ', s @ ..] => {
									super::push(out, 3165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => super::t0::n1526(s, out),
						['ρ', s @ ..] => super::t0::n1527(s, out),
						['ω', s @ ..] => super::t0::n1528(s, out),
						_ => false,
					}
				}
				['ο', s @ ..] => super::t0::n470(s, out),
				['π', s @ ..] => super::t0::n471(s, out),
				['ρ', s @ ..] => super::t0::n472(s, out),
				['σ', s @ ..] => super::t0::n473(s, out),
				['τ', s @ ..] => super::t0::n474(s, out),
				['υ', s @ ..] => super::t0::n475(s, out),
				['φ', s @ ..] => super::t0::n476(s, out),
				['ῥ', s @ ..] => super::t0::n477(s, out),
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n73(s, out),
		['ε', s @ ..] => super::t0::n74(s, out),
		['η', s @ ..] => super::t0::n75(s, out),
		['ι', s @ ..] => super::t0::n76(s, out),
		['ο', s @ ..] => super::t0::n77(s, out),
		['ρ', s @ ..] => super::t0::n78(s, out),
		['υ', s @ ..] => super::t0::n79(s, out),
		['ω', s @ ..] => super::t0::n80(s, out),
		_ => false,
	}
}

/// Words ending -ρ.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 506);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1562);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 510);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['υ', '\u{1}', s @ ..] => {
							super::push(out, 1566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1568);
							match s {
								[] => true,
								['α', 'π', 'ο', s @ ..] => {
									super::push(out, 3189);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1570);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1572);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3193);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 4895);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1574);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 3196);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3198);
									match s {
										[] => true,
										['ψ', s @ ..] => {
											super::push(out, 4896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1575);
							match s {
								[] => true,
								['α', 'μ', 'ρ', 'ε', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 3199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1576);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1577);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3202);
									match s {
										[] => true,
										['ν', 'α', 'δ', 'ο', 'π', '\u{1}', s @ ..] => {
											super::push(out, 4897);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1579);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4898);
											s.is_empty()
										}
										['γ', s @ ..] => {
											super::push(out, 4899);
											s.is_empty()
										}
										['μ', 'ρ', s @ ..] => {
											super::push(out, 4900);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 4901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3207);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4902);
											s.is_empty()
										}
										['η', s @ ..] => {
											super::push(out, 4903);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 4904);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 4905);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 3208);
									match s {
										[] => true,
										['λ', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 4906);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => super::t0::n1607(s, out),
						['ω', s @ ..] => super::t0::n1608(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n83(s, out),
		['υ', s @ ..] => super::t0::n84(s, out),
		['ω', s @ ..] => super::t0::n85(s, out),
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1590);
							match s {
								[] => true,
								['λ', 'α', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1595);
							match s {
								[] => true,
								['τ', 'τ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								['ῥ', '\u{1}', s @ ..] => {
									super::push(out, 3225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						['υ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['α', 'ι', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['ρ', 'κ', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['ι', s @ ..] => super::t0::n533(s, out),
				['κ', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1620);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1621);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3259);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1622);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1624);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1625);
							match s {
								[] => true,
								['κ', s @ ..] => super::t0::n3334(s, out),
								['π', s @ ..] => super::t0::n3335(s, out),
								_ => false,
							}
						}
						['λ', s @ ..] => super::t0::n1654(s, out),
						['ο', s @ ..] => super::t0::n1655(s, out),
						['υ', s @ ..] => super::t0::n1656(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t0::n536(s, out),
				['ν', s @ ..] => super::t0::n537(s, out),
				['ο', s @ ..] => super::t0::n538(s, out),
				['π', s @ ..] => super::t0::n539(s, out),
				['ρ', s @ ..] => super::t0::n540(s, out),
				['σ', s @ ..] => super::t0::n541(s, out),
				['τ', s @ ..] => super::t0::n542(s, out),
				['υ', s @ ..] => super::t0::n543(s, out),
				['φ', s @ ..] => super::t0::n544(s, out),
				['χ', s @ ..] => super::t0::n545(s, out),
				['ψ', s @ ..] => super::t0::n546(s, out),
				['ω', s @ ..] => super::t0::n547(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n87(s, out),
		['η', s @ ..] => super::t0::n88(s, out),
		['ι', s @ ..] => super::t0::n89(s, out),
		['λ', s @ ..] => super::t0::n90(s, out),
		['ν', s @ ..] => super::t0::n91(s, out),
		['ο', s @ ..] => super::t0::n92(s, out),
		['υ', s @ ..] => super::t0::n93(s, out),
		['ω', s @ ..] => super::t0::n94(s, out),
		_ => false,
	}
}

/// Words ending -υ.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['κ', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψ.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['λ', 'β', 'ω', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 103);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['κ', 'γ', 'ι', 'ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['ρ', 'α', s @ ..] => {
							super::push(out, 2116);
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
							super::push(out, 2117);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2120);
							match s {
								[] => true,
								['γ', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 4223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ω.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 108);
			s.is_empty()
		}
		['τ', 'τ', s @ ..] => {
			super::push(out, 109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ια.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2137);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2138);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2140);
							match s {
								[] => true,
								['φ', 'ε', 'κ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2141);
							match s {
								[] => true,
								['μ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5681);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 5682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['ε', 'τ', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['μ', 'υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['π', 'υ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 4226);
									s.is_empty()
								}
								['ρ', 'ο', 'μ', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2147);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2148);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2150);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2151);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4228);
									match s {
										[] => true,
										['ε', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 5683);
											s.is_empty()
										}
										['ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 5684);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 4229);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 5685);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 4230);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5686);
											s.is_empty()
										}
										['θ', s @ ..] => {
											super::push(out, 5687);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6326);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 6327);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 5688);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6328);
													match s {
														[] => true,
														['τ', 'α', 'μ', '\u{1}', s @ ..] => super::t0::n6719(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 6329);
													s.is_empty()
												}
												['τ', s @ ..] => super::t0::n6510(s, out),
												_ => false,
											}
										}
										['λ', s @ ..] => super::t0::n5841(s, out),
										['μ', s @ ..] => super::t0::n5842(s, out),
										['ν', s @ ..] => super::t0::n5843(s, out),
										['ξ', s @ ..] => super::t0::n5844(s, out),
										['π', s @ ..] => super::t0::n5845(s, out),
										['ρ', s @ ..] => super::t0::n5846(s, out),
										['τ', s @ ..] => super::t0::n5847(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n701(s, out),
				['υ', s @ ..] => super::t0::n702(s, out),
				['ω', s @ ..] => super::t0::n703(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n137(s, out),
		['ε', s @ ..] => super::t0::n138(s, out),
		['θ', s @ ..] => super::t0::n139(s, out),
		['ι', s @ ..] => super::t0::n140(s, out),
		['κ', s @ ..] => super::t0::n141(s, out),
		['λ', s @ ..] => super::t0::n142(s, out),
		['μ', s @ ..] => super::t0::n143(s, out),
		['ν', s @ ..] => super::t0::n144(s, out),
		['ξ', s @ ..] => super::t0::n145(s, out),
		['ο', s @ ..] => super::t0::n146(s, out),
		['π', s @ ..] => super::t0::n147(s, out),
		['ρ', s @ ..] => super::t0::n148(s, out),
		['σ', s @ ..] => super::t0::n149(s, out),
		['τ', s @ ..] => super::t0::n150(s, out),
		['υ', s @ ..] => super::t0::n151(s, out),
		['φ', s @ ..] => super::t0::n152(s, out),
		['χ', s @ ..] => super::t0::n153(s, out),
		['ψ', s @ ..] => super::t0::n154(s, out),
		_ => false,
	}
}

/// Words ending -ελλα.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 837);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2397);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιλλα.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 838);
	match s {
		[] => true,
		['μ', 'α', s @ ..] => {
			super::push(out, 2399);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολλα.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 839);
	match s {
		[] => true,
		['κ', 'ο', s @ ..] => {
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4394);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλλα.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 840);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2400);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 2401);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 2402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πλα.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	s.is_empty()
}

/// Words ending -τλα.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	s.is_empty()
}

/// Words ending -υλα.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	s.is_empty()
}

/// Words ending -χλα.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	s.is_empty()
}

/// Words ending -μα.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 2403);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2405);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 4397);
									match s {
										[] => true,
										['α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 5764);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2406);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2408);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4398);
									match s {
										[] => true,
										['χ', '\u{1}', s @ ..] => {
											super::push(out, 5765);
											s.is_empty()
										}
										_ => false,
									}
								}
								['δ', s @ ..] => {
									super::push(out, 4399);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4400);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 4401);
									match s {
										[] => true,
										['α', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 5766);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2409);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								['σ', 'α', s @ ..] => {
									super::push(out, 4403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2411);
							match s {
								[] => true,
								['ο', 'ι', 'ρ', 'η', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2412);
							match s {
								[] => true,
								['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2413);
							match s {
								[] => true,
								['δ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2414);
							match s {
								[] => true,
								['υ', 'ρ', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2415);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 847);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2416);
							match s {
								[] => true,
								['η', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 4410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2417);
							s.is_empty()
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
				['ζ', 'ο', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['θ', 'α', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2418);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2419);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2420);
							match s {
								[] => true,
								['υ', 'ο', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 851);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2421);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2423);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2424);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4412);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4413);
									match s {
										[] => true,
										['τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 5767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2427);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => super::t0::n2470(s, out),
						['κ', s @ ..] => super::t0::n2471(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t0::n863(s, out),
				['ν', s @ ..] => super::t0::n864(s, out),
				['ο', s @ ..] => super::t0::n865(s, out),
				['π', s @ ..] => super::t0::n866(s, out),
				['ρ', s @ ..] => super::t0::n867(s, out),
				['σ', s @ ..] => super::t0::n868(s, out),
				['τ', s @ ..] => super::t0::n869(s, out),
				['υ', s @ ..] => super::t0::n870(s, out),
				['φ', s @ ..] => super::t0::n871(s, out),
				['χ', s @ ..] => super::t0::n872(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n170(s, out),
		['λ', s @ ..] => super::t0::n171(s, out),
		['μ', s @ ..] => super::t0::n172(s, out),
		['ο', s @ ..] => super::t0::n173(s, out),
		['ρ', s @ ..] => super::t0::n174(s, out),
		['σ', s @ ..] => super::t1::n175(s, out),
		['υ', s @ ..] => super::t1::n176(s, out),
		['χ', s @ ..] => super::t1::n177(s, out),
		['ω', s @ ..] => super::t1::n178(s, out),
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 180);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['ι', 'χ', 'α', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['β', 'υ', 'λ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5784);
											s.is_empty()
										}
										['μ', s @ ..] => {
											super::push(out, 5785);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 4455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2530);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4456);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5786);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2532);
							match s {
								[] => true,
								['α', 'ρ', 'ε', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 4458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2533);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4459);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 4460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['θ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξα.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['α', 'μ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2540);
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
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οα.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['π', '\u{1}', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['ῥ', '\u{1}', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πα.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρα.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4461);
									s.is_empty()
								}
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 4462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2543);
							match s {
								[] => true,
								['κ', 'ο', 'ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2545);
							match s {
								[] => true,
								['λ', 'α', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 4464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2547);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['υ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2551);
							match s {
								[] => true,
								['μ', 'α', 'λ', 'α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 4466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2553);
							s.is_empty()
						}
						['ψ', 'ε', 'λ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['υ', '\u{1}', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2556);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2557);
							match s {
								[] => true,
								['ι', 'ρ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5787);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 5788);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4467);
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['ο', 'κ', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4468);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 4469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['μ', 'ε', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['ρ', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2569);
							match s {
								[] => true,
								['α', 'μ', 'ο', 'φ', 'ι', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 4470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						['ζ', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2572);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						['τ', s @ ..] => super::t1::n2622(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['ο', s @ ..] => super::t1::n203(s, out),
		['π', s @ ..] => super::t1::n204(s, out),
		['ρ', s @ ..] => super::t1::n205(s, out),
		['τ', s @ ..] => super::t1::n206(s, out),
		['υ', s @ ..] => super::t1::n207(s, out),
		['φ', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['χ', s @ ..] => super::t1::n209(s, out),
		['ω', s @ ..] => super::t1::n210(s, out),
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['λ', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['α', 'ι', 'δ', 'ο', 'ρ', 'β', 'α', '\u{1}', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2618);
							s.is_empty()
						}
						['λ', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 2620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υα.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['θ', 'χ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['μ', '\u{1}', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['ξ', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['ο', 'β', '\u{1}', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4479);
									s.is_empty()
								}
								['μ', '\u{1}', s @ ..] => {
									super::push(out, 4480);
									s.is_empty()
								}
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

/// Words ending -χα.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	s.is_empty()
}

/// Words ending -ψα.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωα.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιδη.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['υ', 'κ', 'υ', 'λ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 2637);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πωδη.
pub(crate) fn n1044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1031);
	s.is_empty()
}

/// Words ending -εη.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['λ', 'α', 'γ', 'υ', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θη.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['λ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1032);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['π', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['ι', 'μ', 'α', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιη.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['η', 'τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		['λ', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 264);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 265);
			s.is_empty()
		}
		['τ', 'α', 'χ', 'σ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κη.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1039);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1040);
					match s {
						[] => true,
						['τ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1041);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1042);
					match s {
						[] => true,
						['υ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1043);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2640);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2641);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['α', 'ν', 'α', 'τ', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['θ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4485);
									match s {
										[] => true,
										['δ', s @ ..] => {
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 6363);
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 6364);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 4486);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 4487);
									match s {
										[] => true,
										['α', 'κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 5793);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2644);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2645);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4488);
									match s {
										[] => true,
										['ρ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 5794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4489);
									match s {
										[] => true,
										['σ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 5795);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4490);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 5796);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 5797);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 5798);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 5799);
											s.is_empty()
										}
										_ => false,
									}
								}
								['κ', s @ ..] => super::t1::n4601(s, out),
								['λ', s @ ..] => super::t1::n4602(s, out),
								['μ', s @ ..] => super::t1::n4603(s, out),
								['ν', s @ ..] => super::t1::n4604(s, out),
								['π', s @ ..] => super::t1::n4605(s, out),
								['ρ', s @ ..] => super::t1::n4606(s, out),
								['τ', s @ ..] => super::t1::n4607(s, out),
								['φ', s @ ..] => super::t1::n4608(s, out),
								_ => false,
							}
						}
						['υ', s @ ..] => super::t1::n2697(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n270(s, out),
		['κ', s @ ..] => super::t1::n271(s, out),
		['λ', s @ ..] => super::t1::n272(s, out),
		['ο', s @ ..] => super::t1::n273(s, out),
		['ρ', s @ ..] => super::t1::n274(s, out),
		['σ', s @ ..] => super::t1::n275(s, out),
		['υ', s @ ..] => super::t1::n276(s, out),
		['ω', s @ ..] => super::t1::n277(s, out),
		_ => false,
	}
}

/// Words ending -λη.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['ν', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4507);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 4508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4509);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 4510);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 4511);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 4512);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5818);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 5819);
											match s {
												[] => true,
												['μ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
													super::push(out, 6373);
													s.is_empty()
												}
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
							super::push(out, 2675);
							match s {
								[] => true,
								['ε', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2676);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4514);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4515);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 4516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['α', 'ρ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['ο', 'ρ', 'δ', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['β', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4517);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 4518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						['ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['α', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μη.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['ι', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['τ', 'σ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['α', 'ξ', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['α', 'χ', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['ο', 'φ', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['ν', 'γ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νη.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['ρ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2689);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 4519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2690);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 4520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2693);
							match s {
								[] => true,
								['β', 'ο', 'ρ', 'ε', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 4521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2694);
							match s {
								[] => true,
								['δ', 'ν', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 4522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['η', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2696);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2697);
							match s {
								[] => true,
								['α', 'ρ', 'γ', 'ε', 'γ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 4523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 2698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2700);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2703);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['α', 'λ', 'ι', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						['ο', 'β', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2708);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2709);
							match s {
								[] => true,
								['υ', 'μ', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5820);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 5821);
											s.is_empty()
										}
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
					super::push(out, 1145);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['ι', 'λ', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['μ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['ο', 'κ', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1158);
					match s {
						[] => true,
						['ο', s @ ..] => super::t1::n2772(s, out),
						['ω', s @ ..] => {
							super::push(out, 2719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => super::t1::n316(s, out),
		['χ', s @ ..] => super::t1::n317(s, out),
		['ω', s @ ..] => super::t1::n318(s, out),
		_ => false,
	}
}

/// Words ending -οη.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['α', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['λ', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πη.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['ι', 'σ', 'υ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2729);
							match s {
								[] => true,
								['π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4531);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4532);
									s.is_empty()
								}
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
			super::push(out, 330);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['λ', 'ο', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρη.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ση.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τη.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
				['π', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['ο', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 341);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2735);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						['λ', s @ ..] => {
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

/// Words ending -υη.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['δ', '\u{1}', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['κ', 'σ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2738);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φη.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1200);
					match s {
						[] => true,
						['σ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4533);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5827);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 5828);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 5829);
											s.is_empty()
										}
										_ => false,
									}
								}
								['γ', s @ ..] => {
									super::push(out, 4534);
									match s {
										[] => true,
										['υ', 'ξ', '\u{1}', s @ ..] => {
											super::push(out, 5830);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 4535);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4536);
									s.is_empty()
								}
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
			super::push(out, 352);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['ο', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 354);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['υ', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4537);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 4538);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5831);
											match s {
												[] => true,
												['ν', 'α', 'ν', s @ ..] => {
													super::push(out, 6378);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 6379);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 5832);
											match s {
												[] => true,
												['π', 'ε', '\u{1}', s @ ..] => {
													super::push(out, 6380);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 5833);
											s.is_empty()
										}
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
			super::push(out, 357);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χη.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['α', 'μ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4539);
									s.is_empty()
								}
								['ο', 'ρ', 'δ', 'ν', 'ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 4540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['α', 'δ', 'ν', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['α', 'ν', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['λ', 'ι', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2746);
							match s {
								[] => true,
								['ε', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 4541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2751);
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
		['σ', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2752);
							s.is_empty()
						}
						['σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωη.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	s.is_empty()
}

/// Words ending -ον.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4549);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 4550);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4551);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4552);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 4553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1282);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2795);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5834);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 5835);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 4554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2796);
							s.is_empty()
						}
						['ῥ', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2798);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4555);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4556);
									s.is_empty()
								}
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
			super::push(out, 420);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1286);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['ρ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2801);
							match s {
								[] => true,
								['α', 'λ', 'υ', 'ξ', s @ ..] => {
									super::push(out, 4557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n425(s, out),
		['κ', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['ρ', 'α', 'φ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5930);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 5931);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 4740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2928);
							match s {
								[] => true,
								['α', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2931);
							match s {
								[] => true,
								['α', 'κ', 'υ', 'ο', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 4742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4743);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4744);
									match s {
										[] => true,
										['τ', 'σ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 5932);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4746);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 4747);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4748);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 4749);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 4750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['ε', 'λ', 'ε', 'π', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['σ', 'υ', 'ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n427(s, out),
		['μ', s @ ..] => super::t1::n428(s, out),
		['ν', s @ ..] => super::t1::n429(s, out),
		['ξ', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n432(s, out),
		['ρ', s @ ..] => super::t1::n433(s, out),
		['σ', s @ ..] => super::t1::n434(s, out),
		['τ', s @ ..] => super::t1::n435(s, out),
		['υ', s @ ..] => super::t1::n436(s, out),
		['φ', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1401);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				['υ', s @ ..] => super::t1::n1429(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => super::t1::n438(s, out),
		['ψ', s @ ..] => super::t1::n439(s, out),
		['ω', s @ ..] => super::t1::n440(s, out),
		_ => false,
	}
}

/// Words ending -ρυγων.
pub(crate) fn n3187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3120);
	s.is_empty()
}

/// Words ending -ωγων.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1421);
	match s {
		[] => true,
		['π', 'ο', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 3121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δων.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['υ', 'τ', 'ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['μ', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1426);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εων.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1428);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['υ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1430);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζων.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1431);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['θ', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θων.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	s.is_empty()
}

/// Words ending -ιων.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1433);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1434);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1435);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['ν', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1438);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1440);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1441);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1442);
			match s {
				[] => true,
				['α', 'ρ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κων.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1443);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1444);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1445);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1446);
			match s {
				[] => true,
				['α', 'ν', 'υ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['ν', 'ι', 'ο', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1447);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1448);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λων.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1449);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μων.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1451);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1452);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1454);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['α', 'δ', 'ο', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1456);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νων.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['ι', 'σ', s @ ..] => {
			super::push(out, 1459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξων.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1460);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πων.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['λ', 'β', 'ω', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρων.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['γ', 'ι', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σων.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	s.is_empty()
}

/// Words ending -των.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['ε', 'γ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4885);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1471);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υων.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1472);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1473);
			match s {
				[] => true,
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1474);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φων.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1475);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1476);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1477);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χων.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1479);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1480);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φυλαξ.
pub(crate) fn n3231(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 4888);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4889);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5983);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 5984);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 5985);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 5986);
					match s {
						[] => true,
						['α', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 6419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 5987);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5988);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6420);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 6421);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5989);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5990);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6424);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 6425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 5991);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5992);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6426);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6427);
							match s {
								[] => true,
								['μ', 'ι', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 6553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5993);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5994);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6428);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6429);
							match s {
								[] => true,
								['ι', '\u{1}', s @ ..] => {
									super::push(out, 6554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 6430);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6431);
							match s {
								[] => true,
								['σ', 'ι', 'α', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 6555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5995);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5996);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6432);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6433);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 6434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οναξ.
pub(crate) fn n1526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1500);
	s.is_empty()
}

/// Words ending -ρναξ.
pub(crate) fn n1527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1501);
	s.is_empty()
}

/// Words ending -ωναξ.
pub(crate) fn n1528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1502);
	s.is_empty()
}

/// Words ending -οαξ.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	s.is_empty()
}

/// Words ending -παξ.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['ρ', 'α', s @ ..] => {
			super::push(out, 1503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ραξ.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1504);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1507);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['κ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4890);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1511);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σαξ.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	s.is_empty()
}

/// Words ending -ταξ.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1514);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1515);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υαξ.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	s.is_empty()
}

/// Words ending -φαξ.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	s.is_empty()
}

/// Words ending -ῥαξ.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	s.is_empty()
}

/// Words ending -γξ.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3173);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['α', 'θ', 'α', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 3175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['ι', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1527);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εξ.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	s.is_empty()
}

/// Words ending -ηξ.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['λ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4892);
									s.is_empty()
								}
								['ν', 'η', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 4893);
									s.is_empty()
								}
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
			super::push(out, 484);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιξ.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['ε', 'π', 'ο', 'ρ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3179);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3180);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 3181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3182);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3183);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4894);
									s.is_empty()
								}
								['κ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5997);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 5998);
											s.is_empty()
										}
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
					super::push(out, 1544);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['α', 'ρ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1549);
					match s {
						[] => true,
						['σ', 'υ', s @ ..] => {
							super::push(out, 3185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['σ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3186);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οξ.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρξ.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	s.is_empty()
}

/// Words ending -υξ.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['α', 'ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['ο', 'δ', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				['η', 'μ', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωξ.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	s.is_empty()
}

/// Words ending -υτηρ.
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1580);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3210);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 3211);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3213);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3214);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 3215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτηρ.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1581);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3216);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιρ.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['ε', 'χ', 'ι', s @ ..] => {
			super::push(out, 522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υρ.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['τ', 'ρ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωρ.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1582);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['η', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 4908);
									s.is_empty()
								}
								['κ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5999);
											s.is_empty()
										}
										['ο', s @ ..] => {
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6435);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 6436);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n6748(s, out),
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
				['η', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3218);
							match s {
								[] => true,
								['ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', 'λ', 'α', s @ ..] => {
							super::push(out, 3219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['α', 'λ', s @ ..] => {
							super::push(out, 3220);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 3222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 526);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιασ.
pub(crate) fn n533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1602);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3228);
					match s {
						[] => true,
						['π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 4910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['ν', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1605);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 3238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3239);
					match s {
						[] => true,
						['ε', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 4911);
							s.is_empty()
						}
						['σ', 'υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 4912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3240);
					match s {
						[] => true,
						['ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 4913);
							s.is_empty()
						}
						['σ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 4914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3242);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1611);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 4915);
							s.is_empty()
						}
						['ρ', 'ε', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3246);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 3247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3248);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3249);
					s.is_empty()
				}
				['η', 'τ', 'υ', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3250);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3251);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3252);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1614);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3254);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1617);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κελασ.
pub(crate) fn n3334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3262);
	s.is_empty()
}

/// Words ending -πελασ.
pub(crate) fn n3335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3263);
	s.is_empty()
}

/// Words ending -λλασ.
pub(crate) fn n1654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1626);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3264);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3265);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3266);
			s.is_empty()
		}
		['υ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 3267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολασ.
pub(crate) fn n1655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1627);
	s.is_empty()
}

/// Words ending -υλασ.
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1628);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 3268);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 3269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μασ.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['ξ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1630);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νασ.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1632);
			match s {
				[] => true,
				['ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οασ.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	s.is_empty()
}

/// Words ending -πασ.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1633);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1634);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1635);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1636);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρασ.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1638);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3272);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3273);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3275);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σασ.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	s.is_empty()
}

/// Words ending -τασ.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1642);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1643);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1644);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1645);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υασ.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1648);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1649);
			match s {
				[] => true,
				['ν', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1650);
			match s {
				[] => true,
				['δ', 'α', s @ ..] => {
					super::push(out, 3279);
					match s {
						[] => true,
						['μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φασ.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 3281);
					s.is_empty()
				}
				['ν', '\u{1}', s @ ..] => {
					super::push(out, 3282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χασ.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1654);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1655);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1656);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψασ.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	s.is_empty()
}

/// Words ending -ωασ.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	s.is_empty()
}

/// Words ending -εσ.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				['ι', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3286);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6000);
											s.is_empty()
										}
										['μ', s @ ..] => {
											super::push(out, 6001);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ησ.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['ε', 'λ', 'ο', s @ ..] => {
					super::push(out, 1664);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['γ', 'γ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						['ν', 'ω', 'ε', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['ι', 'κ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 4918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1675);
					match s {
						[] => true,
						['φ', 'ο', 'δ', 'ι', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['α', 'β', s @ ..] => {
							super::push(out, 3300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['π', 'ο', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4919);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 4920);
									s.is_empty()
								}
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
			super::push(out, 562);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3301);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3302);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3304);
							s.is_empty()
						}
						['ε', 'τ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4921);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['λ', 'ε', 'τ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3306);
							s.is_empty()
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4923);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3307);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3308);
							match s {
								[] => true,
								['μ', 'ο', 'ρ', 'υ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 4925);
									s.is_empty()
								}
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
			super::push(out, 566);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t1::n568(s, out),
		['υ', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['ρ', 'α', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3392);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 3393);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3394);
							match s {
								[] => true,
								['ρ', 'τ', s @ ..] => {
									super::push(out, 5048);
									s.is_empty()
								}
								['σ', s @ ..] => {
									match s {
										[] => true,
										['α', 'τ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6056);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 6057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 3395);
							match s {
								[] => true,
								['ι', '\u{1}', s @ ..] => {
									super::push(out, 5049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3396);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3397);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3398);
							match s {
								[] => true,
								['ε', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 5051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3399);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5052);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5053);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 5054);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5055);
									s.is_empty()
								}
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

/// Words ending -ισ.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['λ', 'ο', s @ ..] => {
							super::push(out, 3401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				['λ', s @ ..] => {
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
		['γ', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3402);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 3404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['υ', 'ο', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['ζ', '\u{1}', s @ ..] => {
							super::push(out, 3407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['ι', 'ῥ', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['ν', 'ε', 'χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 3409);
							s.is_empty()
						}
						['ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3411);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 3412);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5056);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5057);
									match s {
										[] => true,
										['α', 'π', 'ο', 'λ', 'η', 'μ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 6058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 3413);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3414);
							match s {
								[] => true,
								['υ', 'σ', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3415);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3416);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3417);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3420);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 3421);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3424);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5060);
									match s {
										[] => true,
										['υ', 'ρ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6059);
											s.is_empty()
										}
										_ => false,
									}
								}
								['γ', s @ ..] => {
									super::push(out, 5061);
									match s {
										[] => true,
										['α', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6060);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 5063);
									match s {
										[] => true,
										['υ', s @ ..] => {
											match s {
												[] => true,
												['ρ', 'χ', '\u{1}', s @ ..] => {
													super::push(out, 6451);
													s.is_empty()
												}
												['φ', '\u{1}', s @ ..] => {
													super::push(out, 6452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3425);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['π', 'ο', 'ρ', 'κ', 'α', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => super::t1::n1778(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n582(s, out),
		['ν', s @ ..] => super::t1::n583(s, out),
		['ξ', s @ ..] => super::t1::n584(s, out),
		['ο', s @ ..] => super::t1::n585(s, out),
		['π', s @ ..] => super::t1::n586(s, out),
		['ρ', s @ ..] => super::t1::n587(s, out),
		['σ', s @ ..] => super::t1::n588(s, out),
		['τ', s @ ..] => super::t1::n589(s, out),
		['φ', s @ ..] => super::t1::n590(s, out),
		['χ', s @ ..] => super::t1::n591(s, out),
		['ψ', s @ ..] => super::t1::n592(s, out),
		['ω', s @ ..] => super::t1::n593(s, out),
		['ϝ', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['ῥ', s @ ..] => super::t1::n595(s, out),
		_ => false,
	}
}

/// Words ending -λσ.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	s.is_empty()
}

/// Words ending -νσ.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	s.is_empty()
}

/// Words ending -οσ.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3619);
							match s {
								[] => true,
								['ο', 'θ', s @ ..] => {
									super::push(out, 5244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3620);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6147);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 6148);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 6149);
											s.is_empty()
										}
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
					super::push(out, 1840);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1841);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 3622);
							s.is_empty()
						}
						['ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 3623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3624);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5245);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6150);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 5247);
									match s {
										[] => true,
										['υ', 'θ', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 6151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3625);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3626);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3627);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5249);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 5250);
									match s {
										[] => true,
										['υ', '\u{1}', s @ ..] => {
											super::push(out, 6152);
											s.is_empty()
										}
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
					super::push(out, 1846);
					match s {
						[] => true,
						['ι', 'λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 3628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1847);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3629);
							s.is_empty()
						}
						['ρ', 'ο', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 3630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3631);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3632);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5251);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 3634);
							match s {
								[] => true,
								['ο', 'κ', s @ ..] => {
									super::push(out, 5254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 3635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 3637);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3638);
							match s {
								[] => true,
								['θ', 'φ', 'ι', s @ ..] => {
									super::push(out, 5256);
									s.is_empty()
								}
								['π', 'σ', 'ο', s @ ..] => {
									super::push(out, 5257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3639);
							match s {
								[] => true,
								['σ', 'ι', 'π', '\u{1}', s @ ..] => {
									super::push(out, 5258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['ν', s @ ..] => super::t1::n3718(s, out),
						['τ', s @ ..] => super::t1::n3719(s, out),
						['χ', s @ ..] => super::t1::n3720(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t1::n1884(s, out),
				['λ', s @ ..] => super::t1::n1885(s, out),
				['ο', s @ ..] => super::t1::n1886(s, out),
				['ρ', s @ ..] => super::t1::n1887(s, out),
				['σ', s @ ..] => super::t1::n1888(s, out),
				['υ', s @ ..] => super::t1::n1889(s, out),
				['ω', s @ ..] => super::t1::n1890(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t1::n599(s, out),
		['ε', s @ ..] => super::t1::n600(s, out),
		['ζ', s @ ..] => super::t1::n601(s, out),
		['θ', s @ ..] => super::t1::n602(s, out),
		['ι', s @ ..] => super::t1::n603(s, out),
		['κ', s @ ..] => super::t1::n604(s, out),
		['λ', s @ ..] => super::t1::n605(s, out),
		['μ', s @ ..] => super::t1::n606(s, out),
		['ν', s @ ..] => super::t1::n607(s, out),
		['ξ', s @ ..] => super::t1::n608(s, out),
		['ο', s @ ..] => super::t1::n609(s, out),
		['π', s @ ..] => super::t1::n610(s, out),
		['ρ', s @ ..] => super::t1::n611(s, out),
		['σ', s @ ..] => super::t1::n612(s, out),
		['τ', s @ ..] => super::t1::n613(s, out),
		['υ', s @ ..] => super::t1::n614(s, out),
		['φ', s @ ..] => super::t1::n615(s, out),
		['χ', s @ ..] => super::t1::n616(s, out),
		['ψ', s @ ..] => super::t1::n617(s, out),
		['ω', s @ ..] => super::t1::n618(s, out),
		_ => false,
	}
}

/// Words ending -υσ.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['ν', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2048);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2049);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 2051);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2053);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4170);
							match s {
								[] => true,
								['τ', 'α', 'λ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 5652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 4171);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4172);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5653);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 5654);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 5655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4173);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5656);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 5657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2054);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4174);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4175);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4177);
							match s {
								[] => true,
								['σ', 'α', 'β', s @ ..] => {
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6321);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 6322);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4178);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4179);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4180);
							match s {
								[] => true,
								['τ', 'ι', s @ ..] => {
									super::push(out, 5659);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2057);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4181);
							match s {
								[] => true,
								['λ', 'α', 'β', '\u{1}', s @ ..] => {
									super::push(out, 5660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 4182);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4183);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4184);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['π', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 4185);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4187);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 4188);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 4189);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4190);
							match s {
								[] => true,
								['ι', '\u{1}', s @ ..] => {
									super::push(out, 5662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 4191);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4192);
							match s {
								[] => true,
								['φ', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6323);
											s.is_empty()
										}
										['μ', s @ ..] => {
											super::push(out, 6324);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4193);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4194);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2060);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4196);
							s.is_empty()
						}
						['σ', 'α', s @ ..] => {
							super::push(out, 4197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2061);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4198);
							match s {
								[] => true,
								['μ', 'μ', 'α', 'ρ', 'γ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4199);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2062);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4201);
							match s {
								[] => true,
								['ρ', 'γ', 'γ', s @ ..] => {
									super::push(out, 5664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2063);
					match s {
						[] => true,
						['ο', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['χ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 625);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				['ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t1::n5817(s, out),
								['μ', s @ ..] => super::t1::n5818(s, out),
								_ => false,
							}
						}
						['ο', s @ ..] => super::t1::n4295(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n628(s, out),
		['μ', s @ ..] => super::t1::n629(s, out),
		['ν', s @ ..] => super::t1::n630(s, out),
		['ξ', s @ ..] => super::t1::n631(s, out),
		['ο', s @ ..] => super::t1::n632(s, out),
		['π', s @ ..] => super::t1::n633(s, out),
		['ρ', s @ ..] => super::t1::n634(s, out),
		['σ', s @ ..] => super::t1::n635(s, out),
		['τ', s @ ..] => super::t1::n636(s, out),
		['φ', s @ ..] => super::t1::n637(s, out),
		['χ', s @ ..] => super::t1::n638(s, out),
		_ => false,
	}
}

/// Words ending -ωσ.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['ι', 'ε', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 2105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['λ', 'ι', 'σ', 'η', 'γ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4220);
							s.is_empty()
						}
						['γ', '\u{1}', s @ ..] => {
							super::push(out, 4221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2108);
					s.is_empty()
				}
				['ε', 'σ', 'υ', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2109);
					s.is_empty()
				}
				['τ', 'α', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ματαιολογια.
pub(crate) fn n6719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6528);
	s.is_empty()
}

/// Words ending -τιολογια.
pub(crate) fn n6510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6330);
	s.is_empty()
}

/// Words ending -λολογια.
pub(crate) fn n5841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5689);
	s.is_empty()
}

/// Words ending -μολογια.
pub(crate) fn n5842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5690);
	s.is_empty()
}

/// Words ending -νολογια.
pub(crate) fn n5843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5691);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 6331);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 6332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξολογια.
pub(crate) fn n5844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5692);
	match s {
		[] => true,
		['ο', 'δ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 6333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πολογια.
pub(crate) fn n5845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5693);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 6334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρολογια.
pub(crate) fn n5846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5694);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 6335);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τολογια.
pub(crate) fn n5847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5695);
	s.is_empty()
}

/// Words ending -ργια.
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				['θ', 'η', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 4232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2153);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2154);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5696);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5697);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5698);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5699);
							match s {
								[] => true,
								['ι', 'ε', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 6336);
									s.is_empty()
								}
								['χ', 'σ', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υγια.
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 696);
	match s {
		[] => true,
		['ζ', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωγια.
pub(crate) fn n703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 697);
	match s {
		[] => true,
		['γ', 'α', s @ ..] => {
			match s {
				[] => true,
				['δ', 'ι', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 4233);
					s.is_empty()
				}
				['μ', 'η', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 4234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δια.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['ρ', 'φ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 699);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2162);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4235);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4236);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2163);
					match s {
						[] => true,
						['μ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 4238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2164);
					match s {
						[] => true,
						['α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εια.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2165);
					match s {
						[] => true,
						['σ', 'α', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2169);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2170);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['α', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2173);
					match s {
						[] => true,
						['π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 4242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2174);
					match s {
						[] => true,
						['τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 706);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2176);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5701);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5702);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 5703);
									match s {
										[] => true,
										['υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6338);
											s.is_empty()
										}
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
					super::push(out, 2178);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['γ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['φ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2187);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4251);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6339);
											s.is_empty()
										}
										['τ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 6340);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 5705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4252);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4253);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5706);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 5707);
									match s {
										[] => true,
										['υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6341);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 4254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2189);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4258);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 4261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['γ', 'ι', s @ ..] => {
							super::push(out, 4262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 714);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['ρ', 'τ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2201);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				['δ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2203);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['φ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['α', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 4268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		['τ', s @ ..] => super::t1::n723(s, out),
		['υ', s @ ..] => {
			super::push(out, 718);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['ε', 'ν', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θια.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4276);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4277);
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
		['υ', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['μ', 'ο', 'τ', 'ι', 'ρ', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιια.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['ο', 'π', 'ο', s @ ..] => {
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['α', 'μ', 'ρ', 'α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['ω', 'ρ', s @ ..] => {
							super::push(out, 4279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κια.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 725);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4283);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4284);
							match s {
								[] => true,
								['α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 5716);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 5717);
									s.is_empty()
								}
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
			super::push(out, 727);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['τ', 'ο', 'τ', 'ω', 'ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 729);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λια.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4285);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 732);
			match s {
				[] => true,
				['γ', 'γ', 'α', s @ ..] => {
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4287);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 2247);
					match s {
						[] => true,
						['ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2252);
					match s {
						[] => true,
						['κ', 'γ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				['χ', 'ο', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				['χ', 'σ', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 736);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['ο', 'β', 'υ', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μια.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2259);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
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
		['γ', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2263);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['υ', 'ρ', s @ ..] => {
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4292);
							s.is_empty()
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 747);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2272);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2273);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4294);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4295);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4296);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2274);
					match s {
						[] => true,
						['δ', s @ ..] => {
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5720);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5721);
									match s {
										[] => true,
										['τ', 'α', 'μ', 'ρ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6346);
											s.is_empty()
										}
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
					super::push(out, 2275);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4297);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4298);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['π', 'υ', '\u{1}', s @ ..] => {
											super::push(out, 6347);
											s.is_empty()
										}
										['ρ', 'ε', 'θ', 'υ', 'ε', 'λ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 6348);
											s.is_empty()
										}
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
			super::push(out, 749);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4299);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4300);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4301);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4302);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4303);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5722);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6349);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 6350);
											match s {
												[] => true,
												['α', '\u{1}', s @ ..] => {
													super::push(out, 6529);
													s.is_empty()
												}
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
							super::push(out, 4304);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4305);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5723);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 5724);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 5725);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 4306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['ω', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5727);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5728);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['ν', 'γ', 'ο', 'ι', 'σ', 'υ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νια.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['κ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 4307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['ξ', 'ο', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 4308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['α', 'κ', 'γ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 757);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['η', 'δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4310);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4312);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4314);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4315);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5731);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6351);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6352);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4316);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 760);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4318);
							s.is_empty()
						}
						['α', '\u{1}', s @ ..] => {
							super::push(out, 4319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4320);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 4321);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4322);
							match s {
								[] => true,
								['κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 5732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 4323);
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 5733);
									s.is_empty()
								}
								['λ', 'ο', 'π', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ξια.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2298);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4324);
							s.is_empty()
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5735);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 5736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['ρ', 'ο', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['λ', 'π', 'ο', 'π', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4325);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4326);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5737);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 5738);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 5739);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5740);
									s.is_empty()
								}
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
			super::push(out, 770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οια.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['λ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4327);
							s.is_empty()
						}
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4328);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4329);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['ρ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πια.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 775);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 777);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρια.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4336);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4337);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4338);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4339);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4340);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5741);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 5742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['μ', 'α', 'τ', 'ν', 'ο', 'κ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 785);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['η', s @ ..] => {
							match s {
								[] => true,
								['λ', 'λ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5743);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5744);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 5745);
									match s {
										[] => true,
										['ι', '\u{1}', s @ ..] => {
											super::push(out, 6353);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4343);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2327);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4345);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4346);
							match s {
								[] => true,
								['ρ', 'η', 'λ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 5747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 4347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 787);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['μ', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5748);
									match s {
										[] => true,
										['υ', 'σ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 5749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['θ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2333);
					match s {
						[] => true,
						['ε', 'κ', 'ο', 'δ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2335);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2337);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2339);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σια.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2343);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2344);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4354);
							match s {
								[] => true,
								['κ', 'ο', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 5750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4356);
							match s {
								[] => true,
								['θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4358);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4359);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4362);
							match s {
								[] => true,
								['ο', 'χ', 'υ', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 5752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2353);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2356);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4365);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['σ', 'ω', 'ῥ', '\u{1}', s @ ..] => {
			super::push(out, 798);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 4367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2367);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4368);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4369);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4370);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 5753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4371);
							match s {
								[] => true,
								['α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 5754);
									s.is_empty()
								}
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
			super::push(out, 800);
			match s {
				[] => true,
				['ν', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['ῥ', '\u{1}', s @ ..] => {
					super::push(out, 2369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τια.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4372);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 5755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4373);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6355);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6356);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 6357);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 6358);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 6530);
													s.is_empty()
												}
												['υ', s @ ..] => {
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n6782(s, out),
														['ο', s @ ..] => super::t1::n6783(s, out),
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
		['η', s @ ..] => {
			super::push(out, 802);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 803);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 804);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				['μ', 'ω', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 807);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2378);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4374);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4375);
							match s {
								[] => true,
								['α', 'χ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6359);
											s.is_empty()
										}
										['υ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 6360);
											s.is_empty()
										}
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
					super::push(out, 2379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['α', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υια.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['θ', '\u{1}', s @ ..] => {
			super::push(out, 812);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φια.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2381);
					s.is_empty()
				}
				['ρ', 'γ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4376);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4377);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5756);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 5757);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 5758);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 5759);
									s.is_empty()
								}
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
			super::push(out, 816);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2383);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2384);
					match s {
						[] => true,
						['ο', 'λ', s @ ..] => {
							super::push(out, 4378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 818);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χια.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2385);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4379);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 4381);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4382);
							match s {
								[] => true,
								['ν', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6361);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 6362);
											match s {
												[] => true,
												['ζ', s @ ..] => {
													super::push(out, 6531);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 2386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 821);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 823);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4383);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 4384);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4385);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4386);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4387);
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 5762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4388);
							match s {
								[] => true,
								['ε', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 5763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2387);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2388);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2389);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2390);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4390);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4391);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4392);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψια.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['λ', 'β', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιλημα.
pub(crate) fn n2470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2428);
	s.is_empty()
}

/// Words ending -κλημα.
pub(crate) fn n2471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2429);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 4415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μημα.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 855);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 2430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νημα.
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 856);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2431);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2432);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2433);
			match s {
				[] => true,
				['ο', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 4416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οημα.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 857);
	match s {
		[] => true,
		['ν', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πημα.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 858);
	s.is_empty()
}

/// Words ending -ρημα.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 859);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2435);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2436);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2438);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σημα.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 860);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2440);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τημα.
pub(crate) fn n869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 861);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['α', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 4419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2444);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5768);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υημα.
pub(crate) fn n870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 2445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φημα.
pub(crate) fn n871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2446);
			s.is_empty()
		}
		['ο', 'σ', s @ ..] => {
			super::push(out, 2447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χημα.
pub(crate) fn n872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2448);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 5770);
							s.is_empty()
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 5771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιμα.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		['ρ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λμα.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μμα.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2454);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2455);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5772);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 2456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4422);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2457);
					match s {
						[] => true,
						['λ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['τ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 4425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['κ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['τ', 'σ', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομα.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	s.is_empty()
}

/// Words ending -ρμα.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2465);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

