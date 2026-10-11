use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 23);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 56);
									match s {
										[] => true,
										['ῥ', '\u{1}', s @ ..] => {
											super::push(out, 256);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 57);
									match s {
										[] => true,
										['λ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 257);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ζ', '\u{1}', s @ ..] => {
									super::push(out, 58);
									s.is_empty()
								}
								['η', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 59);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 60);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 259);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 260);
											match s {
												[] => true,
												['λ', 'π', '\u{1}', s @ ..] => {
													super::push(out, 765);
													s.is_empty()
												}
												['μ', '\u{1}', s @ ..] => {
													super::push(out, 766);
													s.is_empty()
												}
												_ => false,
											}
										}
										['κ', '\u{1}', s @ ..] => {
											super::push(out, 261);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 262);
											s.is_empty()
										}
										['π', '\u{1}', s @ ..] => {
											super::push(out, 263);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'β', '\u{1}', s @ ..] => {
									super::push(out, 61);
									s.is_empty()
								}
								['ρ', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 62);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 63);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 264);
											s.is_empty()
										}
										['θ', '\u{1}', s @ ..] => {
											super::push(out, 265);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 266);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 767);
													s.is_empty()
												}
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
				['ξ', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['α', 'λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 25);
							match s {
								[] => true,
								['τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 64);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 65);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 27);
							match s {
								[] => true,
								['α', 'ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 66);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 67);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 267);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 268);
											match s {
												[] => true,
												['β', '\u{1}', s @ ..] => {
													super::push(out, 768);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 769);
													s.is_empty()
												}
												_ => false,
											}
										}
										['δ', '\u{1}', s @ ..] => {
											super::push(out, 269);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 270);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 271);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 770);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 272);
											s.is_empty()
										}
										['χ', '\u{1}', s @ ..] => {
											super::push(out, 273);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 68);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 69);
									s.is_empty()
								}
								['υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 70);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 71);
									match s {
										[] => true,
										['ζ', '\u{1}', s @ ..] => {
											super::push(out, 274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 28);
							match s {
								[] => true,
								['α', 'ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 72);
									s.is_empty()
								}
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 73);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 74);
									match s {
										[] => true,
										['δ', '\u{1}', s @ ..] => {
											super::push(out, 275);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', '\u{1}', s @ ..] => {
									super::push(out, 75);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 29);
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
		['α', 'η', 'ρ', 'ε', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['ε', 'δ', 'σ', 'ο', 'ι', 'ο', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 8);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 76);
							match s {
								[] => true,
								['ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => super::t0::n31(s, out),
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['υ', 'ν', 'ω', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 32);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 11);
			s.is_empty()
		}
		['σ', s @ ..] => super::t0::n12(s, out),
		['ψ', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['θ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 163);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending - (3 syllables).
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['τ', 'ν', 'α', 'σ', 'ι', 'ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 165);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 504);
									s.is_empty()
								}
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
			super::push(out, 16);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 166);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 505);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 971);
											match s {
												[] => true,
												['χ', 'ρ', 'α', 'π', 'υ', '\u{1}', s @ ..] => {
													super::push(out, 1521);
													s.is_empty()
												}
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
		['ι', 'α', 'ν', 'ε', 'μ', 'ο', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 17);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 43);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['ι', 'ρ', 'η', 'τ', 'υ', 'λ', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 167);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 168);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 506);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 972);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 1522);
													match s {
														[] => true,
														['χ', 'ρ', 'α', 'π', 'υ', '\u{1}', s @ ..] => super::t0::n1941(s, out),
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
				['ω', s @ ..] => super::t0::n45(s, out),
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 186);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 561);
									match s {
										[] => true,
										['ν', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 1024);
											s.is_empty()
										}
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
					super::push(out, 47);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 187);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 562);
									s.is_empty()
								}
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
			super::push(out, 20);
			match s {
				[] => true,
				['ω', s @ ..] => super::t0::n48(s, out),
				_ => false,
			}
		}
		['σ', s @ ..] => super::t0::n21(s, out),
		['ω', 'ν', 'ε', 'μ', 'ο', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 22);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ων (2 syllables).
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['λ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 278);
					s.is_empty()
				}
				['η', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 279);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 281);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 282);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 283);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 284);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 772);
							match s {
								[] => true,
								['κ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 1467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 286);
					match s {
						[] => true,
						['ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 288);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 291);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 777);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
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
					match s {
						[] => true,
						['λ', 'ε', 'β', '\u{1}', s @ ..] => {
							super::push(out, 780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 781);
							match s {
								[] => true,
								['ρ', 'β', '\u{1}', s @ ..] => {
									super::push(out, 1469);
									s.is_empty()
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 302);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				['σ', 'α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				['ι', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['ρ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 785);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 786);
							s.is_empty()
						}
						['ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['μ', 'ο', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 314);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 316);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 789);
							s.is_empty()
						}
						['γ', '\u{1}', s @ ..] => {
							super::push(out, 790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['θ', '\u{1}', s @ ..] => {
							super::push(out, 791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 320);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 792);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1470);
									s.is_empty()
								}
								['χ', '\u{1}', s @ ..] => {
									super::push(out, 1471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 794);
							s.is_empty()
						}
						['ι', 'α', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 795);
							s.is_empty()
						}
						['μ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 796);
							s.is_empty()
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 797);
							s.is_empty()
						}
						['ω', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 799);
							match s {
								[] => true,
								['θ', '\u{1}', s @ ..] => {
									super::push(out, 1472);
									s.is_empty()
								}
								['μ', '\u{1}', s @ ..] => {
									super::push(out, 1473);
									s.is_empty()
								}
								['ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', '\u{1}', s @ ..] => {
							super::push(out, 800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 327);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 328);
					s.is_empty()
				}
				['π', s @ ..] => super::t0::n329(s, out),
				['τ', s @ ..] => super::t0::n330(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n95(s, out),
		['φ', s @ ..] => super::t0::n96(s, out),
		['χ', s @ ..] => super::t0::n97(s, out),
		_ => false,
	}
}

/// Words ending -σ (2 syllables).
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 98);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 99);
					match s {
						[] => true,
						['ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 339);
							s.is_empty()
						}
						['ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 100);
					match s {
						[] => true,
						['α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 341);
							s.is_empty()
						}
						['ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 101);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 102);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 344);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 807);
									s.is_empty()
								}
								['ρ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 808);
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
						['α', s @ ..] => {
							super::push(out, 345);
							match s {
								[] => true,
								['ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 346);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 347);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 810);
									s.is_empty()
								}
								['ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ε', 'ζ', '\u{1}', s @ ..] => {
							super::push(out, 348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 104);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 105);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 350);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 351);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 106);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 352);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 813);
									s.is_empty()
								}
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 814);
									s.is_empty()
								}
								['ρ', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 815);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 353);
							match s {
								[] => true,
								['ψ', '\u{1}', s @ ..] => {
									super::push(out, 816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 354);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 817);
									s.is_empty()
								}
								['ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 355);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 356);
							match s {
								[] => true,
								['λ', '\u{1}', s @ ..] => {
									super::push(out, 819);
									s.is_empty()
								}
								['φ', '\u{1}', s @ ..] => {
									super::push(out, 820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 107);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 109);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 110);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 359);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 821);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 822);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1475);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 360);
							match s {
								[] => true,
								['ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 823);
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
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 361);
							match s {
								[] => true,
								['ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 363);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 115);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 366);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 825);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 826);
									s.is_empty()
								}
								['υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 367);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 368);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 828);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 369);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 370);
							match s {
								[] => true,
								['ν', 'ο', 'τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 830);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'λ', 'ε', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				['ρ', 'κ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n36(s, out),
		['υ', s @ ..] => super::t0::n37(s, out),
		['ω', s @ ..] => super::t0::n38(s, out),
		_ => false,
	}
}

/// The word υπαρχομενον (3 syllables).
pub(crate) fn n1941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1935);
	s.is_empty()
}

/// Words ending -ων (3 syllables).
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 974);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1523);
									match s {
										[] => true,
										['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 1936);
											s.is_empty()
										}
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
					super::push(out, 509);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 975);
							match s {
								[] => true,
								['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ι', 'μ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', 'π', 'ο', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['α', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 979);
							match s {
								[] => true,
								['α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 980);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 981);
							match s {
								[] => true,
								['σ', 'υ', 'θ', 'ε', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 982);
							match s {
								[] => true,
								['ι', 'ε', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 1527);
									s.is_empty()
								}
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
			super::push(out, 174);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['ι', 'σ', 'ο', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						['ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'α', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 987);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1528);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1937);
											s.is_empty()
										}
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
			super::push(out, 176);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['ε', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 989);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 990);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1529);
									s.is_empty()
								}
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
			super::push(out, 177);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['α', 'π', 'ε', 'χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 993);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 994);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1531);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1938);
											s.is_empty()
										}
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
					super::push(out, 532);
					match s {
						[] => true,
						['λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 996);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 1532);
									s.is_empty()
								}
								['ι', 'σ', 'η', 'ν', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 1533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 997);
							s.is_empty()
						}
						['χ', 'σ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 999);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1534);
									match s {
										[] => true,
										['ο', s @ ..] => {
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2183);
													match s {
														[] => true,
														['α', 'κ', 'σ', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => super::t0::n2355(s, out),
														_ => false,
													}
												}
												['σ', 'ι', '\u{1}', s @ ..] => {
													super::push(out, 2184);
													s.is_empty()
												}
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
				['ρ', 'ο', 'φ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['β', s @ ..] => super::t0::n1000(s, out),
						['θ', s @ ..] => super::t0::n1001(s, out),
						_ => false,
					}
				}
				['ε', 'μ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => super::t0::n537(s, out),
				['ι', s @ ..] => super::t0::n538(s, out),
				['υ', s @ ..] => super::t0::n539(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t0::n180(s, out),
		['ρ', s @ ..] => super::t0::n181(s, out),
		['σ', s @ ..] => super::t0::n182(s, out),
		['τ', s @ ..] => super::t0::n183(s, out),
		['υ', s @ ..] => super::t0::n184(s, out),
		['χ', s @ ..] => super::t0::n185(s, out),
		_ => false,
	}
}

/// Words ending -ωρ (3 syllables).
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['ψ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['α', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', 'ε', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σ (3 syllables).
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 49);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 190);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 567);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 191);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 192);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 568);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1028);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1544);
											match s {
												[] => true,
												['κ', 'υ', 'ε', 'λ', '\u{1}', s @ ..] => {
													super::push(out, 1941);
													s.is_empty()
												}
												['ρ', 'ω', 'λ', 'χ', '\u{1}', s @ ..] => {
													super::push(out, 1942);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', 'α', 'δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 193);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 194);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 570);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1029);
									s.is_empty()
								}
								['ρ', 'κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'τ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 572);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 573);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'α', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 574);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 196);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 576);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1032);
									match s {
										[] => true,
										['δ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
											super::push(out, 1545);
											s.is_empty()
										}
										['θ', 'φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 1546);
											s.is_empty()
										}
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
					super::push(out, 197);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 577);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1033);
									match s {
										[] => true,
										['ρ', 'η', 'τ', 'σ', 'ι', 'ε', 'λ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 1547);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 578);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1034);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 579);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1035);
									match s {
										[] => true,
										['ε', 'λ', 'ο', 'π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1549);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 1036);
									s.is_empty()
								}
								['ρ', 'ε', 'τ', 'σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 580);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1038);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 1039);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1550);
											match s {
												[] => true,
												['φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
													super::push(out, 1943);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'μ', 'ο', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 581);
							s.is_empty()
						}
						['ρ', 'ε', 'π', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 582);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 583);
							match s {
								[] => true,
								['δ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1041);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 1042);
									s.is_empty()
								}
								['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1043);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1044);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 1045);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											super::push(out, 1551);
											s.is_empty()
										}
										['κ', 'α', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 1552);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 198);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 585);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 199);
					match s {
						[] => true,
						['υ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n50(s, out),
		['ι', s @ ..] => super::t0::n51(s, out),
		['ο', s @ ..] => super::t0::n52(s, out),
		['υ', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['θ', 'χ', 'ι', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				['λ', 'η', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 245);
					s.is_empty()
				}
				['ο', s @ ..] => super::t0::n246(s, out),
				['σ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				['χ', s @ ..] => super::t0::n248(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t0::n54(s, out),
		_ => false,
	}
}

/// Words ending -πτων (2 syllables).
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 802);
			s.is_empty()
		}
		['ι', 'π', '\u{1}', s @ ..] => {
			super::push(out, 803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ττων (2 syllables).
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 804);
			s.is_empty()
		}
		['η', '\u{1}', s @ ..] => {
			super::push(out, 805);
			s.is_empty()
		}
		['ι', 'ε', 'ρ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υων (2 syllables).
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['ρ', 'κ', 'α', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['χ', 'σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φων (2 syllables).
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['υ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χων (2 syllables).
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['ι', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		['σ', s @ ..] => {
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

/// Words ending -οσ (2 syllables).
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 372);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 832);
							s.is_empty()
						}
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 377);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['β', '\u{1}', s @ ..] => {
							super::push(out, 836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 380);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 381);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 837);
							s.is_empty()
						}
						['ω', 'ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 382);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 383);
					match s {
						[] => true,
						['υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 842);
							s.is_empty()
						}
						['υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['ι', 'θ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['η', 'ο', 'β', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'υ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 846);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1476);
									match s {
										[] => true,
										['θ', '\u{1}', s @ ..] => {
											super::push(out, 1929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 847);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1477);
									match s {
										[] => true,
										['χ', '\u{1}', s @ ..] => {
											super::push(out, 1930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 849);
							match s {
								[] => true,
								['α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1478);
									s.is_empty()
								}
								['ο', 'χ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 852);
							match s {
								[] => true,
								['ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 1480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 853);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1481);
									s.is_empty()
								}
								['ν', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 1482);
									s.is_empty()
								}
								['υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 1483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 854);
							match s {
								[] => true,
								['μ', 'υ', 'ν', '\u{1}', s @ ..] => {
									super::push(out, 1484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 857);
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
							super::push(out, 858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => super::t0::n398(s, out),
				['ε', s @ ..] => super::t0::n399(s, out),
				['θ', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['ν', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t0::n401(s, out),
				['λ', s @ ..] => super::t0::n402(s, out),
				['μ', s @ ..] => super::t0::n403(s, out),
				['ν', s @ ..] => super::t0::n404(s, out),
				['ξ', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['α', s @ ..] => super::t0::n884(s, out),
						['ε', 'δ', '\u{1}', s @ ..] => super::t0::n885(s, out),
						_ => false,
					}
				}
				['ο', s @ ..] => super::t0::n406(s, out),
				['π', s @ ..] => super::t0::n407(s, out),
				['ρ', s @ ..] => super::t0::n408(s, out),
				['σ', s @ ..] => super::t0::n409(s, out),
				['τ', s @ ..] => super::t0::n410(s, out),
				['φ', s @ ..] => super::t0::n411(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n129(s, out),
		['λ', s @ ..] => super::t0::n130(s, out),
		['μ', s @ ..] => super::t0::n131(s, out),
		['ν', s @ ..] => super::t0::n132(s, out),
		['ξ', s @ ..] => super::t0::n133(s, out),
		['ο', s @ ..] => super::t0::n134(s, out),
		['π', s @ ..] => super::t0::n135(s, out),
		['ρ', s @ ..] => super::t0::n136(s, out),
		['σ', s @ ..] => super::t0::n137(s, out),
		['τ', s @ ..] => super::t0::n138(s, out),
		['υ', s @ ..] => super::t0::n139(s, out),
		['φ', s @ ..] => super::t0::n140(s, out),
		['χ', s @ ..] => super::t0::n141(s, out),
		['ψ', s @ ..] => super::t0::n142(s, out),
		['ω', s @ ..] => super::t0::n143(s, out),
		['ϝ', 'ν', 'ε', 'ξ', '\u{1}', s @ ..] => super::t0::n144(s, out),
		_ => false,
	}
}

/// Words ending -υσ (2 syllables).
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['β', 'σ', 'ε', 'ρ', 'π', '\u{1}', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['γ', 'ι', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['η', '\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['ω', '\u{1}', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['η', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				['ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 492);
					s.is_empty()
				}
				['ω', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'κ', 'ι', 'ε', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['δ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 966);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 1518);
									s.is_empty()
								}
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 1519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 967);
							match s {
								[] => true,
								['ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 1520);
									s.is_empty()
								}
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
			super::push(out, 153);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 498);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['τ', '\u{1}', s @ ..] => {
							super::push(out, 969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσ (2 syllables).
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 499);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				['τ', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', 'ι', 'ε', 'α', '\u{1}', s @ ..] => {
			super::push(out, 158);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['τ', 'σ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word τρισκακοδαιμων (3 syllables).
pub(crate) fn n2355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2332);
	s.is_empty()
}

/// Words ending -βανων (3 syllables).
pub(crate) fn n1000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1000);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θανων (3 syllables).
pub(crate) fn n1001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1001);
	match s {
		[] => true,
		['ν', 'α', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 1536);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word παραμενων (3 syllables).
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	s.is_empty()
}

/// Words ending -ινων (3 syllables).
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['ε', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνων (3 syllables).
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1537);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πων (3 syllables).
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρων (3 syllables).
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1538);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1940);
									s.is_empty()
								}
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
			super::push(out, 543);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['φ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['θ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						['λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σων (3 syllables).
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'α', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['λ', 'ω', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -των (3 syllables).
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['ε', 'π', 'υ', 'ν', 'ο', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['ε', 'γ', 'υ', 'τ', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', 'ν', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υων (3 syllables).
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 554);
			s.is_empty()
		}
		['λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χων (3 syllables).
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 556);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['ψ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ησ (3 syllables).
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['α', 'σ', 'υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1049);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1944);
											match s {
												[] => true,
												['φ', 'ο', '\u{1}', s @ ..] => {
													super::push(out, 2185);
													s.is_empty()
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 1945);
											match s {
												[] => true,
												['γ', 'ι', 'σ', '\u{1}', s @ ..] => {
													super::push(out, 2186);
													s.is_empty()
												}
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
					super::push(out, 591);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1051);
							match s {
								[] => true,
								['ε', 'χ', 'ι', 'ο', 'τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 1553);
									s.is_empty()
								}
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
			super::push(out, 203);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1052);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1554);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 1946);
											s.is_empty()
										}
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
					super::push(out, 595);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1053);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['γ', 'η', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1057);
							match s {
								[] => true,
								['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 1556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['π', 'υ', 'ν', 'ο', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'ο', 'τ', 'ν', 'ο', 'κ', 'α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ισ (3 syllables).
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['ν', 'ε', 'μ', 'ο', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['π', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['η', 'ν', s @ ..] => {
							super::push(out, 1059);
							match s {
								[] => true,
								['ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 1557);
									s.is_empty()
								}
								['ν', 'ε', 'γ', '\u{1}', s @ ..] => {
									super::push(out, 1558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 1559);
									s.is_empty()
								}
								['ο', 'ρ', 'θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1561);
									match s {
										[] => true,
										['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 1947);
											s.is_empty()
										}
										['ο', 'π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1948);
											s.is_empty()
										}
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
					super::push(out, 607);
					match s {
						[] => true,
						['κ', 'α', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1061);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1062);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1065);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1562);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 1949);
											s.is_empty()
										}
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
		['κ', 'λ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['ν', 'ο', 'υ', 'κ', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['ο', 'ν', 'ε', 'μ', 'ο', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['π', 'σ', 'α', 'ρ', 'ε', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['α', 'χ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['τ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οσ (3 syllables).
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['α', 'ρ', 'ε', 'κ', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1066);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1563);
									match s {
										[] => true,
										['υ', 'σ', s @ ..] => {
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2187);
													match s {
														[] => true,
														['κ', 'ε', 'δ', 'ν', 'ε', '\u{1}', s @ ..] => super::t0::n2356(s, out),
														['τ', 'ν', 'ε', 'π', '\u{1}', s @ ..] => super::t0::n2357(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 2188);
													s.is_empty()
												}
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
					super::push(out, 612);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1067);
							match s {
								[] => true,
								['φ', 'υ', 'ε', '\u{1}', s @ ..] => {
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
		['γ', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1069);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1565);
									match s {
										[] => true,
										['δ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 1950);
											s.is_empty()
										}
										['ν', 'α', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
											super::push(out, 1951);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1566);
									match s {
										[] => true,
										['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 1952);
											s.is_empty()
										}
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
					super::push(out, 615);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1070);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1567);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 1953);
											s.is_empty()
										}
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
					super::push(out, 616);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1071);
							match s {
								[] => true,
								['σ', s @ ..] => {
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
					super::push(out, 617);
					match s {
						[] => true,
						['λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1073);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1569);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1570);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1571);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 1954);
											s.is_empty()
										}
										['ν', 'ο', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 1955);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1956);
											s.is_empty()
										}
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
					super::push(out, 619);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1074);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1075);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1076);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1077);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1079);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1574);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1957);
											s.is_empty()
										}
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
			super::push(out, 222);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1081);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1575);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1958);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1083);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								['σ', 'ρ', 'α', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1085);
							match s {
								[] => true,
								['μ', 'ε', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', 'ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						['υ', s @ ..] => super::t0::n1088(s, out),
						_ => false,
					}
				}
				['χ', 'ι', 'ε', 'τ', 'υ', 'ε', '\u{1}', s @ ..] => super::t0::n629(s, out),
				_ => false,
			}
		}
		['ζ', s @ ..] => super::t0::n224(s, out),
		['θ', s @ ..] => super::t0::n225(s, out),
		['ι', s @ ..] => super::t0::n226(s, out),
		['κ', s @ ..] => super::t0::n227(s, out),
		['λ', s @ ..] => super::t0::n228(s, out),
		['μ', s @ ..] => super::t0::n229(s, out),
		['ν', s @ ..] => super::t0::n230(s, out),
		['ξ', s @ ..] => super::t0::n231(s, out),
		['ο', s @ ..] => super::t0::n232(s, out),
		['π', s @ ..] => super::t0::n233(s, out),
		['ρ', s @ ..] => super::t0::n234(s, out),
		['σ', s @ ..] => super::t0::n235(s, out),
		['τ', s @ ..] => super::t0::n236(s, out),
		['υ', s @ ..] => super::t0::n237(s, out),
		['φ', s @ ..] => super::t0::n238(s, out),
		['χ', s @ ..] => super::t0::n239(s, out),
		['ψ', 'ι', 'δ', 'α', '\u{1}', s @ ..] => super::t0::n240(s, out),
		['ω', 'ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => super::t0::n241(s, out),
		_ => false,
	}
}

/// Words ending -ουσ (3 syllables).
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['ν', 'ε', 'μ', 'ο', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				['υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['ρ', 'α', 'μ', 'ι', 'ε', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυσ (3 syllables).
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['ρ', 'β', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσ (3 syllables).
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['η', 'τ', 'ο', 'κ', 'ε', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 251);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['κ', 'ι', 'ν', 'ε', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['ν', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['η', 'χ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['ο', 'γ', 'ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1465);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1927);
									match s {
										[] => true,
										['γ', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2181);
											s.is_empty()
										}
										['ν', 'ο', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 2182);
											s.is_empty()
										}
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
		['τ', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1466);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1928);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2330);
													s.is_empty()
												}
												['ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
													super::push(out, 2331);
													s.is_empty()
												}
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

/// Words ending -διοσ (2 syllables).
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['ῥ', '\u{1}', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'χ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ειοσ (2 syllables).
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 863);
			s.is_empty()
		}
		['θ', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['η', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 868);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κιοσ (2 syllables).
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['ρ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 871);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιοσ (2 syllables).
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 874);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['ε', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μιοσ (2 syllables).
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		['η', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 880);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νιοσ (2 syllables).
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 882);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αξιοσ (2 syllables).
pub(crate) fn n884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 884);
	s.is_empty()
}

