use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => super::t0::n1(s, out),
		['ε', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 248);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 36);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 249);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 250);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 251);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1082);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2695);
											s.is_empty()
										}
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
		['η', s @ ..] => super::t0::n3(s, out),
		['ι', s @ ..] => super::t0::n4(s, out),
		['ν', s @ ..] => super::t0::n5(s, out),
		['ξ', s @ ..] => super::t0::n6(s, out),
		['ρ', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['α', s @ ..] => super::t0::n86(s, out),
				['η', s @ ..] => super::t0::n87(s, out),
				['ι', s @ ..] => {
					super::push(out, 86);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 560);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1701);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 1702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3319);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 3320);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 5179);
													s.is_empty()
												}
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
					super::push(out, 87);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 562);
							s.is_empty()
						}
						['τ', 'ρ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => super::t0::n91(s, out),
				_ => false,
			}
		}
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
					super::push(out, 122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 742);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2328);
									s.is_empty()
								}
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
			super::push(out, 14);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 126);
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
					super::push(out, 127);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 743);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2329);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4564);
											s.is_empty()
										}
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
					super::push(out, 128);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 744);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 745);
							match s {
								[] => true,
								['τ', 'σ', s @ ..] => {
									super::push(out, 2331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 746);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 747);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 131);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 748);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2334);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4565);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 749);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2335);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4566);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 751);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 132);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 753);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 754);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2338);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 4567);
											s.is_empty()
										}
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
					super::push(out, 133);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 755);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2339);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 4568);
											s.is_empty()
										}
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
		['ζ', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 134);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 135);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 136);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 760);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2340);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4569);
											s.is_empty()
										}
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
					super::push(out, 137);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 761);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', 'μ', 'δ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 17);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['θ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 138);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 139);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 763);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 140);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 764);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 141);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 765);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2344);
									s.is_empty()
								}
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
					super::push(out, 163);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 870);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 2477);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 4686);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 871);
							match s {
								[] => true,
								['λ', s @ ..] => super::t0::n2497(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n166(s, out),
				['λ', s @ ..] => {
					super::push(out, 165);
					match s {
						[] => true,
						['ε', s @ ..] => super::t0::n880(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n21(s, out),
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
			super::push(out, 38);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 252);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1083);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1087);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1088);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2698);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4811);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1089);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2700);
									s.is_empty()
								}
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
			super::push(out, 39);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1091);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1092);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', 'ι', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1093);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1094);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2704);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 2705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 261);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'λ', 'η', s @ ..] => {
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
			super::push(out, 40);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1097);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1099);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1100);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 268);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 270);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 42);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 271);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1103);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'ρ', s @ ..] => {
					super::push(out, 273);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 274);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			match s {
				[] => true,
				['α', 'λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 277);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n45(s, out),
		['λ', s @ ..] => super::t0::n46(s, out),
		['μ', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['λ', 'α', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 304);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['ρ', s @ ..] => super::t0::n1179(s, out),
						_ => false,
					}
				}
				['κ', 'ι', s @ ..] => super::t0::n309(s, out),
				['λ', s @ ..] => super::t0::n310(s, out),
				['μ', s @ ..] => super::t0::n311(s, out),
				['ο', s @ ..] => super::t0::n312(s, out),
				['ρ', s @ ..] => super::t0::n313(s, out),
				['σ', s @ ..] => super::t0::n314(s, out),
				['υ', s @ ..] => super::t0::n315(s, out),
				['χ', s @ ..] => super::t0::n316(s, out),
				['ω', s @ ..] => super::t0::n317(s, out),
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n48(s, out),
		['ο', s @ ..] => super::t0::n49(s, out),
		['π', s @ ..] => super::t0::n50(s, out),
		['ρ', s @ ..] => super::t0::n51(s, out),
		['σ', s @ ..] => super::t0::n52(s, out),
		['τ', s @ ..] => super::t0::n53(s, out),
		['υ', s @ ..] => super::t0::n54(s, out),
		['φ', s @ ..] => super::t0::n55(s, out),
		['χ', s @ ..] => super::t0::n56(s, out),
		['ω', s @ ..] => super::t0::n57(s, out),
		_ => false,
	}
}

/// Words ending -ι.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1270);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2804);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 4854);
											s.is_empty()
										}
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
					super::push(out, 379);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1271);
							match s {
								[] => true,
								['ι', 'χ', s @ ..] => {
									super::push(out, 2805);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4855);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6335);
													match s {
														[] => true,
														['β', '\u{1}', s @ ..] => super::t0::n7096(s, out),
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
							super::push(out, 1272);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2806);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 4856);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6336);
													match s {
														[] => true,
														['κ', '\u{1}', s @ ..] => super::t0::n7097(s, out),
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
				['ζ', s @ ..] => {
					super::push(out, 380);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1273);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2807);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 4857);
											s.is_empty()
										}
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
					super::push(out, 381);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1274);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2808);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 4858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1275);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2809);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1276);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2810);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4860);
											s.is_empty()
										}
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
					super::push(out, 382);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1277);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2811);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 4861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1278);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 383);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1279);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2813);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 4862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1280);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2814);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 4863);
											s.is_empty()
										}
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
					super::push(out, 384);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1281);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2815);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 4864);
											s.is_empty()
										}
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
					super::push(out, 385);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1282);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2816);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1283);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2817);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 4866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => super::t0::n392(s, out),
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 57);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 388);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1290);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
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
			super::push(out, 59);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 389);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n62(s, out),
		['μ', s @ ..] => super::t0::n63(s, out),
		['ν', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['ο', s @ ..] => super::t0::n65(s, out),
		['π', s @ ..] => super::t0::n66(s, out),
		['ρ', s @ ..] => super::t0::n67(s, out),
		['σ', s @ ..] => super::t0::n68(s, out),
		['φ', s @ ..] => super::t0::n69(s, out),
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 413);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1339);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2862);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4895);
											s.is_empty()
										}
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
					super::push(out, 414);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1341);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2863);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4896);
											s.is_empty()
										}
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
					super::push(out, 416);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 419);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1345);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2865);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 4897);
											s.is_empty()
										}
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
			super::push(out, 69);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1346);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 421);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 423);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 426);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1350);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1351);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2868);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4898);
											s.is_empty()
										}
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
					super::push(out, 427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 428);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1354);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 429);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1355);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2870);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 4899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2873);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 4900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2874);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2875);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 4902);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t0::n442(s, out),
				['σ', s @ ..] => {
					super::push(out, 438);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2881);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4905);
											s.is_empty()
										}
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
					super::push(out, 439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n74(s, out),
		['υ', s @ ..] => super::t0::n75(s, out),
		['ω', s @ ..] => super::t0::n76(s, out),
		_ => false,
	}
}

