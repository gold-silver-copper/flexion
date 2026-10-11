use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 23);
							match s {
								[] => true,
								['λ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 86);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 24);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 88);
									match s {
										[] => true,
										['β', '\u{1}', s @ ..] => {
											super::push(out, 289);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 290);
											s.is_empty()
										}
										_ => false,
									}
								}
								['δ', '\u{1}', s @ ..] => {
									super::push(out, 89);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 90);
									s.is_empty()
								}
								['η', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 91);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 92);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['ε', 'λ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 93);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 94);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 291);
											s.is_empty()
										}
										['π', '\u{1}', s @ ..] => {
											super::push(out, 292);
											s.is_empty()
										}
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
					super::push(out, 7);
					match s {
						[] => true,
						['α', 'λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 27);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => super::t0::n2(s, out),
		['\u{13}', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending - (2 syllables).
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 95);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 29);
					match s {
						[] => true,
						['κ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 96);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 97);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 98);
							match s {
								[] => true,
								['υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 99);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 294);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 594);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 295);
									match s {
										[] => true,
										['α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 296);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 297);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 298);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 596);
											s.is_empty()
										}
										['υ', 'λ', '\u{1}', s @ ..] => {
											super::push(out, 597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 299);
									match s {
										[] => true,
										['μ', 'υ', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 100);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 101);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 102);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 300);
									match s {
										[] => true,
										['ι', 'λ', '\u{1}', s @ ..] => {
											super::push(out, 599);
											s.is_empty()
										}
										['ρ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 600);
											s.is_empty()
										}
										_ => false,
									}
								}
								['δ', s @ ..] => {
									super::push(out, 301);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 601);
											s.is_empty()
										}
										_ => false,
									}
								}
								['θ', s @ ..] => {
									super::push(out, 302);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 303);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 602);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 866);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 304);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 305);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 603);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 306);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 307);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 604);
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
							match s {
								[] => true,
								['ν', 'υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 104);
							match s {
								[] => true,
								['ω', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 105);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 106);
							match s {
								[] => true,
								['α', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 107);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 108);
							match s {
								[] => true,
								['ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 109);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 110);
							s.is_empty()
						}
						['ρ', s @ ..] => super::t0::n111(s, out),
						['σ', s @ ..] => super::t0::n112(s, out),
						['τ', s @ ..] => super::t0::n113(s, out),
						['υ', s @ ..] => super::t0::n114(s, out),
						_ => false,
					}
				}
				['ο', s @ ..] => super::t0::n31(s, out),
				['ρ', s @ ..] => super::t0::n32(s, out),
				['σ', s @ ..] => super::t0::n33(s, out),
				['ω', s @ ..] => super::t0::n34(s, out),
				['ϝ', 'ν', 'ε', 'ξ', '\u{1}', s @ ..] => super::t0::n35(s, out),
				_ => false,
			}
		}
		['ε', 'δ', 'α', 'ι', 'ο', 'τ', '\u{1}', s @ ..] => super::t0::n9(s, out),
		['η', s @ ..] => super::t0::n10(s, out),
		['ν', s @ ..] => super::t0::n11(s, out),
		['ξ', s @ ..] => super::t0::n12(s, out),
		['ρ', s @ ..] => super::t0::n13(s, out),
		['σ', s @ ..] => super::t0::n14(s, out),
		['ψ', s @ ..] => super::t0::n15(s, out),
		_ => false,
	}
}

/// Words ending - (3 syllables).
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n16(s, out),
		['ε', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 66);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 67);
					s.is_empty()
				}
				['κ', s @ ..] => super::t0::n68(s, out),
				['λ', s @ ..] => super::t0::n69(s, out),
				['μ', s @ ..] => super::t0::n70(s, out),
				['ν', s @ ..] => super::t0::n71(s, out),
				['π', s @ ..] => {
					super::push(out, 72);
					s.is_empty()
				}
				['ρ', s @ ..] => super::t0::n73(s, out),
				['τ', s @ ..] => super::t0::n74(s, out),
				['χ', s @ ..] => super::t0::n75(s, out),
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n19(s, out),
		['ξ', s @ ..] => super::t0::n20(s, out),
		['ρ', s @ ..] => super::t0::n21(s, out),
		['σ', s @ ..] => super::t0::n22(s, out),
		_ => false,
	}
}

