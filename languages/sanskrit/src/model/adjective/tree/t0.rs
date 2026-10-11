use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['क', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ज', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['ट', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 272);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 43);
					match s {
						[] => true,
						['औ', s @ ..] => {
							super::push(out, 275);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 276);
							s.is_empty()
						}
						['\u{947}', s @ ..] => {
							super::push(out, 277);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['\u{94d}', 'र', '\u{942}', s @ ..] => {
							super::push(out, 279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 45);
					match s {
						[] => true,
						['ज', '\u{1}', s @ ..] => {
							super::push(out, 280);
							s.is_empty()
						}
						['ा', 'ख', '\u{1}', s @ ..] => {
							super::push(out, 281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['आ', s @ ..] => {
							super::push(out, 282);
							s.is_empty()
						}
						['\u{942}', s @ ..] => {
							super::push(out, 283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 47);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 284);
							match s {
								[] => true,
								['न', s @ ..] => {
									super::push(out, 595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['\u{94d}', 'ण', s @ ..] => {
							super::push(out, 286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 287);
							match s {
								[] => true,
								['न', s @ ..] => {
									super::push(out, 596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 288);
							s.is_empty()
						}
						['\u{942}', s @ ..] => {
							super::push(out, 289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['\u{947}', s @ ..] => {
							super::push(out, 290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['\u{947}', s @ ..] => {
							super::push(out, 291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['\u{942}', s @ ..] => {
							super::push(out, 292);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 55);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 56);
					match s {
						[] => true,
						['न', '\u{1}', s @ ..] => {
							super::push(out, 294);
							s.is_empty()
						}
						['प', '\u{1}', s @ ..] => {
							super::push(out, 295);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 296);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									match s {
										[] => true,
										['ष', s @ ..] => {
											super::push(out, 786);
											match s {
												[] => true,
												['\u{941}', 'य', '\u{1}', s @ ..] => {
													super::push(out, 856);
													s.is_empty()
												}
												_ => false,
											}
										}
										['स', 'अ', '\u{1}', s @ ..] => {
											super::push(out, 787);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 297);
							match s {
								[] => true,
								['\u{94d}', 'र', 'ा', 'त', '\u{1}', s @ ..] => {
									super::push(out, 597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ज', 'ा', s @ ..] => {
							super::push(out, 298);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 299);
							s.is_empty()
						}
						['थ', s @ ..] => {
							super::push(out, 300);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 301);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ध', s @ ..] => {
							super::push(out, 302);
							match s {
								[] => true,
								['अ', s @ ..] => {
									super::push(out, 599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 303);
							match s {
								[] => true,
								['ध', s @ ..] => {
									super::push(out, 600);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['प', s @ ..] => {
							super::push(out, 304);
							match s {
								[] => true,
								['\u{942}', s @ ..] => {
									super::push(out, 602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['म', s @ ..] => {
							super::push(out, 305);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 306);
							match s {
								[] => true,
								['ा', 'क', s @ ..] => {
									super::push(out, 604);
									s.is_empty()
								}
								['ी', s @ ..] => {
									super::push(out, 605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ल', s @ ..] => {
							super::push(out, 307);
							match s {
								[] => true,
								['ा', 'न', '\u{1}', s @ ..] => {
									super::push(out, 606);
									s.is_empty()
								}
								['\u{941}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							super::push(out, 308);
							s.is_empty()
						}
						['ष', s @ ..] => {
							super::push(out, 309);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => super::t0::n327(s, out),
						['ह', s @ ..] => {
							super::push(out, 311);
							match s {
								[] => true,
								['ा', 'व', s @ ..] => super::t0::n638(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ी', s @ ..] => super::t0::n58(s, out),
				['\u{941}', s @ ..] => super::t0::n59(s, out),
				['\u{943}', s @ ..] => super::t0::n60(s, out),
				['\u{947}', s @ ..] => super::t0::n61(s, out),
				['\u{948}', s @ ..] => super::t0::n62(s, out),
				['ो', s @ ..] => super::t0::n63(s, out),
				['\u{94d}', s @ ..] => super::t0::n64(s, out),
				_ => false,
			}
		}
		['ख', s @ ..] => super::t0::n2(s, out),
		['ग', s @ ..] => super::t0::n3(s, out),
		['घ', s @ ..] => super::t0::n4(s, out),
		['च', s @ ..] => super::t0::n5(s, out),
		['छ', s @ ..] => super::t0::n6(s, out),
		['ज', s @ ..] => super::t0::n7(s, out),
		['ञ', s @ ..] => super::t0::n8(s, out),
		['ट', s @ ..] => super::t0::n9(s, out),
		['ठ', s @ ..] => super::t0::n10(s, out),
		['ड', s @ ..] => super::t0::n11(s, out),
		['ढ', s @ ..] => super::t0::n12(s, out),
		['ण', s @ ..] => super::t0::n13(s, out),
		['त', s @ ..] => super::t0::n14(s, out),
		['थ', s @ ..] => super::t0::n15(s, out),
		['द', s @ ..] => super::t0::n16(s, out),
		['ध', s @ ..] => super::t0::n17(s, out),
		['न', s @ ..] => super::t0::n18(s, out),
		['प', s @ ..] => super::t0::n19(s, out),
		['फ', s @ ..] => super::t0::n20(s, out),
		['ब', s @ ..] => super::t0::n21(s, out),
		['भ', s @ ..] => super::t0::n22(s, out),
		['म', s @ ..] => super::t0::n23(s, out),
		['य', s @ ..] => super::t0::n24(s, out),
		['र', s @ ..] => super::t0::n25(s, out),
		['ल', s @ ..] => super::t0::n26(s, out),
		['व', s @ ..] => super::t0::n27(s, out),
		['श', s @ ..] => super::t0::n28(s, out),
		['ष', s @ ..] => super::t0::n29(s, out),
		['स', s @ ..] => super::t0::n30(s, out),
		['ह', s @ ..] => super::t0::n31(s, out),
		['ा', s @ ..] => super::t0::n32(s, out),
		['ि', s @ ..] => super::t0::n33(s, out),
		['ी', s @ ..] => super::t0::n34(s, out),
		['\u{941}', s @ ..] => super::t0::n35(s, out),
		['\u{942}', s @ ..] => super::t0::n36(s, out),
		['\u{943}', s @ ..] => super::t0::n37(s, out),
		['\u{94d}', s @ ..] => super::t0::n38(s, out),
		_ => false,
	}
}

/// Words ending -सिक.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -वाहिक.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	s.is_empty()
}

/// Words ending -ीक.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	s.is_empty()
}

/// Words ending -ुक.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ृक.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['त', 'ा', 'म', '\u{1}', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ेक.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	s.is_empty()
}

/// Words ending -ैक.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ोक.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	s.is_empty()
}

/// Words ending -्क.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['ङ', 'व', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ख.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['\u{941}', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 316);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 611);
							s.is_empty()
						}
						['व', '\u{94d}', 'ध', '\u{94d}', 'र', 'ऊ', '\u{1}', s @ ..] => {
							super::push(out, 612);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 613);
							s.is_empty()
						}
						['\u{94d}', 'ङ', 'ि', 'द', '\u{1}', s @ ..] => {
							super::push(out, 614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ग.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['भ', '\u{94d}', 'र', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 67);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ङ', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 615);
							match s {
								[] => true,
								['\u{941}', 'त', 'च', '\u{1}', s @ ..] => {
									super::push(out, 789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['श', s @ ..] => {
							super::push(out, 616);
							match s {
								[] => true,
								['ि', 'प', '\u{1}', s @ ..] => {
									super::push(out, 790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{943}', s @ ..] => {
							super::push(out, 617);
							match s {
								[] => true,
								['श', 'ः', '\u{941}', 'त', 'च', '\u{1}', s @ ..] => {
									super::push(out, 791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{948}', s @ ..] => {
							super::push(out, 618);
							match s {
								[] => true,
								['ल', '\u{1}', s @ ..] => {
									super::push(out, 792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 619);
							match s {
								[] => true,
								['म', '\u{1}', s @ ..] => {
									super::push(out, 793);
									s.is_empty()
								}
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

/// Words ending -घ.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['र', 'ी', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -च.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['\u{94d}', 'च', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['उ', '\u{1}', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -छ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['\u{94d}', 'च', 'अ', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ज.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['\u{94d}', 'ब', '\u{941}', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['क', 'य', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ञ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	s.is_empty()
}

/// Words ending -ट.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['ल', s @ ..] => {
					super::push(out, 325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['फ', s @ ..] => {
					super::push(out, 326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ौ', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['ष', '\u{941}', s @ ..] => {
					super::push(out, 328);
					match s {
						[] => true,
						['त', '\u{902}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ठ.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['\u{94d}', 'ष', s @ ..] => {
			match s {
				[] => true,
				['ष', s @ ..] => {
					super::push(out, 329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 330);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					super::push(out, 331);
					match s {
						[] => true,
						['फ', s @ ..] => {
							super::push(out, 622);
							match s {
								[] => true,
								['\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 794);
									s.is_empty()
								}
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

/// Words ending -ड.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['ा', 'र', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ौ', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['ग', '\u{1}', s @ ..] => {
					super::push(out, 333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ढ.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['\u{942}', 'म', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 334);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ण.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['प', '\u{94d}', 'र', 'त', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', 'य', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['\u{94d}', 'र', 'आ', '\u{1}', s @ ..] => {
					super::push(out, 336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['ा', 'र', 'ा', 'न', '\u{1}', s @ ..] => {
					super::push(out, 337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['क', '\u{1}', s @ ..] => {
					super::push(out, 339);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['आ', '\u{1}', s @ ..] => {
							super::push(out, 625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['ल', '\u{1}', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['ा', 'र', '\u{94d}', 'श', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['\u{942}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['\u{947}', 'श', 'ि', 'व', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						['र', 'ह', '\u{1}', s @ ..] => {
							super::push(out, 627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['य', '\u{94d}', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['ल', 'क', '\u{1}', s @ ..] => {
							super::push(out, 628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['ष', '\u{94d}', 'क', 'द', '\u{1}', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['त', '\u{1}', s @ ..] => {
							super::push(out, 630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['र', s @ ..] => {
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 631);
							match s {
								[] => true,
								['\u{941}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							super::push(out, 632);
							match s {
								[] => true,
								['ौ', s @ ..] => {
									super::push(out, 796);
									s.is_empty()
								}
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

/// Words ending -त.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['ऋ', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['र', '\u{948}', 'न', '\u{1}', s @ ..] => {
							super::push(out, 633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 355);
					match s {
						[] => true,
						['व', 'ए', '\u{1}', s @ ..] => {
							super::push(out, 634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['ग', 'ा', 'भ', '\u{1}', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				['\u{948}', 'र', '\u{1}', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['\u{94d}', 'र', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['ण', s @ ..] => {
					super::push(out, 361);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{943}', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['ि', 'च', 'ा', 'न', '\u{1}', s @ ..] => {
							super::push(out, 636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['क', 'ा', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['श', s @ ..] => {
							super::push(out, 637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 638);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['य', s @ ..] => {
							super::push(out, 639);
							match s {
								[] => true,
								['ज', '\u{1}', s @ ..] => {
									super::push(out, 798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ा', s @ ..] => {
							super::push(out, 640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 641);
							match s {
								[] => true,
								['म', '\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 799);
									s.is_empty()
								}
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

/// Words ending -थ.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['त', '\u{94d}', 'प', 'स', '\u{1}', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['श', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['र', '\u{941}', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['त', 'च', '\u{1}', s @ ..] => {
							super::push(out, 642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -द.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['प', 'ि', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', 'ब', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['ा', 'श', '\u{1}', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ध.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['\u{948}', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['व', '\u{1}', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ौ', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['द', 'ा', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['र', '\u{94d}', s @ ..] => {
							super::push(out, 643);
							match s {
								[] => true,
								['श', '\u{1}', s @ ..] => {
									super::push(out, 800);
									s.is_empty()
								}
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

/// Words ending -न.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['ऊ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		['घ', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			match s {
				[] => true,
				['\u{942}', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['ल', '\u{1}', s @ ..] => {
							super::push(out, 644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'ञ', 'ा', s @ ..] => {
					super::push(out, 378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 379);
					s.is_empty()
				}
				['\u{94d}', 'ञ', 'र', s @ ..] => {
					super::push(out, 380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', '\u{94d}', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['र', 'व', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 645);
							s.is_empty()
						}
						['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						['ि', 'र', 'प', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 801);
									s.is_empty()
								}
								['ि', 'व', '\u{1}', s @ ..] => {
									super::push(out, 802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'द', 'उ', '\u{1}', s @ ..] => {
							super::push(out, 647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 381);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					super::push(out, 382);
					s.is_empty()
				}
				['\u{94d}', 'र', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			match s {
				[] => true,
				['ो', 'ब', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['\u{94d}', 'न', s @ ..] => {
					super::push(out, 385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 110);
			s.is_empty()
		}
		['प', 'ा', 'थ', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['म', 'न', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['ल', 'ा', s @ ..] => {
			super::push(out, 114);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', '\u{94d}', 'र', 'द', '\u{941}', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 387);
					s.is_empty()
				}
				['व', '\u{94d}', 'श', '\u{1}', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				['ो', 'म', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['त', '\u{948}', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['ध', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', 'ि', 'व', '\u{1}', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['य', 'व', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', 'व', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['ी', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				['ग', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{948}', s @ ..] => {
			super::push(out, 120);
			s.is_empty()
		}
		['ौ', s @ ..] => {
			super::push(out, 121);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['ग', 'ा', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['र', '\u{94d}', 'द', '\u{94d}', 'न', 'ऐ', '\u{1}', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['घ', '\u{941}', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -प.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 123);
			s.is_empty()
		}
		['\u{942}', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['स', '\u{1}', s @ ..] => {
							super::push(out, 652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 404);
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 803);
									s.is_empty()
								}
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

/// Words ending -फ.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	s.is_empty()
}

/// Words ending -ब.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	s.is_empty()
}

/// Words ending -भ.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['ौ', 'स', '\u{1}', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 654);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 129);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['र', 'द', '\u{948}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -म.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['च', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['\u{94d}', 'ञ', 'प', '\u{1}', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['\u{94d}', 'ष', 'अ', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['ौ', 'ग', '\u{1}', s @ ..] => {
					super::push(out, 411);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['त', 'श', '\u{902}', 'ि', 'र', '\u{94d}', 'त', '\u{1}', s @ ..] => {
							super::push(out, 656);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 657);
							s.is_empty()
						}
						['प', 'स', '\u{1}', s @ ..] => {
							super::push(out, 658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['\u{94d}', 'र', 'ा', s @ ..] => {
					super::push(out, 412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['\u{94d}', 'र', 'ि', 'न', '\u{1}', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 136);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['न', '\u{1}', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['\u{94d}', 'क', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['क', 'र', s @ ..] => {
					super::push(out, 417);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 419);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['ल', '\u{1}', s @ ..] => {
							super::push(out, 660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 422);
					match s {
						[] => true,
						['\u{94d}', 'ग', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', 'ष', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['\u{94d}', 'क', '\u{1}', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 424);
					s.is_empty()
				}
				['ग', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', 'प', '\u{1}', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['\u{948}', 'र', '\u{94d}', 'ग', '\u{1}', s @ ..] => {
							super::push(out, 663);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 665);
							s.is_empty()
						}
						['ि', 'ज', '\u{1}', s @ ..] => {
							super::push(out, 666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -य.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['ज', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 428);
					match s {
						[] => true,
						['त', 'क', '\u{1}', s @ ..] => {
							super::push(out, 667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['\u{94d}', 'ण', 'र', 'ि', 'ह', '\u{1}', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['ष', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 670);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 437);
					match s {
						[] => true,
						['\u{94d}', 'प', s @ ..] => {
							super::push(out, 672);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 439);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 673);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['ण', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 443);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 445);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 675);
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 806);
									s.is_empty()
								}
								['त', '\u{948}', 'म', '\u{1}', s @ ..] => {
									super::push(out, 807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 447);
					match s {
						[] => true,
						['उ', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 676);
							s.is_empty()
						}
						['ो', s @ ..] => {
							match s {
								[] => true,
								['य', s @ ..] => {
									super::push(out, 808);
									match s {
										[] => true,
										['अ', s @ ..] => {
											super::push(out, 857);
											s.is_empty()
										}
										_ => false,
									}
								}
								['र', 'आ', '\u{1}', s @ ..] => {
									super::push(out, 809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'र', 'व', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['घ', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ण', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 450);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 679);
							match s {
								[] => true,
								['\u{94d}', 'त', 'य', s @ ..] => {
									super::push(out, 810);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				['ध', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['ो', s @ ..] => {
							super::push(out, 680);
							match s {
								[] => true,
								['य', '\u{94d}', 'न', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['ि', 'ज', 'ऋ', '\u{1}', s @ ..] => {
							super::push(out, 681);
							s.is_empty()
						}
						['\u{942}', s @ ..] => {
							super::push(out, 682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['भ', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['ग', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 683);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 684);
							s.is_empty()
						}
						['ा', 'क', '\u{1}', s @ ..] => {
							super::push(out, 685);
							s.is_empty()
						}
						['ो', 'स', '\u{1}', s @ ..] => {
							super::push(out, 686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', 'र', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['\u{94d}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['व', 'र', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['\u{94d}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['व', '\u{1}', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						['ो', 'ग', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -र.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['ष', s @ ..] => {
							super::push(out, 693);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['ग', 'स', 'न', 'म', '\u{94d}', 'ङ', 'ा', 'व', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['इ', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', '\u{94d}', 'न', '\u{941}', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['ा', 'व', '\u{1}', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['\u{94d}', 'श', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['ऐ', '\u{1}', s @ ..] => {
							super::push(out, 697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', 'ा', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['व', '\u{1}', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['ा', 'न', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['फ', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['\u{94d}', 'श', 'ा', 'क', '\u{1}', s @ ..] => {
							super::push(out, 702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['ा', 'श', '\u{1}', s @ ..] => {
							super::push(out, 703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			match s {
				[] => true,
				['त', 'ा', 'म', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['\u{948}', 'व', '\u{94d}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['थ', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['ा', 'म', '\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['प', '\u{94d}', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['र', 'ा', 'क', '\u{1}', s @ ..] => {
							super::push(out, 706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{948}', 'स', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ौ', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['क', 'व', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ग', s @ ..] => {
					super::push(out, 487);
					match s {
						[] => true,
						['उ', s @ ..] => {
							super::push(out, 709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ट', '\u{94d}', 'ष', 'ा', 'र', '\u{94d}', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['भ', '\u{1}', s @ ..] => {
							super::push(out, 710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 489);
					match s {
						[] => true,
						['\u{948}', s @ ..] => {
							super::push(out, 711);
							match s {
								[] => true,
								['म', '\u{1}', s @ ..] => {
									super::push(out, 812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					match s {
						[] => true,
						['\u{941}', 'र', s @ ..] => {
							super::push(out, 712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'न', s @ ..] => {
							match s {
								[] => true,
								['ऐ', s @ ..] => {
									super::push(out, 814);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 859);
											s.is_empty()
										}
										_ => false,
									}
								}
								['च', s @ ..] => {
									super::push(out, 815);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 816);
									match s {
										[] => true,
										['च', '\u{1}', s @ ..] => {
											super::push(out, 860);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 490);
					match s {
						[] => true,
						['ि', 'ष', s @ ..] => {
							super::push(out, 713);
							match s {
								[] => true,
								['\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['भ', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['र', 'औ', '\u{1}', s @ ..] => {
							super::push(out, 714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ह', s @ ..] => {
							super::push(out, 716);
							match s {
								[] => true,
								['स', '\u{1}', s @ ..] => {
									super::push(out, 819);
									s.is_empty()
								}
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

/// Words ending -ल.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['क', '\u{1}', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['ज', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{948}', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['श', '\u{1}', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -व.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['\u{94d}', 'र', 'ा', 'भ', '\u{1}', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['\u{94d}', 'ण', 'ा', 'प', '\u{1}', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['\u{94d}', 'ष', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['\u{94d}', 'न', 'ऐ', '\u{1}', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['\u{94d}', 'न', '\u{948}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['ौ', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 184);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['श', '\u{1}', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 186);
			s.is_empty()
		}
		['ी', 'र', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['\u{94d}', 'ग', 'ल', 'ी', 'न', '\u{1}', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['द', '\u{1}', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{948}', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['\u{948}', s @ ..] => {
							super::push(out, 717);
							match s {
								[] => true,
								['ब', '\u{1}', s @ ..] => {
									super::push(out, 820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 512);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['ऋ', '\u{1}', s @ ..] => {
							super::push(out, 719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -श.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['ई', '\u{1}', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		['ण', '\u{942}', 'द', '\u{1}', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['ि', 'ल', 'ा', 'ब', '\u{1}', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['ी', 'न', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['\u{943}', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ष.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['ि', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['ह', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['म', '\u{1}', s @ ..] => {
							super::push(out, 721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['प', 'व', '\u{1}', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		['ौ', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						['ऋ', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						['य', '\u{94d}', 'त', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						['ल', 'ब', '\u{1}', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						['ा', 'त', '\u{94d}', 'क', 'र', '\u{1}', s @ ..] => {
							super::push(out, 728);
							s.is_empty()
						}
						['\u{943}', 'द', 'स', '\u{1}', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', 'आ', '\u{1}', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -स.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['न', 'ा', 'म', '\u{1}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['ल', 'आ', '\u{1}', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['ष', '\u{94d}', 'क', 'ा', 'र', '\u{1}', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['ह', 'ा', 'स', '\u{1}', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['ा', 'प', '\u{94d}', 'र', 'ा', 'क', '\u{1}', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['ी', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ह.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['म', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['ा', 'व', '\u{1}', s @ ..] => {
							super::push(out, 730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['द', '\u{948}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['म', '\u{94d}', 'र', 'ि', 'न', '\u{1}', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['र', '\u{1}', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['ल', '\u{1}', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['ळ', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ा.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['प', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ि.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['च', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['ऋ', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['\u{94d}', 'र', 'ि', 'न', '\u{1}', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['\u{94d}', 'द', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['\u{941}', 'ब', '\u{94d}', 'र', 'ि', 'न', '\u{1}', s @ ..] => {
							super::push(out, 732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['ा', 'र', '\u{94d}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['\u{94d}', 'ह', 'ा', 'र', '\u{94d}', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['\u{942}', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['भ', '\u{1}', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['ी', 'र', '\u{94d}', 'व', '\u{941}', 'ह', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ी.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['ढ', '\u{942}', 'द', '\u{1}', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['थ', 'र', '\u{1}', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 234);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 734);
							s.is_empty()
						}
						['श', s @ ..] => {
							super::push(out, 735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ु.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['घ', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['ऋ', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['प', '\u{1}', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['व', '\u{94d}', 'स', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 821);
									s.is_empty()
								}
								['\u{941}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['न', s @ ..] => {
							match s {
								[] => true,
								['छ', s @ ..] => {
									super::push(out, 823);
									s.is_empty()
								}
								['ि', 'ह', '\u{1}', s @ ..] => {
									super::push(out, 824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['ा', 'स', '\u{1}', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['ा', 'घ', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['\u{94d}', 'र', 'ी', 'द', '\u{1}', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 825);
									s.is_empty()
								}
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['प', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ळ', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['ी', 'व', '\u{1}', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 254);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['\u{902}', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				['ब', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ू.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['ज', '\u{1}', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['स', '\u{1}', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ृ.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['त', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['च', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['ञ', s @ ..] => {
							super::push(out, 742);
							match s {
								[] => true,
								['द', s @ ..] => {
									super::push(out, 827);
									s.is_empty()
								}
								['य', s @ ..] => {
									super::push(out, 828);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['ज', s @ ..] => {
							super::push(out, 744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['द', '\u{1}', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['\u{943}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['ऋ', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{943}', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['र', 'ा', 'ह', '\u{941}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 756);
							match s {
								[] => true,
								['य', 'त', 'ा', 'य', '\u{1}', s @ ..] => {
									super::push(out, 831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 757);
							match s {
								[] => true,
								['भ', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 758);
							match s {
								[] => true,
								['ग', s @ ..] => {
									super::push(out, 833);
									match s {
										[] => true,
										['\u{941}', 'य', '\u{1}', s @ ..] => {
											super::push(out, 861);
											s.is_empty()
										}
										_ => false,
									}
								}
								['त', s @ ..] => {
									super::push(out, 834);
									s.is_empty()
								}
								['र', s @ ..] => {
									super::push(out, 835);
									match s {
										[] => true,
										['श', '\u{1}', s @ ..] => {
											super::push(out, 862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 759);
							match s {
								[] => true,
								['य', '\u{94d}', 'र', 'त', 'ा', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ी', 'प', '\u{1}', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 761);
							match s {
								[] => true,
								['य', '\u{1}', s @ ..] => {
									super::push(out, 837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['\u{943}', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						['ग', s @ ..] => {
							super::push(out, 766);
							match s {
								[] => true,
								['ा', s @ ..] => {
									super::push(out, 838);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['त', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 768);
							match s {
								[] => true,
								['\u{947}', 'व', '\u{94d}', s @ ..] => {
									super::push(out, 840);
									match s {
										[] => true,
										['र', '\u{941}', 'त', 'च', '\u{1}', s @ ..] => {
											super::push(out, 863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						['प', 'ा', s @ ..] => {
							super::push(out, 770);
							match s {
								[] => true,
								['र', '\u{94d}', 'त', 'व', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['म', s @ ..] => {
							super::push(out, 771);
							match s {
								[] => true,
								['\u{947}', 'ष', '\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 842);
									s.is_empty()
								}
								['\u{94d}', 'र', 'ध', '\u{1}', s @ ..] => {
									super::push(out, 843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['य', s @ ..] => super::t0::n805(s, out),
						['र', s @ ..] => super::t0::n806(s, out),
						['ल', s @ ..] => super::t0::n807(s, out),
						['व', '\u{94d}', s @ ..] => super::t0::n808(s, out),
						['ष', s @ ..] => super::t0::n809(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => super::t0::n283(s, out),
		['श', s @ ..] => super::t0::n284(s, out),
		['ष', s @ ..] => super::t0::n285(s, out),
		['स', s @ ..] => super::t0::n286(s, out),
		['ह', s @ ..] => super::t0::n287(s, out),
		_ => false,
	}
}

/// Words ending -यिन्.
pub(crate) fn n805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['\u{947}', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['प', 'ज', 'ा', 'व', '\u{1}', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -रिन्.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['ा', 'स', '\u{94d}', 'त', '\u{1}', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -लिन्.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	s.is_empty()
}

/// Words ending -्विन्.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['\u{942}', 'प', '\u{1}', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -षिन्.
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 775);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 867);
							match s {
								[] => true,
								['ङ', 'ा', 'क', 'आ', '\u{1}', s @ ..] => {
									super::push(out, 868);
									s.is_empty()
								}
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

/// Words ending -प्.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	s.is_empty()
}

/// Words ending -श्.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['\u{943}', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['द', 'स', '\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ष्.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['ि', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -स्.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['त', s @ ..] => {
			super::push(out, 582);
			s.is_empty()
		}
		['द', 'ा', 'श', 'ि', 'र', '\u{1}', s @ ..] => {
			super::push(out, 583);
			s.is_empty()
		}
		['ध', 'ा', 'र', 'ि', 'त', 'ी', 'व', '\u{1}', s @ ..] => {
			super::push(out, 584);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['म', s @ ..] => {
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						['आ', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['\u{941}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						['\u{94d}', 'र', '\u{941}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['ध', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 588);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['त', '\u{1}', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', 'ो', 'ज', 'स', '\u{1}', s @ ..] => {
			super::push(out, 590);
			s.is_empty()
		}
		['ह', s @ ..] => {
			super::push(out, 591);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['\u{947}', 'न', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 592);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ह्.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['ि', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		_ => false,
	}
}