/// Words ending -ξ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1563);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3199);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5105);
											s.is_empty()
										}
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
					super::push(out, 489);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 490);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3200);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1566);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3201);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3202);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5108);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6456);
													s.is_empty()
												}
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
							super::push(out, 1568);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1569);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3204);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5109);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6457);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 3205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5110);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 5111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1570);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1571);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1572);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3208);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 5112);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 3209);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5113);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1573);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1574);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3211);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5114);
											s.is_empty()
										}
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
					super::push(out, 492);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1576);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3213);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5115);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 5116);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1577);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1578);
							match s {
								[] => true,
								['ρ', s @ ..] => super::t0::n3242(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n498(s, out),
				['ρ', s @ ..] => super::t0::n499(s, out),
				['σ', s @ ..] => super::t0::n500(s, out),
				['τ', s @ ..] => super::t0::n501(s, out),
				['φ', s @ ..] => super::t0::n502(s, out),
				['ῥ', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n78(s, out),
		['ε', s @ ..] => super::t0::n79(s, out),
		['η', s @ ..] => super::t0::n80(s, out),
		['ι', s @ ..] => super::t0::n81(s, out),
		['ο', s @ ..] => super::t0::n82(s, out),
		['ρ', s @ ..] => super::t0::n83(s, out),
		['υ', s @ ..] => super::t0::n84(s, out),
		['ω', s @ ..] => super::t0::n85(s, out),
		_ => false,
	}
}

/// Words ending -αρ.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3291);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1666);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5163);
									s.is_empty()
								}
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
			super::push(out, 539);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1669);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1670);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 3300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3301);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3302);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5165);
									s.is_empty()
								}
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
			super::push(out, 546);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3303);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5166);
									s.is_empty()
								}
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
			super::push(out, 548);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηρ.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3307);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 3308);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5169);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6468);
											s.is_empty()
										}
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
			super::push(out, 554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3309);
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
				['\u{1}', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3310);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5170);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 6469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3311);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3313);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5174);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 5175);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6470);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 5176);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6471);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7105);
													match s {
														[] => true,
														['χ', s @ ..] => super::t0::n7367(s, out),
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
				['υ', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3316);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5178);
									s.is_empty()
								}
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
			super::push(out, 558);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρ.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1703);
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
		['κ', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3322);
							match s {
								[] => true,
								['η', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 5180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3323);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5181);
									match s {
										[] => true,
										['ο', 'τ', s @ ..] => {
											super::push(out, 6472);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7106);
													s.is_empty()
												}
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
							super::push(out, 3324);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5182);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6473);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7107);
													s.is_empty()
												}
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
				['ι', 'ε', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3326);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5184);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6474);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7108);
													s.is_empty()
												}
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
							super::push(out, 3327);
							match s {
								[] => true,
								['λ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6475);
											s.is_empty()
										}
										['η', s @ ..] => {
											super::push(out, 6476);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7109);
													s.is_empty()
												}
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
					super::push(out, 1709);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3328);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5185);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6477);
											s.is_empty()
										}
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
					super::push(out, 1710);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3329);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5186);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6478);
											s.is_empty()
										}
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
					super::push(out, 1711);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5187);
									s.is_empty()
								}
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
			super::push(out, 567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1714);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3331);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 5188);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1716);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1717);
							match s {
								[] => true,
								['λ', 'α', s @ ..] => {
									super::push(out, 3333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1718);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3334);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3335);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 5189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => super::t0::n576(s, out),
				['δ', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['ε', s @ ..] => super::t0::n578(s, out),
				['ι', s @ ..] => super::t0::n579(s, out),
				['κ', s @ ..] => super::t0::n580(s, out),
				['λ', s @ ..] => super::t0::n581(s, out),
				['μ', s @ ..] => super::t0::n582(s, out),
				['ν', s @ ..] => super::t0::n583(s, out),
				['ο', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n585(s, out),
				['ρ', s @ ..] => super::t0::n586(s, out),
				['σ', s @ ..] => super::t0::n587(s, out),
				['τ', s @ ..] => super::t0::n588(s, out),
				['υ', s @ ..] => super::t0::n589(s, out),
				['φ', s @ ..] => super::t0::n590(s, out),
				['χ', s @ ..] => super::t0::n591(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n93(s, out),
		['η', s @ ..] => super::t0::n94(s, out),
		['ι', s @ ..] => super::t0::n95(s, out),
		['λ', s @ ..] => super::t0::n96(s, out),
		['ν', s @ ..] => super::t0::n97(s, out),
		['ο', s @ ..] => super::t0::n98(s, out),
		['υ', s @ ..] => super::t0::n99(s, out),
		['ω', s @ ..] => super::t0::n100(s, out),
		_ => false,
	}
}

/// Words ending -υ.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
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
		['ν', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['γ', s @ ..] => {
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
			super::push(out, 103);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2290);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['κ', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2297);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
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
			super::push(out, 110);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2300);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2302);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['γ', 'ι', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2304);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2305);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4548);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6202);
											s.is_empty()
										}
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
					super::push(out, 717);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2306);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6203);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2307);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4550);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6204);
											s.is_empty()
										}
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
					super::push(out, 718);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4551);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 6205);
											s.is_empty()
										}
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
					super::push(out, 723);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2310);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4552);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6206);
											s.is_empty()
										}
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
			super::push(out, 115);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 4553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['γ', s @ ..] => {
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
			super::push(out, 116);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2314);
							match s {
								[] => true,
								['μ', 'υ', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 4554);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 4555);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6207);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7003);
													s.is_empty()
												}
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
							super::push(out, 2315);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4557);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6208);
											s.is_empty()
										}
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
					super::push(out, 731);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 732);
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
		['δ', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2323);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 4559);
									s.is_empty()
								}
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
			super::push(out, 118);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2324);
							match s {
								[] => true,
								['π', '\u{1}', s @ ..] => {
									super::push(out, 4560);
									s.is_empty()
								}
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
			super::push(out, 119);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 4561);
									s.is_empty()
								}
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
			super::push(out, 121);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4563);
									s.is_empty()
								}
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