/// Words ending -ρια (2 syllables).
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	s.is_empty()
}

/// Words ending -σια (2 syllables).
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τια (2 syllables).
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υια (2 syllables).
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'τ', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['λ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['τ', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οα (2 syllables).
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	s.is_empty()
}

/// Words ending -ρα (2 syllables).
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σα (2 syllables).
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 610);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 611);
							s.is_empty()
						}
						['η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 612);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 613);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 867);
									s.is_empty()
								}
								['β', '\u{1}', s @ ..] => {
									super::push(out, 868);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 869);
									s.is_empty()
								}
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 870);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 871);
									match s {
										[] => true,
										['π', '\u{1}', s @ ..] => {
											super::push(out, 1089);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 614);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 872);
									match s {
										[] => true,
										['κ', '\u{1}', s @ ..] => {
											super::push(out, 1090);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωα (2 syllables).
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	s.is_empty()
}

/// The word ξενϝα (2 syllables).
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	s.is_empty()
}

/// The word τοιαδε (2 syllables).
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	s.is_empty()
}

/// Words ending -η (2 syllables).
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 36);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['ι', 'θ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 38);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 39);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['ω', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'ε', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['γ', 'υ', 'ο', 'ζ', '\u{1}', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 126);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 325);
							s.is_empty()
						}
						['ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 127);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 128);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 129);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 132);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 133);
					match s {
						[] => true,
						['τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 134);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 135);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 44);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['δ', 'γ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 47);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 139);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 140);
					match s {
						[] => true,
						['λ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 141);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 142);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 143);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 144);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 332);
							match s {
								[] => true,
								['ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 145);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 333);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 334);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 146);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 335);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 617);
									s.is_empty()
								}
								['τ', '\u{1}', s @ ..] => {
									super::push(out, 618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ν (2 syllables).
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['η', 'ρ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 48);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 49);
			match s {
				[] => true,
				['ζ', 'ι', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 148);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 149);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 337);
							s.is_empty()
						}
						['τ', 'λ', 'ε', 'β', '\u{1}', s @ ..] => {
							super::push(out, 338);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 339);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 619);
									match s {
										[] => true,
										['ρ', 'β', '\u{1}', s @ ..] => {
											super::push(out, 873);
											s.is_empty()
										}
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
					super::push(out, 150);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 340);
							s.is_empty()
						}
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 151);
					match s {
						[] => true,
						['ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 342);
							s.is_empty()
						}
						['ι', 'ε', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 343);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 344);
							match s {
								[] => true,
								['ι', 'α', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 620);
									s.is_empty()
								}
								['υ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 621);
									s.is_empty()
								}
								['ω', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 152);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 345);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 623);
									match s {
										[] => true,
										['θ', '\u{1}', s @ ..] => {
											super::push(out, 874);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', '\u{1}', s @ ..] => {
									super::push(out, 624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 153);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 346);
							match s {
								[] => true,
								['α', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 625);
									s.is_empty()
								}
								['η', '\u{1}', s @ ..] => {
									super::push(out, 626);
									s.is_empty()
								}
								['ι', 'ε', 'ρ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 627);
									s.is_empty()
								}
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

/// Words ending -ξ (2 syllables).
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['υ', 'ν', 'ω', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 50);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρ (2 syllables).
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	s.is_empty()
}

