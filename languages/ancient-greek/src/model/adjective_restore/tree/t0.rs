use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['η', 'ρ', 'ε', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				['τ', 'ν', 'α', 'σ', 'ι', 'ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['δ', 'σ', 'ο', 'κ', 'ι', 'λ', 'η', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['ν', 'ε', 'μ', 'ο', 'υ', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n4(s, out),
		['ξ', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 17);
					match s {
						[] => true,
						['π', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 55);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 18);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 56);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 57);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 58);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['ι', 'ε', 'χ', 'ο', 'ρ', 'ε', 'τ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 20);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 21);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 59);
							match s {
								[] => true,
								['η', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 60);
							match s {
								[] => true,
								['α', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 258);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 259);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 260);
									match s {
										[] => true,
										['ε', 'λ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 715);
											s.is_empty()
										}
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
		['σ', s @ ..] => super::t0::n7(s, out),
		['ψ', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 28);
					s.is_empty()
				}
				['ω', 'υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['ρ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 30);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 14);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['ι', 'ρ', 'η', 'τ', 'υ', 'λ', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 31);
					s.is_empty()
				}
				['ν', 'ε', 'μ', 'ο', 'υ', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 32);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t0::n16(s, out),
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 22);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 261);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 716);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 717);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2350);
													s.is_empty()
												}
												['σ', 'ι', 'ε', '\u{1}', s @ ..] => {
													super::push(out, 2351);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'β', '\u{1}', s @ ..] => {
							super::push(out, 262);
							s.is_empty()
						}
						['μ', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 263);
							s.is_empty()
						}
						['ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 63);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 64);
					match s {
						[] => true,
						['α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 266);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 267);
							s.is_empty()
						}
						['ι', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['α', 'δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 269);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['η', 'τ', 'π', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 271);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 272);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 719);
									match s {
										[] => true,
										['μ', s @ ..] => {
											match s {
												[] => true,
												['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
													super::push(out, 2352);
													s.is_empty()
												}
												['ο', 'π', 'υ', '\u{1}', s @ ..] => {
													super::push(out, 2353);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 720);
									match s {
										[] => true,
										['κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1521);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', 'τ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 274);
							match s {
								[] => true,
								['δ', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 721);
									s.is_empty()
								}
								['ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 275);
							s.is_empty()
						}
						['η', 'μ', 'τ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => super::t0::n276(s, out),
						['ι', s @ ..] => super::t0::n277(s, out),
						['ρ', 'α', 'π', 'υ', '\u{1}', s @ ..] => super::t0::n278(s, out),
						['υ', s @ ..] => super::t0::n279(s, out),
						['ω', s @ ..] => super::t0::n280(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n68(s, out),
				['ρ', s @ ..] => super::t0::n69(s, out),
				['σ', s @ ..] => super::t0::n70(s, out),
				['τ', s @ ..] => super::t0::n71(s, out),
				['ψ', s @ ..] => super::t0::n72(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n23(s, out),
		['ι', s @ ..] => super::t0::n24(s, out),
		['ο', s @ ..] => super::t0::n25(s, out),
		['υ', s @ ..] => super::t0::n26(s, out),
		['ω', s @ ..] => super::t0::n27(s, out),
		_ => false,
	}
}

/// Words ending -ων.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['δ', 'α', 'ρ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 144);
					s.is_empty()
				}
				['λ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 145);
					s.is_empty()
				}
				['ο', 'β', '\u{1}', s @ ..] => {
					super::push(out, 146);
					s.is_empty()
				}
				['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 148);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1449);
									match s {
										[] => true,
										['ρ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 2342);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 1450);
									s.is_empty()
								}
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
			super::push(out, 35);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 578);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 579);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['ν', 'υ', s @ ..] => {
											match s {
												[] => true,
												['ξ', '\u{1}', s @ ..] => {
													super::push(out, 3005);
													s.is_empty()
												}
												['σ', '\u{1}', s @ ..] => {
													super::push(out, 3006);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ξ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 150);
					match s {
						[] => true,
						['λ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 581);
							s.is_empty()
						}
						['τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 151);
					s.is_empty()
				}
				['σ', 'ι', 'μ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 152);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 153);
					match s {
						[] => true,
						['π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 583);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 584);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', 'π', 'ο', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						['ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						['τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 157);
					match s {
						[] => true,
						['ρ', 'ε', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 588);
							s.is_empty()
						}
						['υ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 158);
					s.is_empty()
				}
				['κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 159);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 160);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 590);
							s.is_empty()
						}
						['ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 591);
							s.is_empty()
						}
						['ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 592);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 593);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1452);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 1453);
									match s {
										[] => true,
										['ε', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 2344);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 161);
					match s {
						[] => true,
						['α', 'ρ', 'ι', 'ο', 'κ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 594);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 595);
							s.is_empty()
						}
						['υ', 'ρ', 'τ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 162);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 38);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 164);
					match s {
						[] => true,
						['ι', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 597);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 598);
							match s {
								[] => true,
								['γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1455);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 1456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 599);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 600);
							match s {
								[] => true,
								['κ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', 'υ', 'σ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 602);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 603);
							match s {
								[] => true,
								['δ', '\u{1}', s @ ..] => {
									super::push(out, 1458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 604);
							match s {
								[] => true,
								['ο', 'π', 'α', 'ρ', 'δ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 606);
							match s {
								[] => true,
								['ι', 'τ', 'τ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 607);
							s.is_empty()
						}
						['ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 608);
							s.is_empty()
						}
						['π', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['ρ', s @ ..] => super::t0::n616(s, out),
						['τ', s @ ..] => super::t0::n617(s, out),
						['χ', s @ ..] => super::t0::n618(s, out),
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n39(s, out),
		['ι', s @ ..] => super::t0::n40(s, out),
		['κ', s @ ..] => super::t0::n41(s, out),
		['λ', s @ ..] => super::t0::n42(s, out),
		['μ', s @ ..] => super::t0::n43(s, out),
		['ν', s @ ..] => super::t0::n44(s, out),
		['ξ', s @ ..] => super::t0::n45(s, out),
		['ο', s @ ..] => super::t0::n46(s, out),
		['π', s @ ..] => super::t0::n47(s, out),
		['ρ', s @ ..] => super::t0::n48(s, out),
		['σ', s @ ..] => super::t0::n49(s, out),
		['τ', s @ ..] => super::t0::n50(s, out),
		['υ', s @ ..] => super::t0::n51(s, out),
		['φ', s @ ..] => super::t0::n52(s, out),
		['χ', s @ ..] => super::t0::n53(s, out),
		['ψ', 'ε', 'ρ', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => super::t0::n54(s, out),
		_ => false,
	}
}

/// The word διατμηξασ.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	s.is_empty()
}

/// Words ending -ιξασ.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['μ', 'μ', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word υπαρξασ.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	s.is_empty()
}

/// Words ending -υξασ.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['ν', '\u{1}', s @ ..] => {
			super::push(out, 725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωξασ.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['ι', 'δ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πασ.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['μ', 'υ', s @ ..] => {
			match s {
				[] => true,
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρασ.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['ι', 'ε', s @ ..] => {
			match s {
				[] => true,
				['δ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σασ.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['β', 'ι', 'β', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['σ', 'α', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						['τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'ι', 'ε', 'χ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['ι', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'υ', 'σ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['ι', 'ε', 'μ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['ι', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1534);
							match s {
								[] => true,
								['δ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['ο', 'β', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						['π', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['ε', 'λ', 'ο', 'π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1538);
							match s {
								[] => true,
								['δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['ν', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['ι', 'ε', 'χ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['ω', 'χ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ρ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1544);
							match s {
								[] => true,
								['α', 'τ', s @ ..] => {
									super::push(out, 2356);
									match s {
										[] => true,
										['α', 'κ', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3416);
													s.is_empty()
												}
												['γ', 'ε', '\u{1}', s @ ..] => {
													super::push(out, 3417);
													s.is_empty()
												}
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
				['χ', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['υ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['μ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 754);
					match s {
						[] => true,
						['α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['χ', '\u{1}', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'α', 'μ', 'η', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				['χ', 'ι', 'ε', 'τ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'μ', 'ο', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['δ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 762);
					match s {
						[] => true,
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['κ', 'α', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['ι', 'ξ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τασ.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['σ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						['τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						['θ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψασ.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['α', 'λ', 'β', '\u{1}', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['ρ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['υ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ησ.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['β', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1559);
									s.is_empty()
								}
								['ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 1560);
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
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 297);
					match s {
						[] => true,
						['ρ', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['π', 'υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['ε', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['α', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['α', 'ρ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 302);
					match s {
						[] => true,
						['ρ', 'φ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 1561);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1562);
									match s {
										[] => true,
										['ρ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2357);
											s.is_empty()
										}
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
					super::push(out, 303);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 304);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 775);
							match s {
								[] => true,
								['ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 776);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['γ', 'υ', 'ρ', 'ε', 'τ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 2358);
											s.is_empty()
										}
										['δ', s @ ..] => {
											super::push(out, 2359);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 2360);
											match s {
												[] => true,
												['ρ', 'υ', 'θ', '\u{1}', s @ ..] => {
													super::push(out, 3011);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 2361);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2362);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3012);
													s.is_empty()
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2363);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 2364);
											match s {
												[] => true,
												['υ', s @ ..] => {
													match s {
														[] => true,
														['γ', 'γ', 'ο', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => super::t0::n3507(s, out),
														['τ', s @ ..] => super::t0::n3508(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 2365);
											match s {
												[] => true,
												['γ', 'ι', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 3013);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => super::t0::n2426(s, out),
										['π', 'ω', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2367);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2368);
											match s {
												[] => true,
												['ε', 'τ', 'σ', 'α', '\u{1}', s @ ..] => super::t0::n3099(s, out),
												['υ', 'θ', '\u{1}', s @ ..] => super::t0::n3100(s, out),
												_ => false,
											}
										}
										['τ', s @ ..] => super::t0::n2429(s, out),
										['υ', s @ ..] => super::t0::n2430(s, out),
										['φ', 'α', 'κ', 'σ', '\u{1}', s @ ..] => super::t0::n2431(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => super::t0::n308(s, out),
				['ω', s @ ..] => super::t0::n309(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n77(s, out),
		['θ', s @ ..] => super::t0::n78(s, out),
		['ι', 'γ', 'υ', '\u{1}', s @ ..] => super::t0::n79(s, out),
		['κ', s @ ..] => super::t0::n80(s, out),
		['λ', s @ ..] => super::t0::n81(s, out),
		['ν', s @ ..] => super::t0::n82(s, out),
		['π', s @ ..] => super::t0::n83(s, out),
		['ρ', s @ ..] => super::t0::n84(s, out),
		['τ', s @ ..] => super::t0::n85(s, out),
		['υ', s @ ..] => super::t0::n86(s, out),
		['φ', s @ ..] => super::t0::n87(s, out),
		['χ', s @ ..] => super::t0::n88(s, out),
		_ => false,
	}
}

/// Words ending -ισ.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['π', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['λ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 835);
							s.is_empty()
						}
						['ν', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 837);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 838);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 839);
							match s {
								[] => true,
								['ι', 'ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1598);
									s.is_empty()
								}
								['κ', 'ι', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 1599);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 1600);
									match s {
										[] => true,
										['β', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 2395);
											s.is_empty()
										}
										['κ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 2396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 1601);
									match s {
										[] => true,
										['υ', 'θ', 'ν', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2397);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1602);
									match s {
										[] => true,
										['ι', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 2398);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'ν', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 1603);
									s.is_empty()
								}
								['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1604);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1605);
									s.is_empty()
								}
								['τ', 'ρ', 'ο', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 840);
							match s {
								[] => true,
								['ρ', 'κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 842);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 843);
							match s {
								[] => true,
								['α', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 1608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 844);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1609);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 2399);
											s.is_empty()
										}
										['δ', 'ε', 'κ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2400);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2401);
											match s {
												[] => true,
												['λ', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3509(s, out),
														['λ', 'α', 'γ', 'α', '\u{1}', s @ ..] => super::t0::n3510(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', 'γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2402);
											s.is_empty()
										}
										['μ', 'ε', 'ρ', 'κ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2403);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', 'θ', 'χ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 1611);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1612);
									match s {
										[] => true,
										['ε', 'π', '\u{1}', s @ ..] => {
											super::push(out, 2404);
											s.is_empty()
										}
										['κ', s @ ..] => super::t0::n2465(s, out),
										['ο', 'ρ', 'θ', 'α', '\u{1}', s @ ..] => super::t0::n2466(s, out),
										['χ', 'ι', 'ε', 'τ', 'ο', 'π', 'α', '\u{1}', s @ ..] => super::t0::n2467(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => super::t0::n858(s, out),
						['φ', s @ ..] => super::t0::n859(s, out),
						['χ', s @ ..] => super::t0::n860(s, out),
						['ω', s @ ..] => super::t0::n861(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n355(s, out),
				['λ', s @ ..] => super::t0::n356(s, out),
				['μ', 'α', 'δ', '\u{1}', s @ ..] => super::t0::n357(s, out),
				['ν', 'α', '\u{1}', s @ ..] => super::t0::n358(s, out),
				['ο', s @ ..] => super::t0::n359(s, out),
				['π', 'υ', 'τ', '\u{1}', s @ ..] => super::t0::n360(s, out),
				['φ', s @ ..] => super::t0::n361(s, out),
				_ => false,
			}
		}
		['κ', 'λ', 'α', 'ν', 'α', '\u{1}', s @ ..] => super::t0::n91(s, out),
		['ν', 'ο', 'υ', 'κ', 'λ', 'α', '\u{1}', s @ ..] => super::t0::n92(s, out),
		['π', s @ ..] => super::t0::n93(s, out),
		['ρ', s @ ..] => super::t0::n94(s, out),
		_ => false,
	}
}

/// Words ending -οσ.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 861);
							match s {
								[] => true,
								['λ', 'υ', 'σ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2418);
											s.is_empty()
										}
										['ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 2419);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2420);
											match s {
												[] => true,
												['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
													super::push(out, 3027);
													s.is_empty()
												}
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
				['ι', s @ ..] => {
					super::push(out, 365);
					s.is_empty()
				}
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 862);
							match s {
								[] => true,
								['θ', '\u{1}', s @ ..] => {
									super::push(out, 1630);
									s.is_empty()
								}
								['κ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1631);
									s.is_empty()
								}
								['λ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1632);
									s.is_empty()
								}
								['ρ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'κ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 368);
					match s {
						[] => true,
						['φ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['ο', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						['β', '\u{1}', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 371);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 868);
							match s {
								[] => true,
								['τ', 'ο', 'κ', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 869);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1635);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1636);
									match s {
										[] => true,
										['θ', 'ν', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2421);
											s.is_empty()
										}
										['ι', 'α', 'ρ', 'θ', 'α', 'λ', 'ι', 'τ', 'κ', 'υ', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 2422);
											s.is_empty()
										}
										['κ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2423);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2424);
											match s {
												[] => true,
												['α', 'π', 'υ', 'ῥ', '\u{1}', s @ ..] => {
													super::push(out, 3028);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2425);
											s.is_empty()
										}
										['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 2426);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 1637);
									match s {
										[] => true,
										['θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 2427);
											s.is_empty()
										}
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
					super::push(out, 372);
					match s {
						[] => true,
						['α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 871);
							match s {
								[] => true,
								['θ', 'φ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 2428);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2429);
											s.is_empty()
										}
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
					super::push(out, 373);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 374);
					match s {
						[] => true,
						['λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 375);
					match s {
						[] => true,
						['λ', s @ ..] => super::t0::n886(s, out),
						['ψ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n380(s, out),
				['υ', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => super::t0::n382(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n98(s, out),
		['ε', s @ ..] => super::t0::n99(s, out),
		['ζ', s @ ..] => super::t0::n100(s, out),
		['θ', s @ ..] => super::t0::n101(s, out),
		['ι', s @ ..] => super::t0::n102(s, out),
		['κ', s @ ..] => super::t0::n103(s, out),
		['λ', s @ ..] => super::t0::n104(s, out),
		['μ', s @ ..] => super::t0::n105(s, out),
		['ν', s @ ..] => super::t0::n106(s, out),
		['ξ', s @ ..] => super::t0::n107(s, out),
		['ο', s @ ..] => super::t0::n108(s, out),
		['π', s @ ..] => super::t0::n109(s, out),
		['ρ', s @ ..] => super::t0::n110(s, out),
		['σ', s @ ..] => super::t0::n111(s, out),
		['τ', s @ ..] => super::t0::n112(s, out),
		['υ', s @ ..] => super::t0::n113(s, out),
		['φ', s @ ..] => super::t0::n114(s, out),
		['χ', s @ ..] => super::t0::n115(s, out),
		['ψ', s @ ..] => super::t0::n116(s, out),
		['ω', s @ ..] => super::t0::n117(s, out),
		_ => false,
	}
}

/// Words ending -υσ.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['α', 'ρ', 'π', '\u{1}', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['γ', 'ι', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', '\u{1}', s @ ..] => {
			super::push(out, 121);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				['ι', '\u{1}', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['ι', 'υ', s @ ..] => {
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 2333);
									s.is_empty()
								}
								['λ', 'ο', 'π', '\u{1}', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['υ', 'λ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['β', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', 'ι', 'ρ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['γ', 'ι', 'ε', 'μ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['ι', 'ε', 'δ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'β', '\u{1}', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				['κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1433);
							match s {
								[] => true,
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2337);
									s.is_empty()
								}
								['φ', 'ρ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'α', 'λ', 'π', '\u{1}', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['φ', '\u{1}', s @ ..] => {
			super::push(out, 131);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					match s {
						[] => true,
						['β', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1435);
							s.is_empty()
						}
						['τ', '\u{1}', s @ ..] => {
							super::push(out, 1436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσ.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['ι', 'ε', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['λ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				['ρ', 'χ', 'ο', 'ι', 'ξ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', 'ι', 'ε', 'α', '\u{1}', s @ ..] => {
			super::push(out, 136);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 137);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['ν', 'ο', 'π', 'ε', 'π', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				['υ', 'λ', 'η', 'λ', 'ε', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'δ', 'ε', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 139);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						['δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1439);
							match s {
								[] => true,
								['η', 'ο', 'β', 'ε', 'β', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1440);
							s.is_empty()
						}
						['π', 'α', 'γ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1442);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['σ', 'ε', 'θ', s @ ..] => {
									super::push(out, 2340);
									match s {
										[] => true,
										['α', 'κ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3004);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 1444);
							match s {
								[] => true,
								['σ', 'ε', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['ε', 'π', 'α', 'ρ', 'ε', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['λ', 'ε', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['ν', 'γ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						['τ', 'π', 'ε', 'π', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['α', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['σ', 'ε', 'φ', 'α', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριζων.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['β', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1461);
			s.is_empty()
		}
		['ε', 'τ', 'ω', 'ε', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 1462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τιζων.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['π', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['χ', '\u{1}', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'υ', 'θ', 'ε', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χιζων.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['α', 'ν', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θων.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['π', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['ι', 'σ', 'ο', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιων.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['β', '\u{1}', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['η', '\u{1}', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['κ', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['ξ', 'α', 'λ', 'λ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'χ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 179);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1468);
							s.is_empty()
						}
						['π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				['α', '\u{1}', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['π', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1472);
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
				['α', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['λ', 'π', 'α', 'τ', 'ν', 'ο', 'τ', 'α', 'κ', 'ε', 'ι', 'α', 'κ', 'α', 'τ', 'ν', 'ο', 'κ', 'η', 'μ', 'ο', 'δ', 'β', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κων.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				['ν', '\u{1}', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['ο', 'ι', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['υ', 'α', 'φ', 'ι', 'π', '\u{1}', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['ν', 'γ', 'ι', 'γ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', 'ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λων.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						['ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['γ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['β', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['γ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3007);
											s.is_empty()
										}
										['ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3008);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1483);
							match s {
								[] => true,
								['σ', s @ ..] => {
									match s {
										[] => true,
										['ι', 'π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 3009);
											s.is_empty()
										}
										['ο', 'π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3010);
											s.is_empty()
										}
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
			super::push(out, 200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μων.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['τ', '\u{1}', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['λ', 'ο', 'π', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['ι', 'σ', 'η', 'ν', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				['χ', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['δ', 'ο', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['κ', 'α', 'κ', 'σ', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', '\u{1}', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['η', 'λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				['ο', 'κ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νων.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['μ', 'α', 'λ', 'α', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['ν', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ρ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				['φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['χ', 'γ', 'υ', 'τ', 'σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['μ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1489);
							match s {
								[] => true,
								['η', 'σ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 2348);
									s.is_empty()
								}
								['υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['π', '\u{1}', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['κ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', 'α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['σ', 'ρ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξων.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οων.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['ν', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πων.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['ε', 'π', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρων.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['θ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['π', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['γ', 'η', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 681);
					match s {
						[] => true,
						['θ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', 'χ', 'α', 'ν', 'α', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σων.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['ι', 'ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['λ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['α', 'μ', 'υ', 'α', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', 'α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['θ', '\u{1}', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						['λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						['μ', '\u{1}', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						['ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'α', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				['υ', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['τ', 'ι', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -των.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['π', 'υ', 'ν', 'ο', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['ε', 'γ', 'υ', 'τ', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['φ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['θ', '\u{1}', s @ ..] => {
							super::push(out, 1506);
							s.is_empty()
						}
						['λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['π', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		['σ', 'ι', 'β', 'υ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υων.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['λ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['ο', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['ι', 'σ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						['υ', 'ο', 'β', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'υ', 'δ', 'ν', 'ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				['π', 'α', 'ρ', 'ε', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['ο', 'γ', 'α', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						['υ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', '\u{1}', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['κ', '\u{1}', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['ρ', 'κ', 'α', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φων.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['α', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χων.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['μ', 'υ', 'α', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						['π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['ρ', 'α', 'π', 'σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επιτρεψων.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// The word στρογγυλοειδησ.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3418);
	s.is_empty()
}

/// Words ending -τυλοειδησ.
pub(crate) fn n3508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3419);
	match s {
		[] => true,
		['κ', 'α', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 3594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νοειδησ.
pub(crate) fn n2426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2366);
	match s {
		[] => true,
		['ε', 'δ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3014);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 3015);
			s.is_empty()
		}
		['ι', 'α', 'τ', 'υ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3016);
			s.is_empty()
		}
		['χ', 'α', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3017);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αστεροειδησ.
pub(crate) fn n3099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3018);
	s.is_empty()
}

/// The word θυροειδησ.
pub(crate) fn n3100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3019);
	s.is_empty()
}

/// Words ending -τοειδησ.
pub(crate) fn n2429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2369);
	match s {
		[] => true,
		['σ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 3020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υοειδησ.
pub(crate) fn n2430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2370);
	match s {
		[] => true,
		['θ', 'χ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 3021);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word σκαφοειδησ.
pub(crate) fn n2431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2371);
	s.is_empty()
}

/// Words ending -υδησ.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['ε', 'ψ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωδησ.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['μ', 'α', 'ρ', 'υ', 'θ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 779);
			s.is_empty()
		}
		['θ', 'υ', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 780);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['η', 'τ', 'η', 'λ', 'η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'π', 'υ', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 782);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['α', 'δ', 'ο', 'ε', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['α', 'γ', 'υ', 'ρ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				['χ', 'α', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 786);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ε', 'ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		['χ', 'α', 'ρ', 'α', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εησ.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['δ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 307);
			s.is_empty()
		}
		['λ', 'κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θησ.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'ε', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word υγιησ.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	s.is_empty()
}

/// Words ending -κησ.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['η', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		['λ', 'α', 'ρ', 'ε', 'τ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['δ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1576);
							s.is_empty()
						}
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λησ.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['θ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1578);
							match s {
								[] => true,
								['σ', 'υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['ι', 'σ', 'υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1579);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1580);
							match s {
								[] => true,
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['ψ', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', 'α', 'κ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νησ.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['μ', 'ο', 'λ', 'α', 'γ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1583);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2378);
									s.is_empty()
								}
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1584);
							match s {
								[] => true,
								['ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1585);
							match s {
								[] => true,
								['κ', 'ι', 'τ', 'ο', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2381);
									s.is_empty()
								}
								['λ', 'υ', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 2382);
									s.is_empty()
								}
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
			super::push(out, 325);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 1587);
							match s {
								[] => true,
								['υ', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 2384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1588);
							match s {
								[] => true,
								['α', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1589);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2386);
									match s {
										[] => true,
										['ν', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 3022);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2387);
									match s {
										[] => true,
										['π', 'υ', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3023);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								['υ', 'ρ', 'ο', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['δ', 'ι', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['ρ', 'κ', 'ι', 'λ', 'ι', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', 'ι', 'α', 'ρ', 'κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πησ.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['π', 'υ', 'ρ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'λ', 'κ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['λ', 'α', 'θ', 'σ', 'υ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['ρ', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρησ.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['β', 'ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['π', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['μ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['τ', 'ο', 'λ', 'κ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τησ.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['β', 'ο', 'κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['κ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 2392);
									match s {
										[] => true,
										['α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3024);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2393);
									match s {
										[] => true,
										['τ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3025);
											s.is_empty()
										}
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
			super::push(out, 338);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['τ', 'ν', 'ο', 'κ', 'α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 825);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1593);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ο', 'τ', 'ν', 'ο', 'κ', 'α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υησ.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['φ', s @ ..] => {
			match s {
				[] => true,
				['ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['ι', 'δ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						['λ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φησ.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['δ', 'ο', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χησ.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['α', 'μ', 'ε', 'ρ', 'γ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'σ', 'υ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word λιασθεισ.
pub(crate) fn n3509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3420);
	s.is_empty()
}

/// The word αγαλλιασθεισ.
pub(crate) fn n3510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3421);
	s.is_empty()
}

/// Words ending -κισθεισ.
pub(crate) fn n2465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2405);
	match s {
		[] => true,
		['ι', 'ο', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
			super::push(out, 3026);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αθροισθεισ.
pub(crate) fn n2466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2406);
	s.is_empty()
}

/// The word αποτειχισθεισ.
pub(crate) fn n2467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2407);
	s.is_empty()
}

/// Words ending -υθεισ.
pub(crate) fn n858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['ν', 'ν', 'α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2409);
					s.is_empty()
				}
				['ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2410);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φθεισ.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	s.is_empty()
}

/// Words ending -χθεισ.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 847);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				['τ', '\u{1}', s @ ..] => {
					super::push(out, 2412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', 'α', 'λ', 'π', 'μ', 'ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1616);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['ν', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωθεισ.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 848);
	match s {
		[] => true,
		['ι', 'ξ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1619);
			s.is_empty()
		}
		['ν', 'μ', 'υ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 1620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιεισ.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	s.is_empty()
}

/// Words ending -λεισ.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	s.is_empty()
}

/// The word δαμεισ.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	s.is_empty()
}

/// The word ανεισ.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	s.is_empty()
}

/// Words ending -οεισ.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['ρ', 'κ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['α', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['α', 'σ', 'υ', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['ι', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['ε', 'μ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['α', 'μ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['ρ', 'κ', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word τυπεισ.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	s.is_empty()
}

/// Words ending -φεισ.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['ρ', 'γ', s @ ..] => {
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2414);
							s.is_empty()
						}
						['γ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2415);
							s.is_empty()
						}
						['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2416);
							s.is_empty()
						}
						['ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αναλκισ.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	s.is_empty()
}

/// The word αλκυονισ.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	s.is_empty()
}

/// Words ending -πισ.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['σ', 'α', 'ρ', 'ε', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρισ.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['α', 'χ', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['τ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λογοσ.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 873);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				['ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ξ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2434);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2435);
					match s {
						[] => true,
						['υ', 'τ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['τ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ργοσ.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				['θ', 'η', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1644);
					match s {
						[] => true,
						['ι', 'ρ', 'τ', 'σ', 'α', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 877);
			match s {
				[] => true,
				['τ', 'σ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['ι', 'σ', 'ε', 'λ', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωγοσ.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						['π', 'π', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['τ', 'ι', 'λ', 'π', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δοσ.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['α', 'λ', 'ε', 'κ', 'σ', 'υ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['π', 'ι', 'π', 'ε', 'λ', 'η', 'λ', 'λ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['ο', 'α', '\u{1}', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['π', 'σ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2446);
									s.is_empty()
								}
								['ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 2447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εοσ.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['ι', 'τ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['ζ', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['ι', 'ν', 'ι', 'ο', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['β', 'ρ', 'α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						['ζ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						['ρ', 'ρ', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						['σ', 'ρ', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						['τ', 'π', 'ε', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['α', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['τ', 'κ', 'ε', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', 'δ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['υ', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['ρ', 'ω', 'μ', 'ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						['τ', 'ι', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ρ', 'κ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['ε', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						['ω', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['ο', 'δ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'α', 'ι', 'β', '\u{1}', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2448);
									s.is_empty()
								}
								['ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2449);
									s.is_empty()
								}
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

/// Words ending -ζοσ.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['ρ', 'ο', 'ρ', 'υ', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θοσ.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['λ', 'ο', 'ν', 'ο', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				['ρ', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['ο', 'τ', 'π', 'ι', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['ν', '\u{1}', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['λ', 'ο', 'κ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2450);
									s.is_empty()
								}
								['ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', 'ο', 'ρ', 'ρ', 'ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιοσ.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 400);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['ρ', 'ο', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['υ', 'λ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						['υ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['μ', 'ο', 'τ', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['τ', 'α', 'ε', 'ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['ρ', 'ι', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 918);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['β', 'μ', 'υ', s @ ..] => {
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											super::push(out, 3032);
											s.is_empty()
										}
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 3033);
											s.is_empty()
										}
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
					super::push(out, 919);
					match s {
						[] => true,
						['γ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						['υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 920);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1679);
							match s {
								[] => true,
								['θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2453);
									s.is_empty()
								}
								['κ', 'υ', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								['λ', 'ι', 'τ', 'υ', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2455);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2456);
									match s {
										[] => true,
										['υ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3034);
											s.is_empty()
										}
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
					super::push(out, 921);
					match s {
						[] => true,
						['ο', 'ρ', 'τ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1682);
							match s {
								[] => true,
								['τ', 'σ', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 2457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						['ι', 'ε', 'ζ', 'α', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						['ο', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1686);
							match s {
								[] => true,
								['κ', 'ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2458);
									s.is_empty()
								}
								['ο', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1687);
							s.is_empty()
						}
						['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1688);
							s.is_empty()
						}
						['ν', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1690);
							match s {
								[] => true,
								['λ', '\u{1}', s @ ..] => {
									super::push(out, 2460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['μ', 'υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						['υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						['υ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						['φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1698);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1699);
							match s {
								[] => true,
								['π', 'υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', '\u{1}', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['χ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n409(s, out),
		['ζ', 'α', 'β', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 406);
			s.is_empty()
		}
		['η', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['ι', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 949);
					match s {
						[] => true,
						['ι', 'ρ', 'ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						['υ', 'κ', 'α', 'ζ', '\u{1}', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ο', '\u{1}', s @ ..] => super::t0::n964(s, out),
				['σ', 'ι', 'μ', '\u{1}', s @ ..] => super::t0::n965(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n413(s, out),
		['λ', s @ ..] => super::t0::n414(s, out),
		['μ', s @ ..] => super::t0::n415(s, out),
		['ν', s @ ..] => super::t0::n416(s, out),
		['ξ', s @ ..] => super::t0::n417(s, out),
		['ο', s @ ..] => super::t0::n418(s, out),
		['π', s @ ..] => super::t0::n419(s, out),
		['ρ', s @ ..] => super::t0::n420(s, out),
		['σ', s @ ..] => super::t0::n421(s, out),
		['τ', s @ ..] => super::t0::n422(s, out),
		['υ', s @ ..] => super::t0::n423(s, out),
		['φ', s @ ..] => super::t0::n424(s, out),
		['χ', s @ ..] => super::t0::n425(s, out),
		_ => false,
	}
}

/// Words ending -κοσ.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				['δ', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['θ', 'λ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1793);
							match s {
								[] => true,
								['ρ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						['ζ', 'υ', 'ρ', 'β', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1796);
							match s {
								[] => true,
								['υ', 'φ', 'μ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1797);
							match s {
								[] => true,
								['α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['δ', 'ν', 'ο', 'χ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 2498);
									s.is_empty()
								}
								['η', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 2499);
									s.is_empty()
								}
								['θ', 'ι', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2500);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2501);
									match s {
										[] => true,
										['κ', '\u{1}', s @ ..] => {
											super::push(out, 3043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['ρ', 'τ', 'σ', 'ο', 'κ', 'α', 'λ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n427(s, out),
		['λ', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['α', 'χ', 'ο', 'ρ', 'η', 'δ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				['υ', 'ο', 'γ', 'υ', 'ζ', '\u{1}', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['λ', 'π', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['ε', 'λ', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['ο', 'θ', 'ν', 'α', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'η', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λοσ.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['ι', 'α', 'δ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						['ν', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', 'α', 'μ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['ο', 'κ', 'α', 'λ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1891);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2616);
									s.is_empty()
								}
								['θ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1892);
							match s {
								[] => true,
								['ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['α', 'π', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						['ε', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1896);
							match s {
								[] => true,
								['κ', s @ ..] => {
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3138);
											s.is_empty()
										}
										['ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3139);
											s.is_empty()
										}
										['ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3140);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 3141);
											match s {
												[] => true,
												['ξ', 'ο', '\u{1}', s @ ..] => {
													super::push(out, 3461);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'σ', 'α', 'μ', 'ο', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['α', 'π', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ε', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2619);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 2620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1898);
							match s {
								[] => true,
								['ζ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1899);
							match s {
								[] => true,
								['ν', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1901);
							s.is_empty()
						}
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', 'ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				['ι', 'ν', 'ε', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				['ο', 'ρ', 'υ', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['α', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['υ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['δ', 'ε', 'π', 'υ', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['ε', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['ι', 'ο', 'π', 'μ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['ι', 'σ', 'υ', 'λ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', '\u{1}', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['υ', 'κ', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['α', 'ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1907);
							s.is_empty()
						}
						['ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['μ', 'υ', 'σ', 'α', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['π', 'υ', 'κ', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 1911);
							s.is_empty()
						}
						['ο', 'β', 'ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1913);
							match s {
								[] => true,
								['α', 'τ', 'ν', 'ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2623);
									s.is_empty()
								}
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
			super::push(out, 435);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1914);
							match s {
								[] => true,
								['ζ', '\u{1}', s @ ..] => {
									super::push(out, 2624);
									s.is_empty()
								}
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1915);
							match s {
								[] => true,
								['φ', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'ρ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['ε', 'π', 'υ', 'ν', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				['ο', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n441(s, out),
		['φ', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['χ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μοσ.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1929);
							s.is_empty()
						}
						['ο', 'π', 'υ', 'θ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 1930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				['ν', 'κ', 'α', 'τ', 'κ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['σ', 'α', 'λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', 'υ', 'ρ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 444);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['μ', 'ο', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1932);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['α', 'π', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['δ', 'υ', 'κ', 'ε', 'ρ', 'γ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1938);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1939);
							match s {
								[] => true,
								['ν', 'α', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 2630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1941);
							match s {
								[] => true,
								['ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1942);
							match s {
								[] => true,
								['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', 'ρ', 'τ', 'υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1115);
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
					super::push(out, 1116);
					match s {
						[] => true,
						['θ', 'φ', 'ο', 'ι', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', 'α', 'ρ', 'γ', 'ο', 'λ', 'η', 'λ', 'λ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 447);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['ι', 'ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1947);
							s.is_empty()
						}
						['ο', 'σ', 'σ', 'ι', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						['υ', 'τ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['δ', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2633);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1952);
							match s {
								[] => true,
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1953);
							match s {
								[] => true,
								['λ', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
								['χ', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1954);
							match s {
								[] => true,
								['ο', 'δ', 'ι', 'λ', 'α', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 2638);
									s.is_empty()
								}
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2639);
									s.is_empty()
								}
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
			super::push(out, 449);
			match s {
				[] => true,
				['ε', s @ ..] => {
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1956);
							match s {
								[] => true,
								['ο', 'γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2640);
									s.is_empty()
								}
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
			super::push(out, 450);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2641);
									s.is_empty()
								}
								['κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2642);
									s.is_empty()
								}
								['μ', 'ι', 'ρ', 'β', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2643);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						['υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['ω', s @ ..] => super::t0::n2008(s, out),
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t0::n455(s, out),
		_ => false,
	}
}

/// Words ending -νοσ.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['β', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2648);
									s.is_empty()
								}
								['ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['ε', 'κ', 'υ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						['ι', 'τ', 'υ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				['θ', 'ι', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				['ι', 'ρ', 'τ', 'κ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						['ρ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['π', 'ο', 'ρ', 'ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						['ε', 'τ', 'σ', 'ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['α', 'π', 'α', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['κ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['π', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n459(s, out),
		['η', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['α', 'γ', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['ε', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['α', 'κ', 'ι', 'ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n461(s, out),
		['κ', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['ο', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['υ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['α', 'ρ', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['ρ', 'κ', 'ο', 'λ', 'η', 'ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						['ι', 'ψ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['φ', 'ι', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['π', 'ε', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'μ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1182);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['θ', 'ο', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						['χ', 'γ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2082);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ν', 'ν', 'α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 3296);
											s.is_empty()
										}
										['φ', 'λ', 'ε', 'δ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3297);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2084);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
								['ρ', 'α', 'β', '\u{1}', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['ι', 'α', 'ι', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2086);
							match s {
								[] => true,
								['σ', 'σ', 'α', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 2828);
									s.is_empty()
								}
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
			super::push(out, 462);
			match s {
				[] => true,
				['α', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						['ρ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		['τ', 'ι', 'λ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'υ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['σ', 'ι', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['ι', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['υ', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['ι', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t0::n472(s, out),
		_ => false,
	}
}

/// Words ending -ξοσ.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['α', 'ρ', 'ο', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2831);
									s.is_empty()
								}
								['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 2832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'π', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οοσ.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['υ', 'κ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'η', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['π', s @ ..] => {
					match s {
						[] => true,
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['υ', 'δ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['θ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['κ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2101);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['θ', 'α', 'β', '\u{1}', s @ ..] => {
									super::push(out, 2833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'κ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'σ', 'ο', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποσ.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['α', 'δ', 'ο', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['ε', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['ο', 'λ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2104);
							s.is_empty()
						}
						['ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['μ', 'ο', 'π', 'ο', 'χ', 'υ', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2106);
							match s {
								[] => true,
								['λ', 'υ', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 2834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2107);
							match s {
								[] => true,
								['ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								['ο', 'ν', 'α', 'ρ', 'υ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', 'λ', 'ε', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2108);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2837);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2838);
									match s {
										[] => true,
										['ι', s @ ..] => {
											match s {
												[] => true,
												['α', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
													super::push(out, 3548);
													s.is_empty()
												}
												['λ', 'ο', 'δ', '\u{1}', s @ ..] => {
													super::push(out, 3549);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2839);
									match s {
										[] => true,
										['λ', 'ο', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3298);
											s.is_empty()
										}
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
		['π', 'ι', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['σ', 'ι', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['ι', 'σ', 'υ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['ε', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['ο', 'ρ', 'π', s @ ..] => {
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2840);
									match s {
										[] => true,
										['κ', 'α', 'ρ', 'ε', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 3299);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'ε', 'μ', 'υ', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				['υ', 'λ', 'β', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροσ.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['ι', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2114);
							match s {
								[] => true,
								['α', 'β', 'ο', s @ ..] => {
									match s {
										[] => true,
										['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
											super::push(out, 3300);
											s.is_empty()
										}
										['ξ', 'ι', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 3301);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				['ε', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				['θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['σ', 'υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2842);
									s.is_empty()
								}
								['ψ', '\u{1}', s @ ..] => {
									super::push(out, 2843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				['ψ', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['α', 'δ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['α', 'ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1236);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2121);
							s.is_empty()
						}
						['ε', 'δ', 'υ', 'θ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['α', 'λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['α', 'λ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						['υ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['α', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t0::n2176(s, out),
						['κ', 'σ', '\u{1}', s @ ..] => super::t0::n2177(s, out),
						_ => false,
					}
				}
				['κ', 'υ', 'λ', 'γ', '\u{1}', s @ ..] => super::t0::n1261(s, out),
				['λ', s @ ..] => super::t0::n1262(s, out),
				['μ', s @ ..] => super::t0::n1263(s, out),
				['ν', 'α', 'φ', '\u{1}', s @ ..] => super::t0::n1264(s, out),
				['τ', s @ ..] => super::t0::n1265(s, out),
				['υ', 'ρ', 'κ', '\u{1}', s @ ..] => super::t0::n1266(s, out),
				['φ', s @ ..] => super::t0::n1267(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n499(s, out),
		['θ', s @ ..] => super::t0::n500(s, out),
		['ι', s @ ..] => super::t0::n501(s, out),
		['κ', s @ ..] => super::t0::n502(s, out),
		['ο', s @ ..] => super::t0::n503(s, out),
		['π', s @ ..] => super::t0::n504(s, out),
		['ρ', s @ ..] => super::t0::n505(s, out),
		['τ', s @ ..] => super::t0::n506(s, out),
		['υ', s @ ..] => super::t0::n507(s, out),
		['φ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => super::t0::n508(s, out),
		['χ', s @ ..] => super::t0::n509(s, out),
		['ω', s @ ..] => super::t0::n510(s, out),
		_ => false,
	}
}

/// Words ending -σοσ.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1312);
					s.is_empty()
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['υ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['λ', 'γ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2897);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2898);
									match s {
										[] => true,
										['π', 'υ', '\u{1}', s @ ..] => {
											super::push(out, 3355);
											s.is_empty()
										}
										['ρ', 'κ', 'ι', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3356);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2899);
									match s {
										[] => true,
										['ν', 'α', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 3357);
											s.is_empty()
										}
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
			super::push(out, 511);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['ν', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['χ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2900);
									match s {
										[] => true,
										['ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 3358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2901);
									s.is_empty()
								}
								['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2902);
									s.is_empty()
								}
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

/// Words ending -τοσ.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2191);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2903);
									s.is_empty()
								}
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'μ', 'υ', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['η', 'κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['α', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2193);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2194);
							match s {
								[] => true,
								['α', 'ρ', 'γ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3359);
											s.is_empty()
										}
										['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 3360);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', 'α', 'σ', 'ι', 'ρ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2195);
							s.is_empty()
						}
						['υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2197);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2198);
							match s {
								[] => true,
								['δ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						['ο', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						['π', 'ο', 'ι', 'μ', 'η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						['ο', s @ ..] => super::t0::n2256(s, out),
						['σ', s @ ..] => {
							super::push(out, 2206);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2915);
									match s {
										[] => true,
										['τ', 'α', 'κ', 'α', '\u{1}', s @ ..] => super::t0::n3459(s, out),
										_ => false,
									}
								}
								['ε', s @ ..] => super::t0::n2994(s, out),
								['ι', s @ ..] => super::t0::n2995(s, out),
								_ => false,
							}
						}
						['υ', s @ ..] => super::t0::n2258(s, out),
						['ω', s @ ..] => super::t0::n2259(s, out),
						_ => false,
					}
				}
				['φ', s @ ..] => super::t0::n1348(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n517(s, out),
		['η', s @ ..] => super::t0::n518(s, out),
		['ι', s @ ..] => super::t0::n519(s, out),
		['κ', s @ ..] => super::t0::n520(s, out),
		['λ', s @ ..] => super::t0::n521(s, out),
		['ν', s @ ..] => super::t0::n522(s, out),
		['ο', s @ ..] => super::t0::n523(s, out),
		['π', s @ ..] => super::t0::n524(s, out),
		['ρ', s @ ..] => super::t0::n525(s, out),
		['σ', s @ ..] => super::t0::n526(s, out),
		['τ', s @ ..] => super::t0::n527(s, out),
		['υ', s @ ..] => super::t0::n528(s, out),
		['ω', s @ ..] => super::t0::n529(s, out),
		_ => false,
	}
}

/// Words ending -υοσ.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['γ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'κ', 'σ', 'α', 'ρ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φοσ.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['δ', 'ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				['ν', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1405);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 2992);
									s.is_empty()
								}
								['ι', 'π', 'ε', 'δ', 'υ', 'ε', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 2993);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2994);
									match s {
										[] => true,
										['ρ', 'α', 'π', 'υ', 'ῥ', '\u{1}', s @ ..] => {
											super::push(out, 3407);
											s.is_empty()
										}
										['σ', 'σ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
											super::push(out, 3408);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['ρ', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', 'ε', 'δ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['λ', 'α', 'τ', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1408);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1409);
					match s {
						[] => true,
						['ο', 'ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2995);
									match s {
										[] => true,
										['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
											super::push(out, 3409);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 3410);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 3411);
											match s {
												[] => true,
												['α', 'π', 'ι', 'λ', '\u{1}', s @ ..] => {
													super::push(out, 3593);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 3412);
											s.is_empty()
										}
										['χ', 'υ', 'ψ', '\u{1}', s @ ..] => {
											super::push(out, 3413);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2996);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2997);
									match s {
										[] => true,
										['λ', 'ο', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3414);
											s.is_empty()
										}
										['ο', 'β', '\u{1}', s @ ..] => {
											super::push(out, 3415);
											s.is_empty()
										}
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
					super::push(out, 1410);
					match s {
						[] => true,
						['ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['ο', 'μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2998);
									s.is_empty()
								}
								['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3000);
									s.is_empty()
								}
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
			super::push(out, 534);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['ρ', 'κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χοσ.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['β', 'μ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2329);
							match s {
								[] => true,
								['τ', 'σ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', 'υ', 'ρ', 'ρ', 'υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 536);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				['τ', 'σ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['δ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				['ι', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 539);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['ν', 'ω', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						['ν', 'α', 'ρ', 'υ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['π', 'ι', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['λ', 'α', 'γ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3002);
									s.is_empty()
								}
								['ρ', 'κ', 'ι', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3003);
									s.is_empty()
								}
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
			super::push(out, 541);
			match s {
				[] => true,
				['τ', 'π', 'ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψοσ.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['ι', 'δ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 542);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωοσ.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				['ι', 'ε', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 545);
			s.is_empty()
		}
		['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ειοσ.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['γ', 'α', 'ρ', 'α', 'μ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['π', 'ι', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'ι', 'σ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 939);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 940);
			match s {
				[] => true,
				['ι', 'α', 'ν', 'υ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['υ', 'ρ', 'η', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['ε', 'τ', 'ο', 'ρ', 'ι', 'ε', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				['ι', 'σ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['α', 'φ', 'ο', 'τ', 'σ', 'ι', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['ο', 'δ', 'ι', 'λ', 'ε', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['φ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['β', s @ ..] => {
					match s {
						[] => true,
						['α', 'τ', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', 'ω', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				['δ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['χ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['κ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						['ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', 'κ', 'α', 'β', '\u{1}', s @ ..] => {
			super::push(out, 947);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ορθιοσ.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	s.is_empty()
}

/// The word μισθιοσ.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	s.is_empty()
}

/// Words ending -κιοσ.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['ο', 'χ', 'ι', 'λ', 'ο', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιοσ.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['τ', 'υ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['υ', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['χ', 'σ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['α', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['σ', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						['τ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1736);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2471);
							s.is_empty()
						}
						['ν', 'ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['υ', 'κ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['ω', 'φ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μιοσ.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['λ', 'α', 'θ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		['ο', 'τ', 'σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['α', 'ρ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νιοσ.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['δ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['μ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['μ', 'α', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['δ', 'ι', 'λ', 'ε', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				['ρ', 'χ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'υ', 'ν', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 973);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				['μ', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				['υ', 'κ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξιοσ.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				['ν', '\u{1}', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οιοσ.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['η', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 977);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['ο', s @ ..] => {
					match s {
						[] => true,
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2476);
							s.is_empty()
						}
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιοσ.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['μ', 'υ', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		['ρ', 'ο', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 983);
			s.is_empty()
		}
		['σ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριοσ.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['κ', s @ ..] => {
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', 'μ', 'ε', 'τ', 'π', 'ε', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 986);
			s.is_empty()
		}
		['γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 987);
			s.is_empty()
		}
		['δ', 'ν', 'ο', 'χ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 988);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1755);
					s.is_empty()
				}
				['μ', 'η', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2480);
							match s {
								[] => true,
								['κ', 'ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2481);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2482);
							match s {
								[] => true,
								['ι', 'ρ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['λ', 'ω', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
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
			super::push(out, 991);
			match s {
				[] => true,
				['ι', 'α', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['ε', 'χ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				['ο', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['α', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				['π', 'μ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σιοσ.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', 'σ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3038);
									s.is_empty()
								}
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['κ', 'α', 'θ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['ι', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						['π', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['ρ', 'α', 'τ', 'ι', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['τ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['δ', 'ο', 'ρ', 'φ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				['ε', 'ρ', 'ε', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['ρ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['κ', 'α', 'ρ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['α', 'λ', 'α', 'θ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2489);
							match s {
								[] => true,
								['ε', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'ν', 'ο', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								['υ', 'α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
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

/// Words ending -τιοσ.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['ε', 'ρ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['ν', 'α', 'θ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				['π', 'ο', 'ρ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2491);
							s.is_empty()
						}
						['ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['υ', 'ο', 'ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1006);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1007);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						['γ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['υ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υιοσ.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	s.is_empty()
}

/// Words ending -φιοσ.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιοσ.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['ι', 'λ', 'ι', 'ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['υ', 'ν', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικοσ.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['κ', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['ε', 'λ', 'ο', 'τ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						['ω', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['α', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				['μ', 'α', 'ρ', 'υ', 'θ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1024);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['η', 'τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['λ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ε', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['θ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3422);
											s.is_empty()
										}
										['μ', 'υ', 'τ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 3423);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 3424);
											s.is_empty()
										}
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
					super::push(out, 1812);
					match s {
						[] => true,
						['υ', 'ο', 'ι', 'μ', 'η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						['π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'π', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1818);
					match s {
						[] => true,
						['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['ζ', 'ι', 'ῥ', '\u{1}', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1819);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['κ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['μ', '\u{1}', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'α', 'χ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1821);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['π', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								['ω', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
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
			super::push(out, 1031);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1822);
					match s {
						[] => true,
						['κ', 'σ', 'α', 'δ', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['κ', 'ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						['τ', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['κ', 'ο', 'κ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['σ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'σ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				['κ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2520);
							match s {
								[] => true,
								['α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 3047);
									s.is_empty()
								}
								['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 3048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						['χ', 'γ', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['ρ', 'δ', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 3049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1830);
					match s {
						[] => true,
						['ν', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', 'υ', 'φ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1831);
					s.is_empty()
				}
				['η', 'δ', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1832);
					s.is_empty()
				}
				['θ', 'υ', 'ῥ', '\u{1}', s @ ..] => {
					super::push(out, 1833);
					s.is_empty()
				}
				['ι', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				['λ', 'α', 'θ', 'φ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				['μ', 'α', 'ρ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 1836);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1837);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2527);
							match s {
								[] => true,
								['ο', 'ρ', 'τ', 'σ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1838);
					match s {
						[] => true,
						['ι', 'τ', 'σ', 'ι', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						['χ', 'η', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['γ', 'γ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['σ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'λ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				['ν', 'α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => super::t0::n1884(s, out),
				['ο', s @ ..] => super::t0::n1885(s, out),
				['υ', 'κ', '\u{1}', s @ ..] => super::t0::n1886(s, out),
				['ω', s @ ..] => super::t0::n1887(s, out),
				_ => false,
			}
		}
		['ξ', s @ ..] => super::t0::n1048(s, out),
		['ο', s @ ..] => super::t0::n1049(s, out),
		['π', s @ ..] => super::t0::n1050(s, out),
		['ρ', s @ ..] => super::t0::n1051(s, out),
		['σ', s @ ..] => super::t0::n1052(s, out),
		['τ', s @ ..] => super::t0::n1053(s, out),
		['υ', s @ ..] => super::t0::n1054(s, out),
		['φ', s @ ..] => super::t0::n1055(s, out),
		['χ', s @ ..] => super::t0::n1056(s, out),
		['ω', s @ ..] => super::t0::n1057(s, out),
		_ => false,
	}
}

/// Words ending -υλοσ.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['κ', 'ο', 'λ', 'υ', 'γ', 'γ', 'ο', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ι', 'λ', 'ο', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1089);
			s.is_empty()
		}
		['κ', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1090);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1091);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['β', 'υ', 'θ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				['ο', 'κ', 'α', 'τ', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['α', 'τ', 'σ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνυμοσ.
pub(crate) fn n2008(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ι', 'λ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2645);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['ε', 'τ', 'σ', 'ι', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2647);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωμοσ.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['μ', 'α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ενοσ.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['μ', s @ ..] => super::t0::n1161(s, out),
		['ξ', s @ ..] => {
			super::push(out, 1146);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1987);
					s.is_empty()
				}
				['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ινοσ.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2708);
							s.is_empty()
						}
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2709);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', 'γ', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1993);
					match s {
						[] => true,
						['β', 'μ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1995);
					match s {
						[] => true,
						['κ', 'κ', 'ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', 'υ', 'λ', 'ο', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1155);
			match s {
				[] => true,
				['ι', 'γ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2002);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2003);
					match s {
						[] => true,
						['α', 'ρ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 2718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', 'ι', 'ῥ', '\u{1}', s @ ..] => {
			super::push(out, 1156);
			s.is_empty()
		}
		['η', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 1157);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1158);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2004);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2005);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						['ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2006);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2722);
							match s {
								[] => true,
								['κ', 'α', 'υ', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								['λ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2723);
							match s {
								[] => true,
								['κ', 'α', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 3253);
									s.is_empty()
								}
								['ψ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'κ', 'ο', 'λ', 'ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['ε', 'κ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2008);
					match s {
						[] => true,
						['β', 'μ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						['δ', 'ι', 'ρ', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 2728);
							match s {
								[] => true,
								['ι', 'μ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2730);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3256);
									match s {
										[] => true,
										['δ', 'ν', 'α', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3545);
											s.is_empty()
										}
										['μ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['θ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								['ο', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['τ', 'σ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 3259);
									s.is_empty()
								}
								['υ', 'τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						['ῥ', '\u{1}', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2009);
					match s {
						[] => true,
						['θ', 'ι', 'π', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2010);
					match s {
						[] => true,
						['ν', 'ι', 'ο', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2734);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2735);
							match s {
								[] => true,
								['υ', 'μ', 'ο', 'ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2012);
					match s {
						[] => true,
						['α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['β', 'μ', 'ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2737);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2738);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2740);
							match s {
								[] => true,
								['α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2741);
							match s {
								[] => true,
								['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
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
			super::push(out, 1160);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						['τ', 'ν', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						['χ', 'ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', 'υ', 'β', '\u{1}', s @ ..] => super::t0::n2062(s, out),
				['ε', s @ ..] => super::t0::n2063(s, out),
				['η', s @ ..] => super::t0::n2064(s, out),
				['ι', s @ ..] => super::t0::n2065(s, out),
				['λ', s @ ..] => super::t0::n2066(s, out),
				['υ', s @ ..] => super::t0::n2067(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n1177(s, out),
		['ν', s @ ..] => super::t0::n1178(s, out),
		['ο', s @ ..] => super::t0::n1179(s, out),
		['π', s @ ..] => super::t0::n1180(s, out),
		['ρ', s @ ..] => super::t1::n1181(s, out),
		['σ', s @ ..] => super::t1::n1182(s, out),
		['τ', s @ ..] => super::t1::n1183(s, out),
		['υ', s @ ..] => super::t1::n1184(s, out),
		['φ', s @ ..] => super::t1::n1185(s, out),
		['χ', s @ ..] => super::t1::n1186(s, out),
		['ψ', s @ ..] => super::t1::n1187(s, out),
		_ => false,
	}
}

/// Words ending -ωνοσ.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2091);
					s.is_empty()
				}
				['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2092);
					s.is_empty()
				}
				['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['ο', 'ρ', 'υ', 'φ', 'ρ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2096);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2097);
					match s {
						[] => true,
						['ε', 'κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						['κ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ιεροσ.
pub(crate) fn n2176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	s.is_empty()
}

/// The word σκιεροσ.
pub(crate) fn n2177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	s.is_empty()
}

/// The word γλυκεροσ.
pub(crate) fn n1261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1242);
	s.is_empty()
}

/// Words ending -λεροσ.
pub(crate) fn n1262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1243);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2130);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεροσ.
pub(crate) fn n1263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1244);
	match s {
		[] => true,
		['ι', 'ε', 'χ', 'σ', 'υ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word φανεροσ.
pub(crate) fn n1264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1245);
	s.is_empty()
}

/// Words ending -τεροσ.
pub(crate) fn n1265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1246);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', 'δ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 3302);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 3303);
							match s {
								[] => true,
								['α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 3304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 2136);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2137);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2846);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2847);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3305);
							match s {
								[] => true,
								['ι', 'β', '\u{1}', s @ ..] => {
									super::push(out, 3551);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3552);
									s.is_empty()
								}
								['λ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3553);
									s.is_empty()
								}
								['τ', 'α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3306);
							match s {
								[] => true,
								['δ', 'η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3555);
									s.is_empty()
								}
								['τ', 'σ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2848);
					match s {
						[] => true,
						['η', 'ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2849);
					match s {
						[] => true,
						['λ', 'ο', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2850);
					match s {
						[] => true,
						['κ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3311);
							match s {
								[] => true,
								['π', 'υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 3557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 3312);
							match s {
								[] => true,
								['υ', 'ρ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 3313);
							match s {
								[] => true,
								['ε', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3314);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3632);
											s.is_empty()
										}
										['μ', '\u{1}', s @ ..] => {
											super::push(out, 3633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3315);
							match s {
								[] => true,
								['μ', 'α', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 3560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3316);
							match s {
								[] => true,
								['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 3317);
							match s {
								[] => true,
								['υ', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 3562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2852);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['π', 'η', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3318);
							s.is_empty()
						}
						['σ', 'ι', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', 'λ', 'ε', 'δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2855);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3320);
							match s {
								[] => true,
								['ι', 'ν', 'ι', 'ο', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 3563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'ι', 'κ', 'ι', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3321);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3322);
							match s {
								[] => true,
								['α', 'υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'κ', 'ω', '\u{1}', s @ ..] => {
					super::push(out, 2856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2140);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['β', 'ι', 'ρ', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3323);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 3324);
							match s {
								[] => true,
								['η', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3325);
							match s {
								[] => true,
								['δ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 3326);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', 'α', 'φ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3328);
							match s {
								[] => true,
								['ο', 'μ', 'ι', 'α', 'δ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'ρ', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						['φ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2859);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3331);
							match s {
								[] => true,
								['α', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', '\u{1}', s @ ..] => {
					super::push(out, 2860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2862);
					match s {
						[] => true,
						['α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2863);
					s.is_empty()
				}
				['χ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n2190(s, out),
		_ => false,
	}
}

/// The word κρυεροσ.
pub(crate) fn n1266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1247);
	s.is_empty()
}

/// Words ending -φεροσ.
pub(crate) fn n1267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1248);
	match s {
		[] => true,
		['υ', 'ρ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2143);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηροσ.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1249);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1250);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['α', 'φ', 'μ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1252);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1254);
			match s {
				[] => true,
				['α', 'χ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1255);
			match s {
				[] => true,
				['υ', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1256);
			match s {
				[] => true,
				['υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1257);
			match s {
				[] => true,
				['α', 'δ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 1258);
			s.is_empty()
		}
		['φ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1259);
			s.is_empty()
		}
		['χ', 'υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θροσ.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['λ', 'ε', 'μ', 'ι', 'ψ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'α', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		['ρ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1264);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1265);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιροσ.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['κ', s @ ..] => {
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1269);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['μ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κροσ.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 2155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2157);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['κ', 'υ', 'λ', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 3347);
									s.is_empty()
								}
								['λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3348);
									s.is_empty()
								}
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
			super::push(out, 1273);
			match s {
				[] => true,
				['λ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οροσ.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1275);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['φ', 'α', 'ι', 'δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2161);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						['τ', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2162);
					match s {
						[] => true,
						['ρ', 'ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2163);
					match s {
						[] => true,
						['κ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						['ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['ο', 'ν', 'ι', 'ῥ', '\u{1}', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2165);
					match s {
						[] => true,
						['ι', 'δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2166);
					match s {
						[] => true,
						['κ', 'ι', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						['ν', 'α', 'φ', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['τ', 'α', 'ν', 'α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2168);
					match s {
						[] => true,
						['γ', 'ι', 'τ', 'σ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						['ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 2889);
							match s {
								[] => true,
								['σ', 'ι', 'π', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 3349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2890);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2891);
							match s {
								[] => true,
								['υ', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2893);
							match s {
								[] => true,
								['α', 'μ', 'ρ', 'ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 3352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['ρ', 'ο', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προσ.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1281);
			s.is_empty()
		}
		['μ', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['ο', 'κ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 1283);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρροσ.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['κ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τροσ.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1288);
			s.is_empty()
		}
		['ι', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1289);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υροσ.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 503);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				['ρ', 'α', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', 'ι', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1295);
			s.is_empty()
		}
		['κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1296);
			s.is_empty()
		}
		['λ', 'ε', 'δ', 'β', '\u{1}', s @ ..] => {
			super::push(out, 1297);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1299);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['ο', 'ρ', 'κ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				['π', 'π', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ο', 'λ', 'β', '\u{1}', s @ ..] => {
			super::push(out, 1302);
			s.is_empty()
		}
		['τ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1303);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1304);
			match s {
				[] => true,
				['α', 'λ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['ε', 'ζ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['ο', 'π', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3353);
									s.is_empty()
								}
								['υ', 'τ', 'α', 'λ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3354);
									s.is_empty()
								}
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
			super::push(out, 1305);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				['σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ελαφροσ.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	s.is_empty()
}

/// Words ending -χροσ.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 505);
	match s {
		[] => true,
		['σ', 'ι', 'λ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 1306);
			s.is_empty()
		}
		['υ', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 1307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωροσ.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 506);
	match s {
		[] => true,
		['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1308);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1309);
			match s {
				[] => true,
				['ι', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2186);
					s.is_empty()
				}
				['υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['α', 'λ', 'α', 'τ', 'ο', 'κ', 'ι', 'δ', 'ο', 'τ', 'ν', 'α', 'φ', 'ο', 'κ', 'υ', 'σ', 'ο', 'τ', 'ι', 'ο', 'φ', 'ο', 'ρ', 'θ', 'ρ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτατοσ.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2906);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2907);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['δ', 'η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3578);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2908);
			match s {
				[] => true,
				['η', 'ψ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2909);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2910);
			match s {
				[] => true,
				['κ', 'ο', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2911);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3366);
					match s {
						[] => true,
						['α', 'κ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3367);
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3368);
					s.is_empty()
				}
				['υ', 'χ', 'σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 2912);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2913);
			s.is_empty()
		}
		['φ', 'λ', 'ε', 'δ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2914);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ακαταστατοσ.
pub(crate) fn n3459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3370);
	s.is_empty()
}

/// Words ending -εστατοσ.
pub(crate) fn n2994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3371);
			match s {
				[] => true,
				['η', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'η', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3372);
			s.is_empty()
		}
		['λ', 'α', 'φ', 'σ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3373);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['μ', 'ι', 'α', 'δ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 3375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιστατοσ.
pub(crate) fn n2995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	s.is_empty()
}

/// Words ending -υτατοσ.
pub(crate) fn n2258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2207);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2918);
			match s {
				[] => true,
				['α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'α', 'ρ', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 2919);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 2920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτατοσ.
pub(crate) fn n2259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	match s {
		[] => true,
		['γ', 'ο', 'λ', 'ο', 'ι', 'ξ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2921);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2922);
			match s {
				[] => true,
				['δ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3377);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3378);
					s.is_empty()
				}
				['ξ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3380);
					match s {
						[] => true,
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3585);
							s.is_empty()
						}
						['τ', 'ο', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2923);
			match s {
				[] => true,
				['α', 'λ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3587);
							s.is_empty()
						}
						['τ', 'ν', 'υ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2924);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2925);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3383);
					match s {
						[] => true,
						['κ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', 'α', 'κ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 2926);
			s.is_empty()
		}
		['π', 'ε', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 2927);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2928);
			match s {
				[] => true,
				['α', 'θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3384);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3385);
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 3590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3386);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3387);
					match s {
						[] => true,
						['γ', 'ρ', 'α', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 3591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 2929);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2931);
			match s {
				[] => true,
				['υ', 'λ', 'γ', 'ο', 'μ', 'α', 'δ', 'ρ', 'α', 'κ', 'ο', 'τ', 'σ', 'ι', 'ρ', 'π', 'ο', 'ν', 'ι', 'μ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φατοσ.
pub(crate) fn n1348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1329);
	match s {
		[] => true,
		['η', 'ν', 'υ', 'δ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 2209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ετοσ.
pub(crate) fn n517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2210);
					match s {
						[] => true,
						['υ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1332);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['ι', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ητοσ.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['β', 'α', 'φ', 'λ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1335);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['η', 'ρ', 'ο', 'χ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', 'ο', 'π', 'υ', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		['θ', 'σ', 'ι', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				['κ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['ω', 'μ', 'α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2222);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				['ν', 'ε', 'γ', 'ο', 'ρ', 'υ', 'φ', 'ρ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1344);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['ε', 'μ', 'ο', 'σ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['ι', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['σ', 'η', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['ι', 'α', 'ρ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', 'α', 'λ', 'η', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 1348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιτοσ.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1349);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['λ', 'κ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'σ', 'υ', 'α', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κτοσ.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['λ', 'α', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['ν', 'ι', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['α', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2942);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2238);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 2945);
							s.is_empty()
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2239);
					match s {
						[] => true,
						['λ', 'π', 'μ', 'ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['λ', 'ε', 'ξ', 'ε', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'φ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['λ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						['φ', 'ι', 'ρ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['π', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['ε', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				['μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['ο', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				['τ', 'σ', 'ο', 'ρ', 'υ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['α', 'φ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λτοσ.
pub(crate) fn n521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	match s {
		[] => true,
		['α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντοσ.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['ι', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['λ', 'α', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2252);
					match s {
						[] => true,
						['χ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3391);
									s.is_empty()
								}
								['ν', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3392);
									s.is_empty()
								}
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

/// Words ending -οτοσ.
pub(crate) fn n523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['ε', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'π', 'α', 'τ', 'ν', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', 'ζ', '\u{1}', s @ ..] => {
			super::push(out, 1364);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πτοσ.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2260);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2262);
					match s {
						[] => true,
						['σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2263);
					match s {
						[] => true,
						['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2264);
					match s {
						[] => true,
						['θ', 'ο', 'μ', 'μ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2958);
							match s {
								[] => true,
								['τ', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'φ', 'μ', 'υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'ν', 'ο', 'π', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1370);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2265);
					match s {
						[] => true,
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				['ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2267);
					s.is_empty()
				}
				['π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτοσ.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1374);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2271);
					match s {
						[] => true,
						['α', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						['φ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'μ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 1375);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1376);
			match s {
				[] => true,
				['κ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2963);
							s.is_empty()
						}
						['γ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στοσ.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['γ', 'ρ', 'ε', 'τ', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2273);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2275);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['ο', 'ν', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2969);
							s.is_empty()
						}
						['ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2970);
							s.is_empty()
						}
						['σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['ῥ', '\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2281);
					match s {
						[] => true,
						['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1379);
			match s {
				[] => true,
				['ρ', 'χ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				['ψ', 'μ', 'ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['α', 'π', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['ο', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2288);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3394);
									s.is_empty()
								}
								['κ', 'α', 'μ', 'ι', 'ε', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3395);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 3396);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3592);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2977);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3397);
									s.is_empty()
								}
								['φ', '\u{1}', s @ ..] => {
									super::push(out, 3398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', 'φ', 'μ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2978);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2979);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 2980);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3400);
									s.is_empty()
								}
								['ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3401);
									s.is_empty()
								}
								['ο', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2982);
							match s {
								[] => true,
								['ρ', 'β', '\u{1}', s @ ..] => {
									super::push(out, 3403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['ι', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2985);
							match s {
								[] => true,
								['ι', 'χ', 'σ', 'ι', 'κ', 'α', 'ρ', 'τ', 'ε', 'τ', 'ο', 'ι', 'ρ', 'υ', 'μ', 'σ', 'ι', 'κ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2986);
							match s {
								[] => true,
								['ο', 'κ', 'α', 'ι', s @ ..] => {
									super::push(out, 3405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'α', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1382);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				['β', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['θ', 'ε', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['π', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['ν', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ττοσ.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 1384);
			s.is_empty()
		}
		['ω', 'λ', 'γ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υτοσ.
pub(crate) fn n528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1386);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1387);
			match s {
				[] => true,
				['ι', 'μ', 'α', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['ν', 'ν', 'α', 'ρ', 'υ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				['ρ', 'τ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1388);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2989);
							match s {
								[] => true,
								['δ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'ι', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1389);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1390);
			s.is_empty()
		}
		['ρ', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1391);
			s.is_empty()
		}
		['τ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1392);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτοσ.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['γ', 'υ', 'ζ', '\u{1}', s @ ..] => {
			super::push(out, 1394);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1395);
			match s {
				[] => true,
				['ι', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'σ', 'ι', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 1396);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['α', 'μ', 'χ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'σ', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['π', 'μ', 'υ', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word τυραννικοσ.
pub(crate) fn n1884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1841);
	s.is_empty()
}

/// Words ending -ονικοσ.
pub(crate) fn n1885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1842);
	match s {
		[] => true,
		['δ', 'ε', 'κ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 2534);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2535);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2536);
			s.is_empty()
		}
		['ρ', 'χ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 2537);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2538);
			match s {
				[] => true,
				['κ', 'ε', 'τ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3052);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word κυνικοσ.
pub(crate) fn n1886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1843);
	s.is_empty()
}

/// Words ending -ωνικοσ.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1844);
	match s {
		[] => true,
		['ι', '\u{1}', s @ ..] => {
			super::push(out, 2539);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2540);
			match s {
				[] => true,
				['α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξικοσ.
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1034);
	s.is_empty()
}

/// Words ending -οικοσ.
pub(crate) fn n1049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1035);
	match s {
		[] => true,
		['β', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1845);
			s.is_empty()
		}
		['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1846);
			s.is_empty()
		}
		['π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1847);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2542);
					s.is_empty()
				}
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2543);
					s.is_empty()
				}
				['ρ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πικοσ.
pub(crate) fn n1050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1036);
	match s {
		[] => true,
		['μ', 'υ', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1850);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['ι', 'θ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2546);
					s.is_empty()
				}
				['ρ', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 1852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρικοσ.
pub(crate) fn n1051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1037);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1853);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 2548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', 'α', 'δ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1854);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['τ', 'σ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['τ', 'κ', 'α', 'μ', 'ι', 'λ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['ε', 'γ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['τ', 'σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2552);
					s.is_empty()
				}
				['φ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1859);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['α', 'ι', 'π', 'π', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2555);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2556);
					s.is_empty()
				}
				['σ', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1862);
			match s {
				[] => true,
				['π', 'ι', 'α', 'λ', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σικοσ.
pub(crate) fn n1052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1038);
	match s {
		[] => true,
		['ι', 'θ', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['φ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						['α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τικοσ.
pub(crate) fn n1053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1039);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2559);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2560);
					match s {
						[] => true,
						['α', 'ρ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 3058);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3425);
									match s {
										[] => true,
										['λ', 'λ', 'α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', 'ε', 'δ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 3059);
							match s {
								[] => true,
								['θ', 'α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3427);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 3428);
									s.is_empty()
								}
								['τ', 'σ', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3060);
							s.is_empty()
						}
						['μ', 'α', 'ρ', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						['ρ', 'ε', 'π', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3062);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3063);
							match s {
								[] => true,
								['ε', 'ν', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 3064);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3431);
									match s {
										[] => true,
										['δ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 3596);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 3432);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 3597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'π', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				['ο', 'ρ', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2563);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3065);
							match s {
								[] => true,
								['θ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['ε', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 3066);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3067);
							match s {
								[] => true,
								['ο', 'τ', 'σ', 'ι', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2565);
					match s {
						[] => true,
						['σ', 'α', s @ ..] => {
							super::push(out, 3068);
							match s {
								[] => true,
								['τ', 'α', 'κ', 'ο', 'π', 'α', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						['ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['η', 'ν', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3072);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3073);
							match s {
								[] => true,
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3074);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3438);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 3439);
									match s {
										[] => true,
										['σ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 3598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3075);
							match s {
								[] => true,
								['π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 3440);
									s.is_empty()
								}
								['τ', 'α', 'μ', 'ο', 'ν', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 3441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3442);
									s.is_empty()
								}
								['β', '\u{1}', s @ ..] => {
									super::push(out, 3443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['ι', 'α', 'φ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['ο', 'λ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3444);
									s.is_empty()
								}
								['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3445);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3446);
									match s {
										[] => true,
										['π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3599);
											s.is_empty()
										}
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
					super::push(out, 2572);
					match s {
						[] => true,
						['α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2574);
					match s {
						[] => true,
						['θ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3080);
							match s {
								[] => true,
								['ε', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2575);
					match s {
						[] => true,
						['ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 3081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['ι', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3082);
							s.is_empty()
						}
						['ο', 'ρ', 'φ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						['ρ', 'ε', 'β', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2577);
					s.is_empty()
				}
				['π', 'ω', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				['ρ', s @ ..] => super::t1::n2645(s, out),
				['τ', s @ ..] => super::t1::n2646(s, out),
				['χ', 'η', 'τ', 'α', 'κ', '\u{1}', s @ ..] => super::t1::n2647(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n1911(s, out),
		['κ', s @ ..] => super::t1::n1912(s, out),
		['λ', s @ ..] => super::t1::n1913(s, out),
		['ν', s @ ..] => super::t1::n1914(s, out),
		['ο', s @ ..] => super::t1::n1915(s, out),
		['π', s @ ..] => super::t1::n1916(s, out),
		['ρ', s @ ..] => super::t1::n1917(s, out),
		['σ', s @ ..] => super::t1::n1918(s, out),
		['τ', s @ ..] => super::t1::n1919(s, out),
		['υ', s @ ..] => super::t1::n1920(s, out),
		['ω', s @ ..] => super::t1::n1921(s, out),
		_ => false,
	}
}

/// Words ending -υικοσ.
pub(crate) fn n1054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1040);
	match s {
		[] => true,
		['θ', 'χ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 1879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φικοσ.
pub(crate) fn n1055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1041);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1880);
			match s {
				[] => true,
				['ρ', 'γ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3133);
							match s {
								[] => true,
								['λ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'ρ', 'ω', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 3134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['σ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', 'γ', 'ο', 'ρ', 'ε', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 1882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χικοσ.
pub(crate) fn n1056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1042);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1883);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3135);
							s.is_empty()
						}
						['ι', 'ρ', 'τ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3136);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3137);
							match s {
								[] => true,
								['η', 'ρ', 'ι', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 1885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωικοσ.
pub(crate) fn n1057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μενοσ.
pub(crate) fn n1161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1145);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2650);
					match s {
						[] => true,
						['ι', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3147);
							match s {
								[] => true,
								['ρ', 'κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3148);
							s.is_empty()
						}
						['υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2652);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3150);
							s.is_empty()
						}
						['ι', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3151);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3152);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3603);
											s.is_empty()
										}
										['π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 3604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 3154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2654);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3155);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 3463);
									s.is_empty()
								}
								['γ', 'ρ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3464);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3465);
									match s {
										[] => true,
										['β', '\u{1}', s @ ..] => {
											super::push(out, 3605);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', 'ε', 'κ', 'σ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 3156);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3467);
									match s {
										[] => true,
										['ο', 'π', s @ ..] => {
											match s {
												[] => true,
												['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
													super::push(out, 3636);
													s.is_empty()
												}
												['ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
													super::push(out, 3637);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3468);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 3469);
									match s {
										[] => true,
										['ε', 'τ', 'σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3606);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3470);
									match s {
										[] => true,
										['σ', 'α', s @ ..] => {
											super::push(out, 3607);
											match s {
												[] => true,
												['τ', 'α', 'κ', '\u{1}', s @ ..] => {
													super::push(out, 3638);
													s.is_empty()
												}
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
							super::push(out, 3157);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3471);
									s.is_empty()
								}
								['ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3472);
									s.is_empty()
								}
								['τ', '\u{1}', s @ ..] => {
									super::push(out, 3473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3158);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['γ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3608);
											s.is_empty()
										}
										['χ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3609);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3159);
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3610);
											match s {
												[] => true,
												['α', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
													super::push(out, 3639);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 3160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2655);
					match s {
						[] => true,
						['σ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3475);
									s.is_empty()
								}
								['θ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3476);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3477);
									match s {
										[] => true,
										['υ', 'ξ', '\u{1}', s @ ..] => {
											super::push(out, 3612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3478);
									s.is_empty()
								}
								['ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1977);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['τ', 'ε', 'τ', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2658);
					match s {
						[] => true,
						['α', 'δ', 'ε', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3162);
							s.is_empty()
						}
						['ε', 'δ', 'ε', 'δ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['θ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['ι', 'φ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t1::n2025(s, out),
		['ι', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['ε', 'κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n2027(s, out),
		['ο', s @ ..] => super::t1::n2028(s, out),
		['ρ', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['α', s @ ..] => super::t1::n2761(s, out),
				_ => false,
			}
		}
		['σ', s @ ..] => super::t1::n2030(s, out),
		['υ', s @ ..] => super::t1::n2031(s, out),
		['ω', s @ ..] => super::t1::n2032(s, out),
		_ => false,
	}
}

/// The word βυβλινοσ.
pub(crate) fn n2062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2016);
	s.is_empty()
}

/// Words ending -ελινοσ.
pub(crate) fn n2063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2017);
	match s {
		[] => true,
		['υ', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 2746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηλινοσ.
pub(crate) fn n2064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2018);
	match s {
		[] => true,
		['μ', 'ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 2747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιλινοσ.
pub(crate) fn n2065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2019);
	match s {
		[] => true,
		['ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 2748);
			s.is_empty()
		}
		['π', '\u{1}', s @ ..] => {
			super::push(out, 2749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λλινοσ.
pub(crate) fn n2066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2020);
	match s {
		[] => true,
		['α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 2750);
			s.is_empty()
		}
		['υ', 'π', 'ρ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλινοσ.
pub(crate) fn n2067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2021);
	match s {
		[] => true,
		['μ', '\u{1}', s @ ..] => {
			super::push(out, 2752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μινοσ.
pub(crate) fn n1177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1161);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2753);
					s.is_empty()
				}
				['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2754);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2755);
					match s {
						[] => true,
						['λ', 'α', 'β', 'ο', 'π', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['κ', 'σ', 'ο', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'κ', 'ω', '\u{1}', s @ ..] => {
			super::push(out, 2023);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3266);
							s.is_empty()
						}
						['ψ', '\u{1}', s @ ..] => {
							super::push(out, 3267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['ε', 'π', 'σ', 'ο', 'ν', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', 'μ', 'α', 'ν', 'ν', 'ι', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νινοσ.
pub(crate) fn n1178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['β', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2758);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2759);
					match s {
						[] => true,
						['α', 'β', 'ο', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'κ', 'γ', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2760);
					s.is_empty()
				}
				['φ', 'α', 'ῥ', '\u{1}', s @ ..] => {
					super::push(out, 2761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['μ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 2762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						['ε', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2764);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2765);
					s.is_empty()
				}
				['π', '\u{1}', s @ ..] => {
					super::push(out, 2766);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2767);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 3271);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['α', 'τ', 'κ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['ι', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 2770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'υ', 'μ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2032);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2771);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οινοσ.
pub(crate) fn n1179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1163);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', 'σ', 'υ', 'θ', 'α', 'β', '\u{1}', s @ ..] => {
			super::push(out, 2036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πινοσ.
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['α', 'ν', 'ι', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2037);
			s.is_empty()
		}
		['π', 'υ', 'τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2038);
			s.is_empty()
		}
		['υ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 2039);
			s.is_empty()
		}
		['ω', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2040);
			s.is_empty()
		}
		_ => false,
	}
}