/// Words ending -ια.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2345);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4570);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2346);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4571);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6210);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7004);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4572);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6211);
											s.is_empty()
										}
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
					super::push(out, 769);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2347);
							match s {
								[] => true,
								['φ', 'ε', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 4573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2351);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4574);
									s.is_empty()
								}
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
			super::push(out, 144);
			match s {
				[] => true,
				['α', 'ρ', 'ρ', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2352);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2353);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4576);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6212);
											s.is_empty()
										}
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
					super::push(out, 777);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2354);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4577);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6213);
											match s {
												[] => true,
												['η', 'κ', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 7005);
													s.is_empty()
												}
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
					super::push(out, 778);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2355);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2356);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['θ', 'α', s @ ..] => {
											super::push(out, 6214);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7006);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 6215);
											match s {
												[] => true,
												['α', 'χ', s @ ..] => {
													super::push(out, 7007);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t0::n7337(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 7008);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n7338(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 6216);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7009);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n7339(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', 'ο', s @ ..] => {
											super::push(out, 6217);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7010);
													match s {
														[] => true,
														['χ', s @ ..] => super::t0::n7340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 6218);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7011);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n7341(s, out),
														_ => false,
													}
												}
												['ω', s @ ..] => {
													super::push(out, 7012);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n7342(s, out),
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
				['ρ', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['ε', 'ν', 'ε', s @ ..] => {
							super::push(out, 2357);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						['υ', 'ο', 'ε', s @ ..] => {
							super::push(out, 2359);
							match s {
								[] => true,
								['θ', s @ ..] => super::t0::n4609(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => super::t1::n147(s, out),
		['ε', s @ ..] => super::t1::n148(s, out),
		['θ', s @ ..] => super::t1::n149(s, out),
		['ι', s @ ..] => super::t1::n150(s, out),
		['κ', s @ ..] => super::t1::n151(s, out),
		['λ', s @ ..] => super::t1::n152(s, out),
		['μ', s @ ..] => super::t1::n153(s, out),
		['ν', s @ ..] => super::t1::n154(s, out),
		['ξ', s @ ..] => super::t1::n155(s, out),
		['ο', s @ ..] => super::t1::n156(s, out),
		['π', s @ ..] => super::t1::n157(s, out),
		['ρ', s @ ..] => super::t1::n158(s, out),
		['σ', s @ ..] => super::t1::n159(s, out),
		['τ', s @ ..] => super::t1::n160(s, out),
		['υ', s @ ..] => super::t1::n161(s, out),
		['φ', s @ ..] => super::t1::n162(s, out),
		['χ', s @ ..] => super::t1::n163(s, out),
		['ψ', s @ ..] => super::t1::n164(s, out),
		_ => false,
	}
}

/// Words ending -λλακα.
pub(crate) fn n2497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2478);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4687);
			match s {
				[] => true,
				['μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 6276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ικα.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2479);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2480);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ελκα.
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 874);
	s.is_empty()
}

/// Words ending -λα.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2481);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 4691);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6277);
											s.is_empty()
										}
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
					super::push(out, 877);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2484);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						['τ', '\u{1}', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2487);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2489);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 4694);
									s.is_empty()
								}
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
			super::push(out, 169);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2490);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4695);
									s.is_empty()
								}
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
			super::push(out, 170);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 4696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2493);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2495);
							match s {
								[] => true,
								['ο', 'θ', s @ ..] => {
									super::push(out, 4698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2496);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 4699);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6278);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ψ', s @ ..] => {
							super::push(out, 2498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2499);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4701);
									s.is_empty()
								}
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
			super::push(out, 172);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2500);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4702);
									s.is_empty()
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2501);
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
				['α', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 890);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
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
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2507);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4703);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 4704);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						['τ', 'σ', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['θ', 'φ', 'ο', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['δ', 'ο', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['π', 'α', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2514);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4706);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6279);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7038);
													s.is_empty()
												}
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
							super::push(out, 2515);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4707);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6280);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 7039);
													s.is_empty()
												}
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
							super::push(out, 2516);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2518);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2519);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 4709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						['ψ', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['ι', 'ο', s @ ..] => {
							super::push(out, 2522);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4710);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6281);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7040);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7349(s, out),
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
				['ν', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2523);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4711);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6282);
											s.is_empty()
										}
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
					super::push(out, 907);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 2526);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4712);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6283);
											s.is_empty()
										}
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
					super::push(out, 908);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'θ', s @ ..] => {
					super::push(out, 909);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2528);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n185(s, out),
		['ο', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t1::n187(s, out),
		['σ', s @ ..] => super::t1::n188(s, out),
		['υ', s @ ..] => super::t1::n189(s, out),
		['χ', s @ ..] => super::t1::n190(s, out),
		['ω', s @ ..] => super::t1::n191(s, out),
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2581);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2582);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2587);
							match s {
								[] => true,
								['β', 'υ', 'λ', 'ο', 'κ', s @ ..] => {
									super::push(out, 4743);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6290);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2588);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4744);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6291);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 7043);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 4745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2591);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4746);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6292);
											s.is_empty()
										}
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
			super::push(out, 194);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2593);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2595);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2596);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4749);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6293);
											s.is_empty()
										}
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
					super::push(out, 970);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2597);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2599);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2600);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4752);
									s.is_empty()
								}
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
			super::push(out, 198);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2602);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4753);
									s.is_empty()
								}
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