/// Words ending -σ (2 syllables).
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 154);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 155);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 156);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 157);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 348);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 349);
							match s {
								[] => true,
								['ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 158);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 350);
							match s {
								[] => true,
								['ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 159);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 160);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 161);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 162);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 163);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 354);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 631);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 632);
									s.is_empty()
								}
								['υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 166);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 167);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 168);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 169);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 170);
					match s {
						[] => true,
						['τ', 'υ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 171);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 357);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 358);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 359);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 360);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 172);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 173);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 174);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 362);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 175);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 176);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 363);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 635);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 875);
											s.is_empty()
										}
										['ο', 'ε', 'θ', '\u{1}', s @ ..] => {
											super::push(out, 876);
											s.is_empty()
										}
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
					super::push(out, 177);
					match s {
						[] => true,
						['π', 'σ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 364);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 365);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 178);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 179);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 368);
							s.is_empty()
						}
						['ζ', 'ι', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 180);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['θ', 'χ', 'ι', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 181);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 182);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 371);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 637);
									match s {
										[] => true,
										['ρ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 877);
											s.is_empty()
										}
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
			super::push(out, 56);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψ (2 syllables).
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['θ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 183);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -α (3 syllables).
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 185);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 186);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 187);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 373);
							match s {
								[] => true,
								['λ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['ν', 'α', '\u{1}', s @ ..] => {
													super::push(out, 1091);
													s.is_empty()
												}
												['τ', 'α', 'κ', '\u{1}', s @ ..] => {
													super::push(out, 1092);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', 'π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 878);
											s.is_empty()
										}
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
			super::push(out, 59);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 188);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 189);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 190);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 191);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 374);
							match s {
								[] => true,
								['γ', 'α', 'ρ', 'α', 'μ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 375);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 639);
									match s {
										[] => true,
										['λ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 879);
											s.is_empty()
										}
										['ρ', 'β', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 880);
											s.is_empty()
										}
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
					super::push(out, 192);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 376);
							s.is_empty()
						}
						['ι', 'ρ', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 193);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 378);
							match s {
								[] => true,
								['λ', 'λ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 194);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 379);
							match s {
								[] => true,
								['υ', 'ο', 'ρ', 'β', 'ε', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', 'μ', 'ε', 'τ', 'π', 'ε', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 380);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 381);
							match s {
								[] => true,
								['μ', 'η', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 196);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 383);
							match s {
								[] => true,
								['ο', 'ρ', 'ε', s @ ..] => {
									super::push(out, 643);
									match s {
										[] => true,
										['χ', 'α', '\u{1}', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 197);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 198);
					match s {
						[] => true,
						['η', 'τ', 'ο', 'κ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 384);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 385);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 386);
							match s {
								[] => true,
								['η', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 882);
											match s {
												[] => true,
												['ι', 'ν', 'ε', 'ν', '\u{1}', s @ ..] => {
													super::push(out, 1093);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 387);
							match s {
								[] => true,
								['η', 'χ', 'ε', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 644);
									s.is_empty()
								}
								['ο', 'γ', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 645);
									s.is_empty()
								}
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
			super::push(out, 60);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 199);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 388);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 646);
									match s {
										[] => true,
										['ε', 'μ', 'ο', 'ρ', 'ω', 'λ', 'χ', '\u{1}', s @ ..] => {
											super::push(out, 884);
											s.is_empty()
										}
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
			super::push(out, 61);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 200);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 389);
							match s {
								[] => true,
								['α', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n202(s, out),
				['η', s @ ..] => {
					super::push(out, 202);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 396);
							match s {
								[] => true,
								['ι', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 203);
					match s {
						[] => true,
						['ο', 'λ', 'ο', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 204);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 205);
					match s {
						[] => true,
						['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 399);
							s.is_empty()
						}
						['λ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t0::n62(s, out),
		['τ', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['σ', s @ ..] => super::t0::n212(s, out),
				_ => false,
			}
		}
		['ω', 'ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κη (3 syllables).
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 695);
							match s {
								[] => true,
								['ι', 'ο', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 696);
							match s {
								[] => true,
								['α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 419);
					match s {
						[] => true,
						['ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 700);
							s.is_empty()
						}
						['ω', 'π', '\u{1}', s @ ..] => {
							super::push(out, 701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 702);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 980);
									match s {
										[] => true,
										['ο', 'λ', 'ο', '\u{1}', s @ ..] => {
											super::push(out, 1149);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['θ', 'χ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 703);
							s.is_empty()
						}
						['ι', 'ε', 'σ', 'ο', 'λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 704);
							s.is_empty()
						}
						['τ', 'ε', 'μ', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 423);
					match s {
						[] => true,
						['υ', 'φ', s @ ..] => {
							super::push(out, 706);
							match s {
								[] => true,
								['α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 707);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 982);
									match s {
										[] => true,
										['ε', 'β', '\u{1}', s @ ..] => {
											super::push(out, 1150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 708);
							match s {
								[] => true,
								['υ', 'ε', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', 'α', 'ι', s @ ..] => {
							super::push(out, 709);
							match s {
								[] => true,
								['σ', 'υ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 710);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 985);
									match s {
										[] => true,
										['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1151);
											s.is_empty()
										}
										['ι', 'π', 'ε', '\u{1}', s @ ..] => {
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
				['υ', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['κ', 'ε', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λη (3 syllables).
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['γ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['φ', 'υ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μη (3 syllables).
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['σ', 'η', s @ ..] => {
					super::push(out, 430);
					match s {
						[] => true,
						['ρ', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['δ', 'β', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νη (3 syllables).
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 432);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				['ι', 'ρ', 'τ', 'κ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', 'σ', s @ ..] => {
							match s {
								[] => true,
								['η', 'ν', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 986);
									s.is_empty()
								}
								['ι', 'ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 987);
									s.is_empty()
								}
								['υ', 'λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 988);
									s.is_empty()
								}
								['ω', 'λ', 'υ', 'ο', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', 'τ', 'ε', 'τ', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 990);
									s.is_empty()
								}
								['ι', 'α', 'δ', 'ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 992);
									s.is_empty()
								}
								['λ', 'κ', 'ε', 'κ', 'σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 993);
									s.is_empty()
								}
								['μ', 'δ', 'ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 994);
									s.is_empty()
								}
								['ν', 'ε', 'γ', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 713);
							match s {
								[] => true,
								['ε', 'κ', s @ ..] => {
									super::push(out, 996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1153);
											s.is_empty()
										}
										['γ', s @ ..] => {
											super::push(out, 1154);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 1211);
													s.is_empty()
												}
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
							super::push(out, 714);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 997);
									match s {
										[] => true,
										['κ', 'σ', 'ε', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1155);
											s.is_empty()
										}
										['ρ', 'γ', s @ ..] => {
											super::push(out, 1156);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 1212);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['η', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['β', 'ι', 'ε', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 999);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 1000);
									s.is_empty()
								}
								['ζ', 'ι', s @ ..] => {
									match s {
										[] => true,
										['λ', 'α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1157);
											s.is_empty()
										}
										['π', 'λ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 1158);
											s.is_empty()
										}
										_ => false,
									}
								}
								['θ', 'ι', 'ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1001);
									s.is_empty()
								}
								['κ', 'σ', 'ω', 'ν', 'γ', 'ι', 'γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1002);
									s.is_empty()
								}
								['λ', 'υ', 'ο', 'β', '\u{1}', s @ ..] => {
									super::push(out, 1003);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1004);
									match s {
										[] => true,
										['α', 'θ', 'ν', 'υ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 1159);
											s.is_empty()
										}
										['γ', 'ι', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 1160);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 1161);
											match s {
												[] => true,
												['γ', s @ ..] => {
													match s {
														[] => true,
														['ο', 'π', 'α', '\u{1}', s @ ..] => super::t0::n1249(s, out),
														['σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => super::t0::n1250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 1162);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1005);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1006);
									s.is_empty()
								}
								['σ', 'η', s @ ..] => {
									match s {
										[] => true,
										['θ', 'η', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 1163);
											s.is_empty()
										}
										['ν', 'ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 1164);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => super::t0::n1033(s, out),
								['χ', 'ρ', 'ε', '\u{1}', s @ ..] => super::t0::n1034(s, out),
								_ => false,
							}
						}
						['σ', s @ ..] => super::t0::n730(s, out),
						['υ', s @ ..] => super::t0::n731(s, out),
						['ω', s @ ..] => super::t0::n732(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n226(s, out),
		['ι', s @ ..] => super::t0::n227(s, out),
		_ => false,
	}
}

/// Words ending -ρη (3 syllables).
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	s.is_empty()
}

/// Words ending -τη (3 syllables).
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 731);
							match s {
								[] => true,
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 732);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1021);
									match s {
										[] => true,
										['ι', 'α', 'κ', 'κ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 1174);
											s.is_empty()
										}
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
					super::push(out, 450);
					match s {
						[] => true,
						['ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						['υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 451);
					match s {
						[] => true,
						['α', 'θ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 735);
							s.is_empty()
						}
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 453);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						['σ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['ν', 'ε', 'γ', 'ο', 'ρ', 'υ', 'φ', 'ρ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['μ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['α', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['υ', 'κ', 'γ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				['ε', 'β', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 466);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['δ', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 742);
							s.is_empty()
						}
						['λ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 743);
							s.is_empty()
						}
						['ρ', 'α', 'κ', 'α', 'μ', 'ι', 'ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 744);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 746);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1022);
									s.is_empty()
								}
								['γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1023);
									s.is_empty()
								}
								['ι', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 468);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 749);
							match s {
								[] => true,
								['η', 'ο', 'δ', 'γ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 1025);
									s.is_empty()
								}
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
			super::push(out, 236);
			match s {
				[] => true,
				['α', 'κ', 'ι', 'λ', 'η', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['λ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				['ρ', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['ρ', 'π', 'ο', 'τ', 'σ', 'ο', 'κ', 'η', 'μ', 'ο', 'δ', 'β', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				['σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χη (3 syllables).
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	s.is_empty()
}

/// Words ending -ν (3 syllables).
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['ω', 'π', 'ο', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 240);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 477);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 241);
					match s {
						[] => true,
						['α', 'π', 'ε', 'χ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 478);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 479);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 480);
							match s {
								[] => true,
								['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 752);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 753);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 1026);
											s.is_empty()
										}
										['ι', 'σ', 'η', 'ν', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 1027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'κ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 754);
									s.is_empty()
								}
								['χ', 'σ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 481);
							match s {
								[] => true,
								['α', 'δ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 1028);
											match s {
												[] => true,
												['α', 'κ', 'σ', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
													super::push(out, 1175);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 1029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', 'ι', 'ε', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 243);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 482);
							match s {
								[] => true,
								['ι', 'θ', 'α', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 756);
									s.is_empty()
								}
								['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', 'σ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['ι', 'ε', 'γ', 'υ', 'τ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 483);
							s.is_empty()
						}
						['τ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξ (3 syllables).
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρ (3 syllables).
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 487);
							match s {
								[] => true,
								['ψ', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 249);
					match s {
						[] => true,
						['α', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 488);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 489);
							match s {
								[] => true,
								['μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', 'ε', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σ (3 syllables).
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 80);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 251);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 491);
							match s {
								[] => true,
								['α', 'σ', 'υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 252);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 492);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 493);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 494);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 495);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 496);
							match s {
								[] => true,
								['δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 762);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 763);
									match s {
										[] => true,
										['ε', 'χ', 'ι', 'ο', 'τ', 'σ', '\u{1}', s @ ..] => {
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
				['θ', s @ ..] => {
					super::push(out, 253);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 497);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 498);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 764);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1031);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 1176);
													s.is_empty()
												}
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
							super::push(out, 499);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 765);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 1032);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 500);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 501);
							match s {
								[] => true,
								['γ', 'η', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 502);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 769);
									match s {
										[] => true,
										['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
											super::push(out, 1033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 503);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['ε', 'α', 'τ', 'ν', 'ο', 'κ', 'α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 505);
							s.is_empty()
						}
						['υ', 'ο', 'τ', 'ν', 'ο', 'κ', 'α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['α', 'π', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				['π', 'σ', 'α', 'ρ', 'ε', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['α', 'χ', 'υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n83(s, out),
		['υ', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['θ', 'χ', 'ι', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				['λ', 'η', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 286);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 591);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 863);
									match s {
										[] => true,
										['τ', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1087);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ρ', 'α', 'μ', 'ι', 'ε', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 287);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 288);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 593);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 865);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1088);
											match s {
												[] => true,
												['γ', 'ι', 'α', '\u{1}', s @ ..] => {
													super::push(out, 1197);
													s.is_empty()
												}
												['ν', 'ο', 'μ', '\u{1}', s @ ..] => {
													super::push(out, 1198);
													s.is_empty()
												}
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

/// Words ending -ερα (3 syllables).
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', 'ι', 'α', 'ι', 'δ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['υ', 'ε', 'λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['μ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						['φ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ω', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						['φ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 890);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 892);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['ι', 'μ', 'ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 894);
							s.is_empty()
						}
						['ξ', 'ο', 'δ', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σα (3 syllables).
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 404);
					match s {
						[] => true,
						['ι', 'ε', 'θ', s @ ..] => {
							super::push(out, 660);
							match s {
								[] => true,
								['φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['α', 'ι', 'ρ', 'η', 'τ', 'σ', 'ι', 'ε', 'λ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 661);
							s.is_empty()
						}
						['η', s @ ..] => {
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 897);
									match s {
										[] => true,
										['ε', 'λ', 'ο', 'π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1094);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 898);
									s.is_empty()
								}
								['ρ', 'ε', 'τ', 'σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'ν', s @ ..] => {
							super::push(out, 662);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 900);
									match s {
										[] => true,
										['φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 1095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 663);
							match s {
								[] => true,
								['ε', 'π', 'κ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'λ', s @ ..] => {
							super::push(out, 664);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 902);
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
						['σ', s @ ..] => {
							super::push(out, 665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['υ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 408);
					match s {
						[] => true,
						['θ', s @ ..] => {
							match s {
								[] => true,
								['η', 'ν', s @ ..] => {
									super::push(out, 903);
									match s {
										[] => true,
										['ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 1096);
											s.is_empty()
										}
										['ν', 'ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 1097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', 'ι', s @ ..] => {
									match s {
										[] => true,
										['ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 1098);
											s.is_empty()
										}
										['ο', 'ρ', 'θ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1099);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 904);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 1100);
											match s {
												[] => true,
												['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
													super::push(out, 1199);
													s.is_empty()
												}
												['ο', 'π', 'α', '\u{1}', s @ ..] => {
													super::push(out, 1200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 667);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 906);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1101);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1102);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 1201);
													s.is_empty()
												}
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
		['σ', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 409);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 668);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 669);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 907);
									match s {
										[] => true,
										['α', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1103);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 908);
									match s {
										[] => true,
										['ο', 'τ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1104);
											s.is_empty()
										}
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
		['υ', s @ ..] => super::t0::n210(s, out),
		['ω', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['ε', 'ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['η', 'ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 692);
							match s {
								[] => true,
								['ο', '\u{1}', s @ ..] => {
									super::push(out, 976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 693);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 694);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 977);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1147);
											s.is_empty()
										}
										['φ', 'α', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στα (3 syllables).
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['ι', 'ρ', 'ε', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word απογενομενη (3 syllables).
pub(crate) fn n1249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1220);
	s.is_empty()
}

/// The word προσγενομενη (3 syllables).
pub(crate) fn n1250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	s.is_empty()
}

/// Words ending -υομενη (3 syllables).
pub(crate) fn n1033(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['ο', 'ρ', 'κ', 'γ', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ερχομενη (3 syllables).
pub(crate) fn n1034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	s.is_empty()
}

/// Words ending -σμενη (3 syllables).
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1008);
			s.is_empty()
		}
		['ι', 'ε', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υμενη (3 syllables).
pub(crate) fn n731(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1222);
									s.is_empty()
								}
								['α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 1223);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['β', 'ο', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['θ', 'η', 'ο', 'β', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωμενη (3 syllables).
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	s.is_empty()
}

/// Words ending -ηνη (3 syllables).
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['λ', 'α', 'κ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['μ', 'α', 'γ', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ινη (3 syllables).
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['α', 'λ', 'ε', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'μ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['τ', 'ω', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ρ', 'υ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['μ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1016);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1171);
									match s {
										[] => true,
										['σ', '\u{1}', s @ ..] => {
											super::push(out, 1214);
											s.is_empty()
										}
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
					super::push(out, 725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1018);
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
				['ω', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1019);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1173);
									match s {
										[] => true,
										['ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
											super::push(out, 1215);
											s.is_empty()
										}
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
		['σ', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['σ', 'υ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οσ (3 syllables).
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 770);
							match s {
								[] => true,
								['λ', 'υ', 'σ', 'α', 'κ', 'ε', 'δ', 'ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 1034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 772);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1035);
									match s {
										[] => true,
										['δ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1177);
											s.is_empty()
										}
										['ν', 'α', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
											super::push(out, 1178);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1036);
									match s {
										[] => true,
										['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 1179);
											s.is_empty()
										}
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
					super::push(out, 512);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 773);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 775);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1038);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1039);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1040);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 1180);
											s.is_empty()
										}
										['ν', 'ο', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 1181);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1182);
											s.is_empty()
										}
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
					super::push(out, 515);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 776);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 777);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 778);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 780);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1043);
									s.is_empty()
								}
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
			super::push(out, 267);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 781);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 782);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1044);
									match s {
										[] => true,
										['ο', s @ ..] => {
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
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 783);
							match s {
								[] => true,
								['τ', 'ν', 'α', 'ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 1045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 784);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 785);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'σ', 'ο', 'χ', 'ι', 'λ', 'ο', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['η', 'μ', 'α', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['σ', 'α', 'ρ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 789);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 790);
							match s {
								[] => true,
								['τ', 'υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 1047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['υ', 'λ', 'η', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 792);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 793);
							s.is_empty()
						}
						['τ', 'ω', 'ρ', 'π', 'ο', 'κ', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n273(s, out),
		['μ', s @ ..] => super::t0::n274(s, out),
		['ν', s @ ..] => super::t0::n275(s, out),
		['ξ', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['κ', 'η', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t0::n278(s, out),
		['ρ', s @ ..] => super::t0::n279(s, out),
		['σ', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t0::n281(s, out),
		['υ', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['φ', s @ ..] => super::t0::n283(s, out),
		['χ', s @ ..] => super::t0::n284(s, out),
		_ => false,
	}
}

/// Words ending -υσα (3 syllables).
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 909);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1105);
									s.is_empty()
								}
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
			super::push(out, 412);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 910);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 912);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1107);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1108);
									match s {
										[] => true,
										['κ', 'ε', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1203);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1109);
									s.is_empty()
								}
								['τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 914);
							match s {
								[] => true,
								['υ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 1111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 915);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 916);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 918);
							match s {
								[] => true,
								['π', 'κ', s @ ..] => {
									super::push(out, 1113);
									match s {
										[] => true,
										['ε', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 1204);
											s.is_empty()
										}
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
					super::push(out, 678);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 919);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1114);
									match s {
										[] => true,
										['ι', 'ε', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 1205);
											s.is_empty()
										}
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
					super::push(out, 679);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 920);
							match s {
								[] => true,
								['λ', '\u{1}', s @ ..] => {
									super::push(out, 1115);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1116);
									match s {
										[] => true,
										['ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1206);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', 'ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['χ', 'ο', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n690(s, out),
				['κ', s @ ..] => super::t0::n691(s, out),
				['λ', s @ ..] => super::t0::n692(s, out),
				['μ', s @ ..] => super::t0::n693(s, out),
				['ν', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['α', 'θ', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['ε', '\u{1}', s @ ..] => super::t0::n972(s, out),
						['ι', s @ ..] => super::t0::n973(s, out),
						['υ', s @ ..] => super::t0::n974(s, out),
						_ => false,
					}
				}
				['ξ', s @ ..] => super::t0::n695(s, out),
				['ο', s @ ..] => super::t0::n696(s, out),
				['π', s @ ..] => super::t0::n697(s, out),
				['ρ', s @ ..] => super::t0::n698(s, out),
				['σ', s @ ..] => super::t0::n699(s, out),
				['τ', s @ ..] => super::t0::n700(s, out),
				['υ', s @ ..] => super::t0::n701(s, out),
				['χ', s @ ..] => super::t0::n702(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λοσ (3 syllables).
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['φ', 'ε', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['κ', 'ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['π', 'η', 'μ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1050);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1184);
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
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μοσ (3 syllables).
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['α', s @ ..] => {
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
				['α', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 804);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1053);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['ι', 'α', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1055);
							match s {
								[] => true,
								['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['τ', 'σ', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['τ', 'η', 'τ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νοσ (3 syllables).
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['π', 'η', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'υ', 'α', 'λ', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1058);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ποσ (3 syllables).
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', 'ο', 'π', 'ο', 'χ', 'υ', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 560);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1062);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', 'τ', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροσ (3 syllables).
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 565);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['σ', 'ι', 'ρ', 'α', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['γ', 'ο', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 826);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1071);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1189);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 1216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1072);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1190);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1073);
							match s {
								[] => true,
								['γ', 'ι', 'τ', 'σ', 'α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 1191);
									s.is_empty()
								}
								['φ', 'η', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 1192);
									s.is_empty()
								}
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
			super::push(out, 571);
			match s {
				[] => true,
				['υ', 'λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 835);
					match s {
						[] => true,
						['ι', 'α', 'λ', 'α', 'τ', 'ο', 'κ', 'ι', 'δ', 'ο', 'τ', 'ν', 'α', 'φ', 'ο', 'κ', 'υ', 'σ', 'ο', 'τ', 'ι', 'ο', 'φ', 'ο', 'ρ', 'θ', 'ρ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τοσ (3 syllables).
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 836);
					match s {
						[] => true,
						['η', 'χ', 'ο', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['κ', 'ι', 'ο', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 838);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1076);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1193);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1218);
											s.is_empty()
										}
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
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['ω', 'ρ', 'π', 'υ', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['β', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['β', 'ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1077);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'ρ', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['θ', 'ο', 'μ', 'μ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1080);
							match s {
								[] => true,
								['λ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'ρ', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 848);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1081);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['β', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['λ', 'α', 'μ', 'χ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φοσ (3 syllables).
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['ρ', 'γ', 'ο', 'ρ', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['λ', 'α', 'τ', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χοσ (3 syllables).
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['ο', 'τ', s @ ..] => {
							super::push(out, 1083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 588);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['ξ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 859);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 860);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 861);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1086);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1196);
									match s {
										[] => true,
										['ρ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 1219);
											s.is_empty()
										}
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
					super::push(out, 862);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιουσα (3 syllables).
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['μ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 929);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κουσα (3 syllables).
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 681);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 933);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 934);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 936);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λουσα (3 syllables).
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 682);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				['κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			match s {
				[] => true,
				['γ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 939);
			s.is_empty()
		}
		['κ', 'υ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μουσα (3 syllables).
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 683);
	match s {
		[] => true,
		['α', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ενουσα (3 syllables).
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	s.is_empty()
}

/// Words ending -ινουσα (3 syllables).
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνουσα (3 syllables).
pub(crate) fn n974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	s.is_empty()
}

/// Words ending -ξουσα (3 syllables).
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	s.is_empty()
}

/// Words ending -οουσα (3 syllables).
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 686);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πουσα (3 syllables).
pub(crate) fn n697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 952);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρουσα (3 syllables).
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	match s {
		[] => true,
		['α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σουσα (3 syllables).
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['ρ', 'α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 964);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τουσα (3 syllables).
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['π', 'υ', 'ν', 'ο', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', 'ν', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υουσα (3 syllables).
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χουσα (3 syllables).
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['μ', 'υ', 'α', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 974);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