/// The word δεξιοσ (2 syllables).
pub(crate) fn n885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 885);
	s.is_empty()
}

/// Words ending -οιοσ (2 syllables).
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['η', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πιοσ (2 syllables).
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριοσ (2 syllables).
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 889);
			s.is_empty()
		}
		['θ', 'ι', 'α', '\u{1}', s @ ..] => {
			super::push(out, 890);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιοσ (2 syllables).
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['θ', '\u{1}', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιοσ (2 syllables).
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φιοσ (2 syllables).
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	s.is_empty()
}

/// Words ending -κοσ (2 syllables).
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['ε', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', 'η', 'ρ', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λοσ (2 syllables).
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', 'υ', 'ο', 'ζ', '\u{1}', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['τ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['χ', 'α', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μοσ (2 syllables).
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['ε', '\u{1}', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['τ', '\u{1}', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', 'ο', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νοσ (2 syllables).
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				['η', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				['π', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1498);
							match s {
								[] => true,
								['θ', '\u{1}', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
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
			super::push(out, 436);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['υ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξοσ (2 syllables).
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οοσ (2 syllables).
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['δ', 'γ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						['ο', 'ε', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 1500);
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
				['θ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				['κ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ποσ (2 syllables).
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['ι', 'ο', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροσ (2 syllables).
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 449);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['τ', '\u{1}', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['γ', '\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						['κ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								['φ', '\u{1}', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', '\u{1}', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						['ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1510);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
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
			super::push(out, 460);
			match s {
				[] => true,
				['ζ', '\u{1}', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σοσ (2 syllables).
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['ε', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τοσ (2 syllables).
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['β', '\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['δ', '\u{1}', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				['θ', '\u{1}', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['λ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['δ', '\u{1}', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				['π', '\u{1}', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						['τ', '\u{1}', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['χ', '\u{1}', s @ ..] => {
					super::push(out, 964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υοσ (2 syllables).
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['ζ', 'ι', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φοσ (2 syllables).
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χοσ (2 syllables).
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['ο', 'η', 'ι', 'α', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψοσ (2 syllables).
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωοσ (2 syllables).
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['θ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ξενϝοσ (2 syllables).
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	s.is_empty()
}

/// The word ενδεκασυλλαβοσ (3 syllables).
pub(crate) fn n2356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2333);
	s.is_empty()
}

/// The word πεντασυλλαβοσ (3 syllables).
pub(crate) fn n2357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2334);
	s.is_empty()
}

/// Words ending -υτεοσ (3 syllables).
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						['τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ευτειχεοσ (3 syllables).
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	s.is_empty()
}

/// Words ending -ζοσ (3 syllables).
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['ι', 'ρ', 'ο', 'ρ', 'υ', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 630);
			s.is_empty()
		}
		['υ', 'ρ', 'β', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θοσ (3 syllables).
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['α', 'γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['ο', 'ν', 'ο', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1582);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1960);
									s.is_empty()
								}
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

/// Words ending -ιοσ (3 syllables).
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['β', 'ρ', 'ο', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1584);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1961);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2191);
											s.is_empty()
										}
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
					super::push(out, 1095);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1585);
							match s {
								[] => true,
								['θ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['ν', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2192);
											s.is_empty()
										}
										['ν', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2193);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['ι', 'κ', 'ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1587);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1963);
									s.is_empty()
								}
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
			super::push(out, 638);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['γ', 'α', 'ρ', 'α', 'μ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1591);
							match s {
								[] => true,
								['ε', 'ν', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 1964);
									s.is_empty()
								}
								['π', 'ι', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 1965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', 'ι', 'σ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'δ', 'α', 'π', 'μ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['ε', 'τ', 'ο', 'ρ', 'ι', 'ε', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'ω', 'ρ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['ν', 'ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', 'α', 'β', 'α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				['υ', 'ν', 'ι', 'α', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['σ', 'ο', 'χ', 'ι', 'λ', 'ο', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1117);
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
					super::push(out, 1118);
					match s {
						[] => true,
						['τ', 'υ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						['υ', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['χ', 'σ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'μ', 'ε', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				['η', 'μ', 'α', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						['ε', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ρ', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1602);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', 'φ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['α', 'τ', 'ο', 'π', 'ο', 'σ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				['ε', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['σ', 'α', 'ρ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['ρ', 'υ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1967);
									s.is_empty()
								}
								['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 1968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', 'μ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['θ', 'ι', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						['μ', 'υ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'υ', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1607);
							match s {
								[] => true,
								['λ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1969);
									s.is_empty()
								}
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
			super::push(out, 645);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['δ', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1610);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1970);
									s.is_empty()
								}
								['σ', 'σ', 'ω', 'λ', 'γ', '\u{1}', s @ ..] => super::t0::n1981(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t0::n647(s, out),
		['ρ', s @ ..] => super::t0::n648(s, out),
		['σ', s @ ..] => super::t0::n649(s, out),
		['τ', s @ ..] => super::t0::n650(s, out),
		['φ', 'α', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => super::t0::n651(s, out),
		['χ', s @ ..] => super::t0::n652(s, out),
		_ => false,
	}
}

/// Words ending -κοσ (3 syllables).
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['δ', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['ζ', 'υ', 'ρ', 'β', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1625);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1626);
							match s {
								[] => true,
								['ι', 'ο', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 1982);
									s.is_empty()
								}
								['υ', 'φ', 'μ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1627);
							match s {
								[] => true,
								['α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 1984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'υ', s @ ..] => {
							super::push(out, 1628);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 1985);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 1986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2204);
											s.is_empty()
										}
										['σ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['δ', 'υ', 'ο', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						['μ', s @ ..] => {
							match s {
								[] => true,
								['α', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 1987);
									s.is_empty()
								}
								['ε', 'λ', 'ο', 'τ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 1988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', 'η', 'θ', 'α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1630);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['α', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['σ', 'α', 'λ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						['ε', 'π', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						['ν', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						['ο', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['ε', 'τ', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1989);
									s.is_empty()
								}
								['σ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 1990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['ω', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['η', 'δ', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['α', 'χ', 'η', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1647);
							match s {
								[] => true,
								['μ', s @ ..] => super::t0::n2002(s, out),
								['ρ', s @ ..] => super::t0::n2003(s, out),
								_ => false,
							}
						}
						['ρ', 'ο', 'π', '\u{1}', s @ ..] => super::t0::n1653(s, out),
						['ω', s @ ..] => super::t0::n1654(s, out),
						_ => false,
					}
				}
				['ο', s @ ..] => super::t0::n1180(s, out),
				['π', s @ ..] => super::t0::n1181(s, out),
				['ρ', s @ ..] => super::t0::n1182(s, out),
				['σ', s @ ..] => super::t0::n1183(s, out),
				['τ', s @ ..] => super::t0::n1184(s, out),
				['υ', s @ ..] => super::t0::n1185(s, out),
				['φ', s @ ..] => super::t0::n1186(s, out),
				['χ', s @ ..] => super::t0::n1187(s, out),
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n655(s, out),
		['ο', s @ ..] => super::t0::n656(s, out),
		['ρ', s @ ..] => super::t0::n657(s, out),
		['υ', s @ ..] => super::t0::n658(s, out),
		_ => false,
	}
}

/// Words ending -λοσ (3 syllables).
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['ζ', 'α', 'μ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1677);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2227);
											s.is_empty()
										}
										['α', '\u{1}', s @ ..] => {
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
				['φ', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1678);
							match s {
								[] => true,
								['κ', 'ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', 'α', 'π', 'κ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1679);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2017);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2229);
											s.is_empty()
										}
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
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['δ', 'ε', 'π', 'υ', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['ο', 'δ', 'ι', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						['μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['π', 'η', 'μ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1687);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1688);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1691);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2020);
									match s {
										[] => true,
										['δ', 'υ', s @ ..] => {
											match s {
												[] => true,
												['λ', 'ο', 'π', '\u{1}', s @ ..] => {
													super::push(out, 2346);
													s.is_empty()
												}
												['χ', s @ ..] => {
													super::push(out, 2347);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['π', 'ο', 'λ', 'ο', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['τ', 'ρ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μοσ (3 syllables).
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				['ν', 'υ', 'δ', 'ο', 'σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['ρ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', 'υ', 'ρ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1229);
					match s {
						[] => true,
						['ω', 'δ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				['μ', 'ο', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['ο', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 1698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1699);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2022);
									match s {
										[] => true,
										['ε', 'τ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2230);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2023);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2231);
											s.is_empty()
										}
										['ν', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2232);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1703);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2025);
									match s {
										[] => true,
										['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 2233);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', 'ρ', 'τ', 'υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['ν', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', 'α', 'ρ', 'γ', 'ο', 'λ', 'η', 'λ', 'λ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['β', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1709);
							match s {
								[] => true,
								['τ', 'υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1710);
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 2027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', 'ο', 'κ', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 679);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1715);
							match s {
								[] => true,
								['ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2028);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2029);
									match s {
										[] => true,
										['ε', 'τ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2234);
											s.is_empty()
										}
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
					super::push(out, 1250);
					match s {
						[] => true,
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						['η', 'τ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['μ', 'α', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				['ρ', 'χ', 'ο', 'ν', 'ο', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νοσ (3 syllables).
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['β', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1719);
							match s {
								[] => true,
								['κ', 'υ', 'ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ρ', 'τ', 'κ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['ρ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 1721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['σ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1723);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2031);
									s.is_empty()
								}
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
			super::push(out, 683);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['θ', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				['μ', s @ ..] => super::t0::n1263(s, out),
				['ξ', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1735);
							s.is_empty()
						}
						['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['β', 'α', 'λ', 'κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1264);
					s.is_empty()
				}
				['θ', 'υ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1266);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1738);
							match s {
								[] => true,
								['γ', '\u{1}', s @ ..] => {
									super::push(out, 2076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1267);
					match s {
						[] => true,
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['ε', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['μ', 'ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n1270(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n686(s, out),
		['μ', s @ ..] => super::t0::n687(s, out),
		['ν', 'α', 'ρ', 'υ', 'τ', '\u{1}', s @ ..] => super::t0::n688(s, out),
		['ο', s @ ..] => super::t0::n689(s, out),
		['π', s @ ..] => super::t0::n690(s, out),
		['ρ', 'ε', 'τ', 'σ', 'υ', 'σ', 'α', 'δ', '\u{1}', s @ ..] => super::t0::n691(s, out),
		['τ', 'ι', 'λ', 'ε', 'π', '\u{1}', s @ ..] => super::t0::n692(s, out),
		['υ', s @ ..] => super::t0::n693(s, out),
		['χ', s @ ..] => super::t0::n694(s, out),
		['ω', s @ ..] => super::t0::n695(s, out),
		_ => false,
	}
}

/// Words ending -ξοσ (3 syllables).
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1790);
							match s {
								[] => true,
								['κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
								['ν', 'ε', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οοσ (3 syllables).
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['κ', 'η', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ποσ (3 syllables).
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['α', 'δ', 'ο', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 700);
			s.is_empty()
		}
		['ε', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1307);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', 'ο', 'π', 'ο', 'χ', 'υ', 'ψ', '\u{1}', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1794);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', 'λ', 'ε', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1310);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1311);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', 'ι', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
			super::push(out, 706);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροσ (3 syllables).
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1799);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2109);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2303);
											match s {
												[] => true,
												['ο', 'ξ', 'ι', 'μ', '\u{1}', s @ ..] => {
													super::push(out, 2388);
													s.is_empty()
												}
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
					super::push(out, 1318);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1800);
							match s {
								[] => true,
								['μ', '\u{1}', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', 'α', 'δ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 711);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['υ', 'ε', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1807);
							match s {
								[] => true,
								['φ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1809);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2112);
									s.is_empty()
								}
								['φ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'ε', 'χ', 'σ', 'υ', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						['ρ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 1811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
								['μ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								['φ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 2116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['α', 'ι', 'δ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 2117);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2118);
									match s {
										[] => true,
										['ν', '\u{1}', s @ ..] => {
											super::push(out, 2304);
											s.is_empty()
										}
										['φ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 2305);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1814);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2119);
									s.is_empty()
								}
								['π', '\u{1}', s @ ..] => {
									super::push(out, 2120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2121);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2306);
											match s {
												[] => true,
												['ε', 'μ', 'υ', '\u{1}', s @ ..] => {
													super::push(out, 2389);
													s.is_empty()
												}
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
							super::push(out, 1816);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1817);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2122);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2123);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2307);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2390);
													match s {
														[] => true,
														['φ', 'μ', 'α', '\u{1}', s @ ..] => super::t0::n2428(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', '\u{1}', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 1819);
							match s {
								[] => true,
								['ι', 'μ', 'ι', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								['ξ', 'ο', 'δ', 'ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['ο', 'ν', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 714);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				['λ', 'κ', 'ο', 'λ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['ι', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1822);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1823);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n715(s, out),
		['ι', s @ ..] => super::t0::n716(s, out),
		['κ', s @ ..] => super::t0::n717(s, out),
		['ο', s @ ..] => super::t0::n718(s, out),
		['π', 'ο', 'κ', 'ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => super::t0::n719(s, out),
		['ρ', s @ ..] => super::t0::n720(s, out),
		['τ', s @ ..] => super::t0::n721(s, out),
		['υ', s @ ..] => super::t0::n722(s, out),
		['φ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => super::t0::n723(s, out),
		['χ', 'σ', 'υ', 'μ', 'α', '\u{1}', s @ ..] => super::t0::n724(s, out),
		['ω', s @ ..] => super::t0::n725(s, out),
		_ => false,
	}
}

/// Words ending -σοσ (3 syllables).
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['ε', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 726);
			s.is_empty()
		}
		['ι', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 727);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
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
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1372);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2137);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2138);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2311);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'μ', 'υ', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1374);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2140);
									match s {
										[] => true,
										['ι', 'α', 'κ', 'κ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2312);
											s.is_empty()
										}
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
					super::push(out, 1375);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1849);
							match s {
								[] => true,
								['κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								['χ', 'ο', 'ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 2142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1376);
					match s {
						[] => true,
						['α', 'κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2143);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						['ρ', 'α', 'σ', 'ι', 'ρ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 1853);
							s.is_empty()
						}
						['υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1377);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1855);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2144);
									match s {
										[] => true,
										['υ', 'χ', 'α', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 2314);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1857);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2145);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 2315);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1379);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						['ο', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						['π', 'ο', 'ι', 'μ', 'η', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1863);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2146);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2316);
											s.is_empty()
										}
										['τ', 'α', 'κ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2317);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								['υ', '\u{1}', s @ ..] => {
									super::push(out, 2148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['η', 'ν', 'υ', 'δ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1870);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2149);
									s.is_empty()
								}
								_ => false,
							}
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
		['η', s @ ..] => {
			super::push(out, 732);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1872);
							match s {
								[] => true,
								['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['ν', 'ε', 'γ', 'ο', 'ρ', 'υ', 'φ', 'ρ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', 'υ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1393);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1394);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1396);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1875);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2151);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2318);
											s.is_empty()
										}
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
					super::push(out, 1397);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1876);
							match s {
								[] => true,
								['κ', s @ ..] => super::t0::n2169(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => super::t0::n1399(s, out),
				['μ', s @ ..] => super::t0::n1400(s, out),
				['σ', s @ ..] => super::t0::n1401(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n734(s, out),
		['ν', s @ ..] => super::t0::n735(s, out),
		['ο', s @ ..] => super::t0::n736(s, out),
		['π', s @ ..] => super::t0::n737(s, out),
		['ρ', s @ ..] => super::t0::n738(s, out),
		['σ', s @ ..] => super::t0::n739(s, out),
		['τ', s @ ..] => super::t0::n740(s, out),
		['υ', s @ ..] => super::t0::n741(s, out),
		['ω', s @ ..] => super::t0::n742(s, out),
		_ => false,
	}
}

/// Words ending -υοσ (3 syllables).
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	s.is_empty()
}

/// Words ending -φοσ (3 syllables).
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 743);
			match s {
				[] => true,
				['δ', 'ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1917);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2326);
											s.is_empty()
										}
										['σ', 'σ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
											super::push(out, 2327);
											s.is_empty()
										}
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
		['η', 'ψ', 'ο', 'σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 744);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['ν', 'ν', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['ρ', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['λ', 'α', 'τ', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χοσ (3 syllables).
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1920);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2178);
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
				_ => false,
			}
		}
		['γ', 'υ', 'ρ', 'ρ', 'υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['ρ', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['ξ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 753);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['ν', 'ω', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1923);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2180);
									match s {
										[] => true,
										['ρ', 'τ', '\u{1}', s @ ..] => {
											super::push(out, 2329);
											s.is_empty()
										}
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
					super::push(out, 1453);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['ρ', 'ε', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['τ', 'π', 'ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αδιψοσ (3 syllables).
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	s.is_empty()
}

/// The word υπερωοσ (3 syllables).
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	s.is_empty()
}

/// The word γλωσσοποιοσ (3 syllables).
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1971);
	s.is_empty()
}

/// Words ending -πιοσ (3 syllables).
pub(crate) fn n647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 647);
	match s {
		[] => true,
		['μ', 'υ', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1138);
			s.is_empty()
		}
		['ο', 'τ', 'ν', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1139);
			s.is_empty()
		}
		['ρ', 'ο', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1140);
			s.is_empty()
		}
		['ω', 'ν', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριοσ (3 syllables).
pub(crate) fn n648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['υ', 'ο', 'ρ', 'β', 'ε', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', 'μ', 'ε', 'τ', 'π', 'ε', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		['δ', 'ν', 'ο', 'χ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1144);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['ν', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2194);
									s.is_empty()
								}
								['σ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
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
			super::push(out, 1146);
			match s {
				[] => true,
				['κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							match s {
								[] => true,
								['κ', 'ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2196);
									s.is_empty()
								}
								['λ', 'η', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'υ', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['η', 'ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2198);
									s.is_empty()
								}
								['ι', 'ρ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 2199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1975);
							match s {
								[] => true,
								['λ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2335);
											s.is_empty()
										}
										['ω', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 2336);
											s.is_empty()
										}
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
		['θ', 'ι', 'α', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['ι', 'ε', 'χ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1148);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['π', 'μ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1616);
					match s {
						[] => true,
						['γ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σιοσ (3 syllables).
pub(crate) fn n649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['π', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1152);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['τ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1154);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1155);
			match s {
				[] => true,
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1156);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1157);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1977);
							match s {
								[] => true,
								['ν', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['κ', 'ε', 'α', '\u{1}', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1980);
							match s {
								[] => true,
								['ν', 'ο', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1981);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2202);
									match s {
										[] => true,
										['χ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 2337);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', 'α', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 2203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', 'τ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1158);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τιοσ (3 syllables).
pub(crate) fn n650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['π', 'ο', 'ρ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1160);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['υ', 'ο', 'ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'υ', 'γ', 'ι', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1162);
			s.is_empty()
		}
		['σ', 'υ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		['τ', 'α', 'λ', 'α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 1164);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιταφιοσ (3 syllables).
pub(crate) fn n651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	s.is_empty()
}

/// Words ending -χιοσ (3 syllables).
pub(crate) fn n652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['ι', 'λ', 'ι', 'ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['υ', 'ν', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μονικοσ (3 syllables).
pub(crate) fn n2002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1991);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 2206);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2338);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρονικοσ (3 syllables).
pub(crate) fn n2003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1992);
	match s {
		[] => true,
		['χ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 2207);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word πορνικοσ (3 syllables).
pub(crate) fn n1653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1648);
	s.is_empty()
}

/// Words ending -ωνικοσ (3 syllables).
pub(crate) fn n1654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1649);
	match s {
		[] => true,
		['ι', '\u{1}', s @ ..] => {
			super::push(out, 1993);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οικοσ (3 syllables).
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 1650);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πικοσ (3 syllables).
pub(crate) fn n1181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['ο', 'ι', 'θ', 'ι', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρικοσ (3 syllables).
pub(crate) fn n1182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['τ', 'σ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1654);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['ο', 'λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', 'χ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1655);
			s.is_empty()
		}
		['ι', 'ε', 'σ', 'ο', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1656);
			s.is_empty()
		}
		['τ', 'ε', 'μ', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1657);
			s.is_empty()
		}
		['υ', 'σ', 'σ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σικοσ (3 syllables).
pub(crate) fn n1183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1659);
			match s {
				[] => true,
				['φ', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τικοσ (3 syllables).
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1660);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2210);
							match s {
								[] => true,
								['ε', 'ν', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', 'ι', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['ξ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'σ', 'α', 'τ', 'α', 'κ', 'ο', 'π', 'α', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['θ', s @ ..] => {
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2213);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2341);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2394);
											s.is_empty()
										}
										['π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 2395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2215);
							match s {
								[] => true,
								['β', '\u{1}', s @ ..] => {
									super::push(out, 2342);
									s.is_empty()
								}
								['μ', 'μ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'α', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1662);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['ο', 'ρ', 'φ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2003);
					match s {
						[] => true,
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2004);
					match s {
						[] => true,
						['α', 'π', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1663);
			match s {
				[] => true,
				['υ', 'ε', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1664);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2006);
					s.is_empty()
				}
				['ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1665);
			match s {
				[] => true,
				['α', 'τ', 'σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['ε', 'π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1666);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2010);
					match s {
						[] => true,
						['λ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2221);
							match s {
								[] => true,
								['σ', 'υ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2222);
							match s {
								[] => true,
								['ε', 'κ', 'σ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2011);
					match s {
						[] => true,
						['ν', 'ω', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2012);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2013);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 2014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['π', 'ο', 'κ', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υικοσ (3 syllables).
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['θ', 'χ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		['κ', 'ε', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 1672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φικοσ (3 syllables).
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['α', 'ρ', 'γ', 'ο', 'ρ', 'ω', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 1673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χικοσ (3 syllables).
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	s.is_empty()
}

/// Words ending -λκοσ (3 syllables).
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['υ', 'ο', 'γ', 'υ', 'ζ', '\u{1}', s @ ..] => {
			super::push(out, 1187);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκοσ (3 syllables).
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	s.is_empty()
}

/// Words ending -ρκοσ (3 syllables).
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	s.is_empty()
}

/// Words ending -υκοσ (3 syllables).
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	s.is_empty()
}

/// Words ending -μενοσ (3 syllables).
pub(crate) fn n1263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1262);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['η', 'ν', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2235);
							s.is_empty()
						}
						['ι', 'ν', 'α', 'φ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2236);
							s.is_empty()
						}
						['υ', 'λ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2237);
							s.is_empty()
						}
						['ω', 'λ', s @ ..] => {
							super::push(out, 2238);
							match s {
								[] => true,
								['υ', 'ο', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2034);
					match s {
						[] => true,
						['σ', 'ι', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2349);
									s.is_empty()
								}
								['π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2350);
									s.is_empty()
								}
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
			super::push(out, 1725);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['ρ', 'φ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['τ', 'ε', 'τ', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2036);
					match s {
						[] => true,
						['α', 'δ', 'ε', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2242);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2351);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				['υ', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1726);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2243);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', 'ε', 'κ', 'σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['δ', 'ε', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ε', s @ ..] => {
					super::push(out, 2040);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2246);
							match s {
								[] => true,
								['ε', 'γ', '\u{1}', s @ ..] => {
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
					super::push(out, 2041);
					match s {
						[] => true,
						['ο', 'φ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2248);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
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
			super::push(out, 1728);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2249);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 2356);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2397);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2357);
									match s {
										[] => true,
										['π', 'υ', '\u{1}', s @ ..] => {
											super::push(out, 2398);
											s.is_empty()
										}
										['ρ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 2399);
											s.is_empty()
										}
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
			super::push(out, 1729);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2044);
					match s {
						[] => true,
						['θ', 'ε', 'τ', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2251);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2358);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2400);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', 'σ', 'ε', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2045);
					match s {
						[] => true,
						['λ', 'ι', 'ε', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						['ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'λ', 'α', 'κ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2047);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2254);
							match s {
								[] => true,
								['σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 2048);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2255);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2361);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2401);
											match s {
												[] => true,
												['α', '\u{1}', s @ ..] => {
													super::push(out, 2406);
													s.is_empty()
												}
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
				['δ', s @ ..] => super::t0::n2065(s, out),
				['ε', 'γ', 'ω', 'γ', 'α', 'ρ', 'κ', 'ε', 'ν', '\u{1}', s @ ..] => super::t0::n2066(s, out),
				['ζ', s @ ..] => super::t0::n2067(s, out),
				['θ', 'ι', s @ ..] => super::t0::n2068(s, out),
				['κ', 'σ', s @ ..] => super::t0::n2069(s, out),
				['λ', s @ ..] => super::t0::n2070(s, out),
				['ν', s @ ..] => super::t0::n2071(s, out),
				['π', s @ ..] => super::t0::n2072(s, out),
				['ρ', s @ ..] => super::t0::n2073(s, out),
				['σ', s @ ..] => super::t0::n2074(s, out),
				['υ', s @ ..] => super::t0::n2075(s, out),
				['φ', s @ ..] => super::t0::n2076(s, out),
				['χ', s @ ..] => super::t0::n2077(s, out),
				['ψ', 'ο', s @ ..] => super::t0::n2078(s, out),
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t0::n1736(s, out),
		['σ', s @ ..] => super::t0::n1737(s, out),
		['υ', s @ ..] => super::t0::n1738(s, out),
		['ω', s @ ..] => super::t0::n1739(s, out),
		_ => false,
	}
}

/// Words ending -ρηνοσ (3 syllables).
pub(crate) fn n1270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	match s {
		[] => true,
		['α', 'κ', 'ι', 'ψ', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1742);
			s.is_empty()
		}
		['υ', 'μ', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ινοσ (3 syllables).
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 686);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['β', 'μ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						['γ', 'γ', 'ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1271);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'μ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['γ', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1752);
					match s {
						[] => true,
						['ω', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1275);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['ρ', 'υ', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['φ', '\u{1}', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['χ', 'ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['μ', '\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1277);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['ρ', 'κ', 'γ', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['μ', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['π', 'η', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2297);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2386);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2404);
													s.is_empty()
												}
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
					super::push(out, 1766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['α', 'λ', 'α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2097);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2299);
									match s {
										[] => true,
										['ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
											super::push(out, 2387);
											s.is_empty()
										}
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
			super::push(out, 1281);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['π', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['υ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1772);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['α', 'φ', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['ε', 'λ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2300);
									s.is_empty()
								}
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
			super::push(out, 1283);
			match s {
				[] => true,
				['ξ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				['ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['η', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μνοσ (3 syllables).
pub(crate) fn n687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['α', 'ρ', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ρ', 'π', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1288);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word τυραννοσ (3 syllables).
pub(crate) fn n688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	s.is_empty()
}

/// Words ending -ονοσ (3 syllables).
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1289);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1291);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2104);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2301);
									s.is_empty()
								}
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
			super::push(out, 1294);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πνοσ (3 syllables).
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['α', 'κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1295);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1296);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word δασυστερνοσ (3 syllables).
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	s.is_empty()
}

/// The word πελιτνοσ (3 syllables).
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 692);
	s.is_empty()
}

/// Words ending -υνοσ (3 syllables).
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2105);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', 'υ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χνοσ (3 syllables).
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ι', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνοσ (3 syllables).
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 1303);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1304);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αμφαριστεροσ (3 syllables).
pub(crate) fn n2428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2405);
	s.is_empty()
}

/// Words ending -θροσ (3 syllables).
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιροσ (3 syllables).
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1343);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κροσ (3 syllables).
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', 'λ', 'ο', 'τ', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οροσ (3 syllables).
pub(crate) fn n718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1829);
					match s {
						[] => true,
						['α', 'ι', 'δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1832);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2131);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2132);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1834);
					match s {
						[] => true,
						['γ', 'ι', 'τ', 'σ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2133);
							s.is_empty()
						}
						['φ', 'η', 'ψ', '\u{1}', s @ ..] => {
							super::push(out, 2134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word φιλοκοπροσ (3 syllables).
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 719);
	s.is_empty()
}

/// Words ending -ρροσ (3 syllables).
pub(crate) fn n720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 720);
	s.is_empty()
}

/// Words ending -τροσ (3 syllables).
pub(crate) fn n721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 721);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1352);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υροσ (3 syllables).
pub(crate) fn n722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 722);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1355);
			s.is_empty()
		}
		['ζ', 'ι', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['α', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['ι', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1358);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		['τ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1363);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1841);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ελαφροσ (3 syllables).
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 723);
	s.is_empty()
}

/// The word αμυσχροσ (3 syllables).
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 724);
	s.is_empty()
}

/// Words ending -ωροσ (3 syllables).
pub(crate) fn n725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 725);
	match s {
		[] => true,
		['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 1366);
			s.is_empty()
		}
		['λ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1367);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2136);
							match s {
								[] => true,
								['λ', 'α', 'τ', 'ο', 'κ', 'ι', 'δ', 'ο', 'τ', 'ν', 'α', 'φ', 'ο', 'κ', 'υ', 'σ', 'ο', 'τ', 'ι', 'ο', 'φ', 'ο', 'ρ', 'θ', 'ρ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 2310);
									s.is_empty()
								}
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

/// Words ending -κλειτοσ (3 syllables).
pub(crate) fn n2169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2152);
	s.is_empty()
}

/// Words ending -λιτοσ (3 syllables).
pub(crate) fn n1399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μιτοσ (3 syllables).
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	match s {
		[] => true,
		['α', 'ξ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιτοσ (3 syllables).
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1400);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 1879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κτοσ (3 syllables).
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1402);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['λ', 'ο', 'ν', 'ι', 'ο', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1405);
			s.is_empty()
		}
		['ω', 'ρ', 'π', 'υ', 'ρ', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντοσ (3 syllables).
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οτοσ (3 syllables).
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 736);
	match s {
		[] => true,
		['β', 'υ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1409);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1410);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2153);
							s.is_empty()
						}
						['μ', 'α', '\u{1}', s @ ..] => {
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

/// Words ending -πτοσ (3 syllables).
pub(crate) fn n737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 737);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1413);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['θ', 'ο', 'μ', 'μ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				['σ', 'υ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτοσ (3 syllables).
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 738);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				['τ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'κ', 'γ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 1417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στοσ (3 syllables).
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 739);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1892);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1419);
			match s {
				[] => true,
				['β', 'σ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1420);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['β', 'σ', 'ε', 'ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['γ', s @ ..] => {
					match s {
						[] => true,
						['ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['ο', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1899);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1901);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2319);
									s.is_empty()
								}
								['κ', 'α', 'μ', 'ι', 'ε', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2320);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 2321);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 2391);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2163);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2322);
									s.is_empty()
								}
								['φ', '\u{1}', s @ ..] => {
									super::push(out, 2323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'ε', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2164);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1904);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2167);
							s.is_empty()
						}
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						['ι', 'ε', 'τ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2170);
							match s {
								[] => true,
								['ι', 'χ', 'σ', 'ι', 'κ', 'α', 'ρ', 'τ', 'ε', 'τ', 'ο', 'ι', 'ρ', 'υ', 'μ', 'σ', 'ι', 'κ', 'α', 'ξ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 2324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1907);
					match s {
						[] => true,
						['η', 'ο', 'δ', 'γ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 2172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 1909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ττοσ (3 syllables).
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 1425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υτοσ (3 syllables).
pub(crate) fn n741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1911);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2175);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2325);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2392);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', 'ι', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1428);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1429);
			s.is_empty()
		}
		['ρ', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτοσ (3 syllables).
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1431);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1913);
					match s {
						[] => true,
						['ρ', 'ι', 'ε', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1914);
					match s {
						[] => true,
						['μ', 'χ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1433);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['π', 'ο', 'τ', 'σ', 'ο', 'κ', 'η', 'μ', 'ο', 'δ', 'β', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δομενοσ (3 syllables).
pub(crate) fn n2065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2049);
	s.is_empty()
}

/// The word νεκραγωγεομενοσ (3 syllables).
pub(crate) fn n2066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2050);
	s.is_empty()
}

/// Words ending -ζομενοσ (3 syllables).
pub(crate) fn n2067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2051);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['γ', 'ρ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2257);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['λ', 'α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['ο', 'ν', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2402);
							s.is_empty()
						}
						['ω', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 2403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιθομενοσ (3 syllables).
pub(crate) fn n2068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2052);
	match s {
		[] => true,
		['ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 2259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σκομενοσ (3 syllables).
pub(crate) fn n2069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2053);
	match s {
		[] => true,
		['ω', 'ν', 'γ', 'ι', 'γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2260);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λομενοσ (3 syllables).
pub(crate) fn n2070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2054);
	match s {
		[] => true,
		['υ', 'ο', 'β', '\u{1}', s @ ..] => {
			super::push(out, 2261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νομενοσ (3 syllables).
pub(crate) fn n2071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2055);
	match s {
		[] => true,
		['α', 'θ', 'ν', 'υ', 'π', '\u{1}', s @ ..] => {
			super::push(out, 2262);
			s.is_empty()
		}
		['γ', 'ι', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 2263);
			s.is_empty()
		}
		['ε', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 2264);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πομενοσ (3 syllables).
pub(crate) fn n2072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2056);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρομενοσ (3 syllables).
pub(crate) fn n2073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2057);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['φ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σομενοσ (3 syllables).
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2058);
	match s {
		[] => true,
		['ε', '\u{1}', s @ ..] => {
			super::push(out, 2269);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 2270);
			match s {
				[] => true,
				['θ', 'η', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				['ν', 'ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υομενοσ (3 syllables).
pub(crate) fn n2075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2059);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2271);
			match s {
				[] => true,
				['λ', 'υ', 'ο', 'β', '\u{1}', s @ ..] => {
					super::push(out, 2372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['ρ', 'κ', 'γ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φομενοσ (3 syllables).
pub(crate) fn n2076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2060);
	match s {
		[] => true,
		['α', 'ρ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 2274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χομενοσ (3 syllables).
pub(crate) fn n2077(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ρ', 'ε', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οψομενοσ (3 syllables).
pub(crate) fn n2078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2061);
	match s {
		[] => true,
		['π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρμενοσ (3 syllables).
pub(crate) fn n1736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1731);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σμενοσ (3 syllables).
pub(crate) fn n1737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1732);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2063);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['ι', 'β', 'ε', 'β', '\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				['μ', 'η', 'σ', 'ε', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['υ', 'ε', 'κ', 'σ', 'ε', s @ ..] => {
					match s {
						[] => true,
						['ρ', s @ ..] => {
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
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['ε', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμενοσ (3 syllables).
pub(crate) fn n1738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1733);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2065);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2066);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2284);
					match s {
						[] => true,
						['ι', 'ε', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['β', 'ο', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['γ', 'η', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['η', 'ο', 'β', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 2383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ε', 'μ', 'η', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2288);
					s.is_empty()
				}
				['κ', 'ι', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['ο', '\u{1}', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ο', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2069);
			s.is_empty()
		}
		['τ', 'ρ', 'η', 'ξ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωμενοσ (3 syllables).
pub(crate) fn n1739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1734);
	match s {
		[] => true,
		['δ', 'υ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2072);
			match s {
				[] => true,
				['ε', 'μ', 'η', 'σ', 'ε', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2073);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2075);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