/// Words ending -ξα.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οα.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 207);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 981);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2605);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
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
			super::push(out, 210);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2608);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2609);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2610);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2615);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2616);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4760);
									match s {
										[] => true,
										['χ', '\u{1}', s @ ..] => {
											super::push(out, 6294);
											s.is_empty()
										}
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
					super::push(out, 997);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2619);
							match s {
								[] => true,
								['μ', 'α', s @ ..] => {
									super::push(out, 4761);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6295);
											match s {
												[] => true,
												['α', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 7044);
													s.is_empty()
												}
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
							super::push(out, 2620);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4762);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6296);
											s.is_empty()
										}
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
					super::push(out, 999);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['θ', 'ν', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2623);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2624);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 4764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2625);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2626);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4766);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6297);
											s.is_empty()
										}
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
			super::push(out, 215);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2627);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2628);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4768);
									match s {
										[] => true,
										['κ', 'α', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6298);
											s.is_empty()
										}
										['σ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6299);
											s.is_empty()
										}
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
		['θ', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2629);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2630);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4770);
									match s {
										[] => true,
										['ρ', s @ ..] => super::t1::n6338(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => super::t1::n1021(s, out),
				['ι', s @ ..] => super::t1::n1022(s, out),
				['ρ', s @ ..] => super::t1::n1023(s, out),
				['χ', s @ ..] => super::t1::n1024(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n219(s, out),
		['κ', s @ ..] => super::t1::n220(s, out),
		['ο', s @ ..] => super::t1::n221(s, out),
		['π', s @ ..] => super::t1::n222(s, out),
		['ρ', s @ ..] => super::t1::n223(s, out),
		['τ', s @ ..] => super::t1::n224(s, out),
		['υ', s @ ..] => super::t1::n225(s, out),
		['φ', s @ ..] => super::t1::n226(s, out),
		['χ', s @ ..] => super::t1::n227(s, out),
		['ω', s @ ..] => super::t1::n228(s, out),
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1050);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2680);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4803);
									s.is_empty()
								}
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
			super::push(out, 228);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['α', 'λ', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['λ', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
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
			super::push(out, 231);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2684);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4804);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6316);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 2685);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 2686);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4806);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 6317);
											s.is_empty()
										}
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
			super::push(out, 232);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4807);
									s.is_empty()
								}
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
			super::push(out, 233);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2688);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4808);
									s.is_empty()
								}
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
			super::push(out, 234);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2689);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4809);
									s.is_empty()
								}
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
			super::push(out, 235);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['λ', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['ι', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['ο', '\u{1}', s @ ..] => {
					super::push(out, 1071);
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
		['μ', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		['ο', 'β', '\u{1}', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
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
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2694);
							match s {
								[] => true,
								['α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 4810);
									s.is_empty()
								}
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

/// Words ending -ψα.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωα.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κη.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['ι', 'ρ', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2711);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4812);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6318);
											s.is_empty()
										}
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
					super::push(out, 1111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2712);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4813);
									s.is_empty()
								}
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
			super::push(out, 281);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2713);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2715);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2718);
							match s {
								[] => true,
								['τ', 'κ', s @ ..] => {
									super::push(out, 4816);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6319);
											s.is_empty()
										}
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
					super::push(out, 1119);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2719);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2720);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4817);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6320);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7050);
													s.is_empty()
												}
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
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['ε', 'ρ', s @ ..] => {
							super::push(out, 2722);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4818);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6321);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', 'α', s @ ..] => {
							super::push(out, 2723);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4819);
									s.is_empty()
								}
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
			super::push(out, 283);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4820);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6322);
											match s {
												[] => true,
												['υ', 'κ', '\u{1}', s @ ..] => {
													super::push(out, 7051);
													s.is_empty()
												}
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
		['ο', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2729);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2730);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2731);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 4823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2732);
							match s {
								[] => true,
								['α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 4824);
									s.is_empty()
								}
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
			super::push(out, 289);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λη.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2733);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', 'α', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2734);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4826);
									s.is_empty()
								}
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
			super::push(out, 291);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2735);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2737);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6323);
											s.is_empty()
										}
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
					super::push(out, 1142);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['μ', 'ι', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2738);
							match s {
								[] => true,
								['α', 'μ', s @ ..] => {
									super::push(out, 4829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', 'ε', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1158);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2747);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1162);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 2750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριμη.
pub(crate) fn n1179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1171);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικμη.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	s.is_empty()
}

/// Words ending -λμη.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μμη.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομη.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1174);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρμη.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1176);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σμη.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	s.is_empty()
}

/// Words ending -υμη.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1179);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 1180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χμη.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	s.is_empty()
}

/// Words ending -ωμη.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1181);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2755);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νη.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['π', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4832);
									s.is_empty()
								}
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
			super::push(out, 316);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2757);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4833);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 6324);
											s.is_empty()
										}
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
					super::push(out, 1187);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2758);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4834);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 2759);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4835);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6326);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7052);
													match s {
														[] => true,
														['ρ', 'ε', 'π', '\u{1}', s @ ..] => super::t1::n7350(s, out),
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
			super::push(out, 317);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['γ', '\u{1}', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						['ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', '\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['ο', 'β', 'ε', 'λ', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1200);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						['η', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2771);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2772);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4838);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6327);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['λ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ο', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 2774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2775);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4839);
									s.is_empty()
								}
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
			super::push(out, 320);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2776);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4840);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6328);
											s.is_empty()
										}
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
						['η', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2778);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['ε', 'φ', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2779);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4842);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6329);
											s.is_empty()
										}
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
					super::push(out, 1212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t1::n1222(s, out),
				['σ', s @ ..] => super::t1::n1223(s, out),
				['χ', s @ ..] => super::t1::n1224(s, out),
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => super::t1::n329(s, out),
		['ω', s @ ..] => super::t1::n330(s, out),
		_ => false,
	}
}

/// Words ending -οη.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πη.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		['μ', 'ο', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1230);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2787);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4849);
									s.is_empty()
								}
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
			super::push(out, 337);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρη.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 341);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ση.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τη.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'λ', 'ε', 'μ', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4850);
									s.is_empty()
								}
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
			super::push(out, 350);
			match s {
				[] => true,
				['α', 'ι', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['α', 'λ', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υη.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 358);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φη.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2800);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'τ', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['λ', 'ι', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', 'υ', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χη.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2802);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1264);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2803);
							match s {
								[] => true,
								['ε', 'ν', s @ ..] => {
									super::push(out, 4853);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6334);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7054);
													s.is_empty()
												}
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
		['ρ', 'υ', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωη.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	s.is_empty()
}

/// The word βακχιαδαι.
pub(crate) fn n7096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7055);
	s.is_empty()
}

/// The word καλανδαι.
pub(crate) fn n7097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7056);
	s.is_empty()
}

/// Words ending -ται.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1284);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2820);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4867);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2821);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 4868);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2822);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2823);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2824);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4871);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6339);
									s.is_empty()
								}
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
			super::push(out, 1289);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2825);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4872);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6340);
									match s {
										[] => true,
										['π', '\u{1}', s @ ..] => {
											super::push(out, 7057);
											s.is_empty()
										}
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

/// Words ending -λι.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
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
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2828);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
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
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2830);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4874);
									s.is_empty()
								}
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
			super::push(out, 395);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2831);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2832);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1307);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2833);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4877);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6341);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7058);
													s.is_empty()
												}
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
							super::push(out, 2834);
							match s {
								[] => true,
								['α', 'π', s @ ..] => {
									super::push(out, 4878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2835);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4879);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6342);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 7059);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7351(s, out),
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
				['δ', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2837);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1310);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1311);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1312);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4881);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2840);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2841);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2843);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4884);
									s.is_empty()
								}
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
			super::push(out, 403);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2844);
							match s {
								[] => true,
								['ο', 'β', s @ ..] => {
									super::push(out, 4885);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6344);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7060);
													s.is_empty()
												}
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
				['γ', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2846);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4886);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 2847);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4887);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 2848);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2849);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t1::n409(s, out),
		['σ', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['ν', 'α', 'ζ', s @ ..] => super::t1::n1339(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => super::t1::n412(s, out),
		_ => false,
	}
}

/// Words ending -πι.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
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
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2860);
							match s {
								[] => true,
								['π', '\u{1}', s @ ..] => {
									super::push(out, 4894);
									s.is_empty()
								}
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

/// Words ending -σι.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φι.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οιν.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2877);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2879);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2880);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4904);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6347);
									s.is_empty()
								}
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

/// Words ending -ον.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2882);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1372);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1373);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'ῥ', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2887);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 4907);
									match s {
										[] => true,
										['ο', s @ ..] => {
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
		['η', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n454(s, out),
		['κ', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1400);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2961);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1401);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2963);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4973);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6394);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7083);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 4974);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6395);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7084);
													s.is_empty()
												}
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
							super::push(out, 2964);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4975);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2965);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4976);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6397);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7085);
													s.is_empty()
												}
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
					super::push(out, 1402);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2967);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1404);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2969);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2970);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2971);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1407);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2974);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2975);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2976);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ψ', s @ ..] => {
							super::push(out, 2977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1409);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2978);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4981);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 6398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2979);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => super::t1::n1419(s, out),
				['ο', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => super::t1::n1422(s, out),
				['υ', s @ ..] => super::t1::n1423(s, out),
				['ω', s @ ..] => super::t1::n1424(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n457(s, out),
		['ν', s @ ..] => super::t1::n458(s, out),
		['ξ', s @ ..] => super::t1::n459(s, out),
		['ο', s @ ..] => {
			super::push(out, 455);
			s.is_empty()
		}
		['π', s @ ..] => super::t1::n461(s, out),
		['ρ', s @ ..] => super::t1::n462(s, out),
		['σ', s @ ..] => super::t1::n463(s, out),
		['τ', s @ ..] => super::t1::n464(s, out),
		['υ', s @ ..] => super::t1::n465(s, out),
		['φ', s @ ..] => super::t1::n466(s, out),
		['χ', s @ ..] => super::t1::n467(s, out),
		['ψ', s @ ..] => super::t1::n468(s, out),
		['ω', s @ ..] => super::t1::n469(s, out),
		_ => false,
	}
}

/// Words ending -υν.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ων.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3093);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3095);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3096);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 3097);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5057);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6437);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3098);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3099);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5059);
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
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3100);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5060);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6438);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7100);
													s.is_empty()
												}
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
							super::push(out, 3101);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5061);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6439);
											s.is_empty()
										}
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
					super::push(out, 1483);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3103);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3104);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5063);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6440);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 3105);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3109);
							match s {
								[] => true,
								['θ', 'ν', 'ε', s @ ..] => {
									super::push(out, 5065);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 5066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1487);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3110);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5067);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6441);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7101);
													s.is_empty()
												}
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
							super::push(out, 3111);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3112);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5069);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 6442);
											s.is_empty()
										}
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
					super::push(out, 1491);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3113);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3115);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5071);
									s.is_empty()
								}
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
			super::push(out, 471);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3117);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3118);
							match s {
								[] => true,
								['α', s @ ..] => super::t1::n5105(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t1::n1506(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t1::n478(s, out),
		['ι', s @ ..] => super::t1::n479(s, out),
		['κ', s @ ..] => super::t1::n480(s, out),
		['λ', s @ ..] => super::t1::n481(s, out),
		['μ', s @ ..] => super::t1::n482(s, out),
		['ν', s @ ..] => super::t1::n483(s, out),
		['ξ', s @ ..] => super::t1::n484(s, out),
		['π', s @ ..] => super::t1::n485(s, out),
		['ρ', s @ ..] => super::t1::n486(s, out),
		['σ', s @ ..] => super::t1::n487(s, out),
		['τ', s @ ..] => super::t1::n488(s, out),
		['υ', s @ ..] => super::t1::n489(s, out),
		['φ', s @ ..] => super::t1::n490(s, out),
		['χ', s @ ..] => super::t1::n491(s, out),
		_ => false,
	}
}

/// Words ending -ρωναξ.
pub(crate) fn n3242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3215);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 5117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παξ.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3217);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6458);
									s.is_empty()
								}
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
			super::push(out, 1580);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3218);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ραξ.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		['ι', 'ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 1583);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1584);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3220);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6459);
									s.is_empty()
								}
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
			super::push(out, 1585);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σαξ.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1586);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3222);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταξ.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1588);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3224);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1589);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 5126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαξ.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1590);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1591);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γξ.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3229);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 3230);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3231);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5128);
									s.is_empty()
								}
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
			super::push(out, 500);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 3233);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3234);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3235);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5131);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5132);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6460);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3236);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3237);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3238);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5135);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6461);
											s.is_empty()
										}
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
			super::push(out, 501);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3241);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 3242);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5137);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6462);
											s.is_empty()
										}
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
					super::push(out, 1603);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εξ.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηξ.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1607);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1608);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3246);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5138);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6463);
											s.is_empty()
										}
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
			super::push(out, 507);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1609);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3247);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5139);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6464);
											s.is_empty()
										}
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
		['φ', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
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

/// Words ending -ιξ.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1611);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3248);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3250);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 3251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1614);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3254);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5142);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6465);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7104);
													match s {
														[] => true,
														['υ', 'σ', '\u{1}', s @ ..] => super::t1::n7366(s, out),
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
				['ω', s @ ..] => {
					super::push(out, 1616);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3255);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5143);
									s.is_empty()
								}
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
			super::push(out, 513);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3256);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3263);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5145);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 5146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3266);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5147);
									match s {
										[] => true,
										['λ', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3267);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3268);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3269);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3270);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5151);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6467);
											s.is_empty()
										}
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
		['φ', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3271);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οξ.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρξ.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υξ.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1641);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3273);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3274);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3275);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1644);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3276);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5155);
									s.is_empty()
								}
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
			super::push(out, 527);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3277);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5156);
									s.is_empty()
								}
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
			super::push(out, 528);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3279);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5157);
									s.is_empty()
								}
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
			super::push(out, 529);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3281);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5158);
									s.is_empty()
								}
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
			super::push(out, 530);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1649);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3285);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3286);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3288);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωξ.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χερμαστηρ.
pub(crate) fn n7367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7325);
	s.is_empty()
}

/// Words ending -γασ.
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3336);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5190);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6479);
									s.is_empty()
								}
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
			super::push(out, 1720);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3337);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1721);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1722);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1723);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εασ.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3341);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5192);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5193);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6481);
									s.is_empty()
								}
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

/// Words ending -ιασ.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['δ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3342);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3344);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3345);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5194);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6482);
									s.is_empty()
								}
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
			super::push(out, 1728);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['κ', 'σ', s @ ..] => {
			super::push(out, 1730);
			s.is_empty()
		}
		['λ', 'ι', s @ ..] => {
			super::push(out, 1731);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1732);
			s.is_empty()
		}
		['ν', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3346);
					s.is_empty()
				}
				['υ', 'ο', s @ ..] => {
					super::push(out, 3347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 1733);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3348);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5195);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6483);
									s.is_empty()
								}
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
			super::push(out, 1734);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 3349);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1736);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3351);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5196);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6484);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7110);
											s.is_empty()
										}
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
					super::push(out, 3352);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1737);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1738);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1739);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 1740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κασ.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1741);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3354);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5197);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6485);
									s.is_empty()
								}
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
			super::push(out, 1742);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3355);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5198);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6486);
									s.is_empty()
								}
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
			super::push(out, 1743);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3357);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λασ.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1746);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3359);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5200);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3360);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5201);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1747);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				['υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1748);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3365);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5204);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6489);
									s.is_empty()
								}
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

/// Words ending -μασ.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νασ.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1754);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3370);
					s.is_empty()
				}
				['ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 3371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πασ.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 580);
	match s {
		[] => true,
		['α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1756);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1758);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3374);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3375);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5209);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6490);
									s.is_empty()
								}
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

/// Words ending -ρασ.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 581);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5211);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5212);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6491);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5213);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6492);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7112);
											s.is_empty()
										}
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
					super::push(out, 3379);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1762);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3383);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1763);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1766);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3387);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1767);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σασ.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 582);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 3389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τασ.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3390);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1770);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3391);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1772);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1773);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1774);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3393);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5220);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6493);
									s.is_empty()
								}
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

/// Words ending -υασ.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1775);
			s.is_empty()
		}
		['δ', 'ν', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3395);
					match s {
						[] => true,
						['α', 'μ', s @ ..] => {
							super::push(out, 5221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φασ.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 585);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3396);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 3398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3399);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5223);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6494);
									s.is_empty()
								}
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

/// Words ending -χασ.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1780);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3400);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5224);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6495);
									s.is_empty()
								}
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

/// Words ending -εσ.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3401);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5225);
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
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3402);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3403);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5226);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6496);
											s.is_empty()
										}
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
		['δ', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3404);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3405);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5228);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 6497);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3406);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 5229);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6498);
											s.is_empty()
										}
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
			super::push(out, 589);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3407);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5230);
									s.is_empty()
								}
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
			super::push(out, 590);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3408);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 5231);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3409);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5232);
									s.is_empty()
								}
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
			super::push(out, 592);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3411);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5233);
									s.is_empty()
								}
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
			super::push(out, 594);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3412);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3413);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5235);
									s.is_empty()
								}
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
			super::push(out, 595);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ησ.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3415);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1795);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3417);
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
				['γ', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3419);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5237);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6499);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7113);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 5238);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6500);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7114);
													s.is_empty()
												}
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
					super::push(out, 1800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', 'α', 'μ', 'ο', 'ρ', 'ω', '\u{1}', s @ ..] => {
			super::push(out, 599);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3420);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3423);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5240);
									s.is_empty()
								}
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
			super::push(out, 602);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3426);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5242);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6501);
											s.is_empty()
										}
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
					super::push(out, 1811);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1812);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3429);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5243);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6502);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7115);
													match s {
														[] => true,
														['β', 'ι', 'β', '\u{1}', s @ ..] => super::t1::n7368(s, out),
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
		['ν', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3430);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 5244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 3432);
							match s {
								[] => true,
								['π', '\u{1}', s @ ..] => {
									super::push(out, 5246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3434);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1816);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3435);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5248);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 6503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3437);
							match s {
								[] => true,
								['ο', s @ ..] => super::t1::n5283(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t1::n1830(s, out),
				['υ', s @ ..] => super::t1::n1831(s, out),
				['χ', s @ ..] => super::t1::n1832(s, out),
				['ω', s @ ..] => super::t1::n1833(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n609(s, out),
		['π', s @ ..] => super::t1::n610(s, out),
		['ρ', s @ ..] => super::t1::n611(s, out),
		['σ', s @ ..] => super::t1::n612(s, out),
		['τ', s @ ..] => super::t1::n613(s, out),
		['υ', s @ ..] => super::t1::n614(s, out),
		['χ', s @ ..] => super::t1::n615(s, out),
		_ => false,
	}
}

/// Words ending -ισ.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 611);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3523);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3524);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5344);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6562);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3525);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5345);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 6563);
											s.is_empty()
										}
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
					super::push(out, 1850);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3527);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3528);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5346);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6564);
											s.is_empty()
										}
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
					super::push(out, 1852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3529);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 3530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3531);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1854);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3532);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5348);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6565);
											s.is_empty()
										}
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
		['γ', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3533);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5349);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6566);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7145);
													s.is_empty()
												}
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
				['λ', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3534);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 5350);
									s.is_empty()
								}
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
			super::push(out, 615);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3535);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5351);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6567);
											s.is_empty()
										}
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
					super::push(out, 1858);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3537);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5352);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6568);
											s.is_empty()
										}
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
					super::push(out, 1860);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1862);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3541);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 5354);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 6569);
											s.is_empty()
										}
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
					super::push(out, 1865);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1867);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 3544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3545);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 5355);
									s.is_empty()
								}
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
			super::push(out, 619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1874);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3546);
							match s {
								[] => true,
								['α', s @ ..] => super::t1::n5390(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => super::t1::n1887(s, out),
				['ι', s @ ..] => super::t1::n1888(s, out),
				['ο', s @ ..] => super::t1::n1889(s, out),
				['ρ', s @ ..] => super::t1::n1890(s, out),
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n626(s, out),
		['μ', s @ ..] => super::t1::n627(s, out),
		['ν', s @ ..] => super::t1::n628(s, out),
		['ξ', s @ ..] => super::t1::n629(s, out),
		['ο', s @ ..] => super::t1::n630(s, out),
		['π', s @ ..] => super::t1::n631(s, out),
		['ρ', s @ ..] => super::t1::n632(s, out),
		['σ', s @ ..] => super::t1::n633(s, out),
		['τ', s @ ..] => super::t1::n634(s, out),
		['φ', s @ ..] => super::t1::n635(s, out),
		['χ', s @ ..] => super::t1::n636(s, out),
		['ψ', s @ ..] => super::t1::n637(s, out),
		['ω', s @ ..] => super::t1::n638(s, out),
		['ϝ', s @ ..] => super::t1::n639(s, out),
		['ῥ', s @ ..] => super::t1::n640(s, out),
		_ => false,
	}
}

/// Words ending -λσ.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νσ.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1977);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3754);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1978);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3755);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5514);
									s.is_empty()
								}
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

/// Words ending -οσ.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1980);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1981);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1983);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3756);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5515);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 6644);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6645);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3757);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3758);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1984);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3759);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6646);
											s.is_empty()
										}
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
					super::push(out, 1985);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1986);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3761);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5518);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6647);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 3762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3763);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5520);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6648);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7177);
													s.is_empty()
												}
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
							super::push(out, 3764);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3765);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3766);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5523);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6649);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 5524);
									s.is_empty()
								}
								['ρ', 'κ', s @ ..] => {
									super::push(out, 5525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3768);
							s.is_empty()
						}
						['ρ', 'τ', s @ ..] => {
							super::push(out, 3769);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 3770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1990);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3771);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3772);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1991);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3773);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5528);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6650);
											s.is_empty()
										}
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
					super::push(out, 1992);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1993);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3775);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => super::t1::n645(s, out),
		['δ', s @ ..] => super::t1::n646(s, out),
		['ε', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2016);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3848);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3849);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2018);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3850);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2019);
					match s {
						[] => true,
						['ι', 'ρ', 'ε', 'π', s @ ..] => {
							super::push(out, 3852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 3854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ο', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['α', 'π', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2024);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3855);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 5594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t1::n649(s, out),
		['ι', s @ ..] => super::t1::n650(s, out),
		['κ', s @ ..] => super::t1::n651(s, out),
		['λ', s @ ..] => super::t1::n652(s, out),
		['μ', s @ ..] => super::t1::n653(s, out),
		['ν', s @ ..] => super::t1::n654(s, out),
		['ξ', s @ ..] => super::t1::n655(s, out),
		['ο', s @ ..] => super::t1::n656(s, out),
		['π', s @ ..] => super::t1::n657(s, out),
		['ρ', s @ ..] => super::t1::n658(s, out),
		['σ', s @ ..] => super::t1::n659(s, out),
		['τ', s @ ..] => super::t1::n660(s, out),
		['υ', s @ ..] => super::t1::n661(s, out),
		['φ', s @ ..] => super::t1::n662(s, out),
		['χ', s @ ..] => super::t2::n663(s, out),
		['ψ', s @ ..] => super::t2::n664(s, out),
		['ω', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υσ.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2205);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4464);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6155);
									s.is_empty()
								}
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
			super::push(out, 664);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2206);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4467);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6156);
									s.is_empty()
								}
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
			super::push(out, 666);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2209);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4468);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6157);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6979);
											s.is_empty()
										}
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
					super::push(out, 2210);
					match s {
						[] => true,
						['λ', 'η', s @ ..] => {
							super::push(out, 4469);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6158);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6980);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', 'ω', s @ ..] => {
							super::push(out, 4470);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6159);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6981);
											s.is_empty()
										}
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
					super::push(out, 2211);
					match s {
						[] => true,
						['ο', 'ι', s @ ..] => {
							super::push(out, 4471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2214);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2215);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4473);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2216);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['υ', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2218);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4475);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6161);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4477);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4478);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6163);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4480);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6164);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6984);
											s.is_empty()
										}
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
					super::push(out, 2224);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4483);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4485);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6166);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6985);
											s.is_empty()
										}
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
					super::push(out, 2230);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2231);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4488);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2232);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['ρ', s @ ..] => super::t2::n4520(s, out),
						_ => false,
					}
				}
				['κ', s @ ..] => super::t2::n2248(s, out),
				_ => false,
			}
		}
		['ξ', s @ ..] => super::t2::n679(s, out),
		['ο', s @ ..] => super::t2::n680(s, out),
		['π', s @ ..] => super::t2::n681(s, out),
		['ρ', s @ ..] => super::t2::n682(s, out),
		['σ', s @ ..] => super::t2::n683(s, out),
		['τ', s @ ..] => super::t2::n684(s, out),
		['φ', s @ ..] => super::t2::n685(s, out),
		['χ', s @ ..] => super::t2::n686(s, out),
		_ => false,
	}
}

/// Words ending -ωσ.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2269);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4526);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6195);
									s.is_empty()
								}
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
			super::push(out, 684);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2270);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2271);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 4529);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6197);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7000);
											s.is_empty()
										}
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
			super::push(out, 686);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 4531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2273);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4532);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6198);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7001);
											s.is_empty()
										}
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
		['θ', s @ ..] => {
			super::push(out, 687);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2274);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2275);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2279);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4537);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6199);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7002);
											s.is_empty()
										}
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
					super::push(out, 2280);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4538);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4539);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 4540);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2285);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρχαιολογια.
pub(crate) fn n7337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7295);
	s.is_empty()
}

/// Words ending -υσιολογια.
pub(crate) fn n7338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7296);
	s.is_empty()
}

/// The word ομολογια.
pub(crate) fn n7339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7297);
	s.is_empty()
}

/// Words ending -χρονολογια.
pub(crate) fn n7340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7298);
	s.is_empty()
}

/// Words ending -στρολογια.
pub(crate) fn n7341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7299);
	s.is_empty()
}

/// Words ending -εωρολογια.
pub(crate) fn n7342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7300);
	s.is_empty()
}

/// Words ending -θεουργια.
pub(crate) fn n4609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4578);
	s.is_empty()
}

