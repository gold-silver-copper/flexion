use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['उ', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['त', 'ि', 'त', '\u{1}', s @ ..] => {
					super::push(out, 44);
					s.is_empty()
				}
				_ => false,
			}
		}
		['क', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 45);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ए', '\u{1}', s @ ..] => {
					super::push(out, 46);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				['ज', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['ी', s @ ..] => {
							super::push(out, 450);
							match s {
								[] => true,
								['ब', '\u{1}', s @ ..] => {
									super::push(out, 1205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ट', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 451);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 452);
							match s {
								[] => true,
								['प', '\u{1}', s @ ..] => {
									super::push(out, 1206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'ष', s @ ..] => {
							super::push(out, 453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ठ', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['ड', 'ख', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ण', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 455);
							match s {
								[] => true,
								['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['\u{94d}', 'स', '\u{941}', s @ ..] => {
							super::push(out, 456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['थ', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['\u{94d}', 'त', 'ल', '\u{941}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 458);
							s.is_empty()
						}
						['उ', s @ ..] => {
							super::push(out, 459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					super::push(out, 56);
					match s {
						[] => true,
						['\u{94d}', 'र', s @ ..] => {
							super::push(out, 460);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 461);
							s.is_empty()
						}
						['ड', s @ ..] => {
							super::push(out, 462);
							match s {
								[] => true,
								['ी', 'र', '\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ा', 'थ', s @ ..] => {
							super::push(out, 463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 464);
							s.is_empty()
						}
						['\u{942}', s @ ..] => {
							super::push(out, 465);
							match s {
								[] => true,
								['र', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1769);
											s.is_empty()
										}
										['प', 'उ', '\u{1}', s @ ..] => {
											super::push(out, 1770);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ब', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['\u{94d}', 'म', '\u{941}', 'ट', s @ ..] => {
							super::push(out, 467);
							match s {
								[] => true,
								['\u{941}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['\u{94d}', 'र', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => super::t0::n61(s, out),
				['र', s @ ..] => super::t0::n62(s, out),
				['ल', s @ ..] => super::t0::n63(s, out),
				['व', s @ ..] => {
					match s {
						[] => true,
						['क', s @ ..] => super::t0::n536(s, out),
						['ा', s @ ..] => super::t0::n537(s, out),
						['\u{947}', s @ ..] => super::t0::n538(s, out),
						_ => false,
					}
				}
				['श', s @ ..] => super::t0::n65(s, out),
				['ष', s @ ..] => super::t0::n66(s, out),
				['स', s @ ..] => super::t0::n67(s, out),
				['ह', s @ ..] => super::t0::n68(s, out),
				['ा', s @ ..] => super::t0::n69(s, out),
				['ि', s @ ..] => super::t0::n70(s, out),
				['ी', s @ ..] => super::t0::n71(s, out),
				['\u{941}', s @ ..] => super::t0::n72(s, out),
				['\u{942}', s @ ..] => super::t0::n73(s, out),
				['\u{943}', s @ ..] => super::t0::n74(s, out),
				['\u{947}', s @ ..] => super::t0::n75(s, out),
				['ो', s @ ..] => super::t0::n76(s, out),
				['\u{94d}', s @ ..] => super::t0::n77(s, out),
				_ => false,
			}
		}
		['ख', s @ ..] => super::t0::n3(s, out),
		['ग', s @ ..] => super::t0::n4(s, out),
		['घ', s @ ..] => super::t0::n5(s, out),
		['ङ', s @ ..] => super::t0::n6(s, out),
		['च', s @ ..] => super::t0::n7(s, out),
		['छ', s @ ..] => super::t0::n8(s, out),
		['ज', s @ ..] => super::t0::n9(s, out),
		['ञ', s @ ..] => super::t0::n10(s, out),
		['ट', s @ ..] => super::t0::n11(s, out),
		['ठ', s @ ..] => super::t0::n12(s, out),
		['ड', s @ ..] => super::t0::n13(s, out),
		['ढ', s @ ..] => super::t0::n14(s, out),
		['ण', s @ ..] => super::t0::n15(s, out),
		['त', s @ ..] => super::t0::n16(s, out),
		['थ', s @ ..] => super::t0::n17(s, out),
		['द', s @ ..] => super::t0::n18(s, out),
		['ध', s @ ..] => super::t0::n19(s, out),
		['न', s @ ..] => super::t0::n20(s, out),
		['प', s @ ..] => super::t0::n21(s, out),
		['फ', s @ ..] => super::t0::n22(s, out),
		['ब', s @ ..] => super::t0::n23(s, out),
		['भ', s @ ..] => super::t0::n24(s, out),
		['म', s @ ..] => super::t0::n25(s, out),
		['य', s @ ..] => super::t0::n26(s, out),
		['र', s @ ..] => super::t0::n27(s, out),
		['ल', s @ ..] => super::t0::n28(s, out),
		['व', s @ ..] => super::t0::n29(s, out),
		['श', s @ ..] => super::t0::n30(s, out),
		['ष', s @ ..] => super::t0::n31(s, out),
		['स', s @ ..] => super::t0::n32(s, out),
		['ह', s @ ..] => super::t0::n33(s, out),
		['ा', s @ ..] => super::t0::n34(s, out),
		['ि', s @ ..] => super::t0::n35(s, out),
		['ी', s @ ..] => super::t0::n36(s, out),
		['\u{941}', s @ ..] => super::t0::n37(s, out),
		['\u{942}', s @ ..] => super::t0::n38(s, out),
		['\u{943}', s @ ..] => super::t0::n39(s, out),
		['\u{948}', s @ ..] => super::t0::n40(s, out),
		['ो', s @ ..] => super::t0::n41(s, out),
		['ौ', s @ ..] => super::t0::n42(s, out),
		['\u{94d}', s @ ..] => super::t0::n43(s, out),
		_ => false,
	}
}

/// Words ending -यक.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ण', 'र', 'आ', '\u{1}', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['ा', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -रक.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', 'म', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['ा', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -लक.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['फ', s @ ..] => {
			super::push(out, 473);
			s.is_empty()
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['ज', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['त', '\u{1}', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', 'ग', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -कवक.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	s.is_empty()
}

/// Words ending -ावक.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['\u{94d}', 'श', '\u{1}', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ेवक.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	s.is_empty()
}

/// Words ending -शक.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	s.is_empty()
}

/// Words ending -षक.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['\u{94d}', 'र', 'ी', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['श', '\u{1}', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -सक.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// Words ending -हक.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['ा', 'द', 'ि', 'व', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['\u{941}', 'क', '\u{1}', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ाक.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['ट', 'त', '\u{1}', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['ि', 'प', '\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -िक.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['\u{94d}', 'र', 'ा', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['ज', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['\u{94d}', 'ञ', 'ा', 'क', '\u{1}', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', '\u{94d}', 'ष', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['ौ', 'प', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['\u{948}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['ो', 'ब', '\u{948}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['\u{94d}', 'ङ', 'ा', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		['स', 'ा', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['न', '\u{941}', 'न', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ीक.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['त', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['ड', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ुक.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['ट', 'क', '\u{1}', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['ड', '\u{94d}', 'ण', 'उ', '\u{1}', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['थ', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['द', '\u{94d}', 'न', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						['द', 'ा', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', 'ा', 'क', '\u{1}', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		['ल', 'आ', '\u{1}', s @ ..] => {
			super::push(out, 507);
			s.is_empty()
		}
		['श', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['\u{902}', s @ ..] => {
					match s {
						[] => true,
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						['ा', 'द', 'ा', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						['ि', 'श', '\u{1}', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', '\u{94d}', 'त', 'उ', '\u{1}', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ूक.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	s.is_empty()
}

/// Words ending -ृक.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['स', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ेक.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ोक.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['त', '\u{1}', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', 'ि', 'र', '\u{94d}', 'त', '\u{1}', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्क.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['क', '\u{943}', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['ङ', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['य', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['\u{94d}', 'र', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ख.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['ः', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 79);
			s.is_empty()
		}
		['\u{941}', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1242);
							match s {
								[] => true,
								['\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', 'न', 'ो', 'य', '\u{1}', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						['ी', s @ ..] => {
							super::push(out, 1244);
							match s {
								[] => true,
								['र', '\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 1783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'ङ', 'ि', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['ङ', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ग.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['ल', '\u{1}', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['उ', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['ड', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['त', '\u{1}', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['य', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{943}', s @ ..] => {
			super::push(out, 88);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 89);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['य', 'म', '\u{94d}', 'र', 'क', '\u{1}', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ङ', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 1248);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['भ', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 1251);
							match s {
								[] => true,
								['ल', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['द', '\u{947}', 'व', '\u{1}', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', 'ल', '\u{1}', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 1255);
							match s {
								[] => true,
								['श', '\u{1}', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{943}', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['श', '\u{1}', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{948}', s @ ..] => {
							super::push(out, 1257);
							match s {
								[] => true,
								['ल', '\u{1}', s @ ..] => {
									super::push(out, 1789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['\u{941}', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['द', '\u{1}', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
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
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['अ', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['ङ', s @ ..] => {
					super::push(out, 542);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ङ.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	s.is_empty()
}

/// Words ending -च.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['ो', 'म', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ञ', 'ौ', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['श', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -छ.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['\u{941}', 'त', s @ ..] => {
							super::push(out, 1259);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ञ', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['ि', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ज.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['ट', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['ड', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['\u{94d}', 'ड', 'ह', '\u{1}', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', '\u{947}', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['प', '\u{1}', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['\u{947}', 'भ', '\u{1}', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', 'ि', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['व', '\u{94d}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', 'र', 'ह', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', 'ब', '\u{1}', s @ ..] => {
			super::push(out, 102);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ञ', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['ब', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ञ.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			super::push(out, 103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ट.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['क', s @ ..] => {
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['स', '\u{1}', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', '\u{94d}', 'र', 'क', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['ल', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['ल', '\u{1}', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', 'र', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['ि', 'क', '\u{1}', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['ट', s @ ..] => {
					match s {
						[] => true,
						['घ', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						['स', '\u{1}', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						['ि', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ण', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['ष', s @ ..] => {
					match s {
						[] => true,
						['द', '\u{1}', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						['\u{941}', 'ज', '\u{1}', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							match s {
								[] => true,
								['क', s @ ..] => {
									super::push(out, 1792);
									s.is_empty()
								}
								['द', 'र', '\u{941}', 'द', '\u{1}', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								['प', '\u{94d}', 'स', 'ः', '\u{941}', 'द', '\u{1}', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								['म', '\u{1}', s @ ..] => {
									super::push(out, 1795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{947}', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
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
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['ी', s @ ..] => {
			super::push(out, 109);
			s.is_empty()
		}
		['\u{94d}', 'ष', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['प', 'न', 'ि', 'ठ', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ो', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ड.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['\u{947}', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['ड', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				['ण', s @ ..] => {
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['म', 'आ', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1277);
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 1796);
									s.is_empty()
								}
								['भ', '\u{943}', 'त', 'ा', 'र', '\u{94d}', 'भ', '\u{1}', s @ ..] => {
									super::push(out, 1797);
									s.is_empty()
								}
								['म', '\u{94d}', 'ष', '\u{941}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						['\u{941}', 'त', '\u{1}', s @ ..] => {
							super::push(out, 1279);
							s.is_empty()
						}
						['\u{947}', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ढ.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['ी', 'म', '\u{1}', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['\u{943}', s @ ..] => {
			super::push(out, 114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ण.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		['क', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				['य', 'ो', 'त', '\u{1}', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['ङ', 'ो', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['\u{94d}', 'ङ', 'अ', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', '\u{1}', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['प', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['र', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['फ', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['र', '\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1283);
							match s {
								[] => true,
								['आ', s @ ..] => {
									super::push(out, 1799);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['श', '\u{1}', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'ह', 'ा', 'र', '\u{94d}', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', 'ा', 'र', '\u{94d}', 'द', '\u{94d}', 'न', 'ा', 'च', '\u{1}', s @ ..] => {
			super::push(out, 121);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['ई', '\u{1}', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['उ', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['क', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						['म', 'ा', 'न', '\u{1}', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['ी', 'व', s @ ..] => {
							super::push(out, 1288);
							match s {
								[] => true,
								['\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 1800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['च', '\u{1}', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['त', '\u{1}', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						['आ', '\u{1}', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['ा', 'र', 'क', '\u{902}', 'श', '\u{1}', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', 'ध', 'ा', 'स', '\u{1}', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['ि', 'क', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['ल', '\u{1}', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['क', s @ ..] => {
									super::push(out, 1801);
									s.is_empty()
								}
								['घ', s @ ..] => {
									super::push(out, 1802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['क', '\u{943}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				['ग', 'व', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				['प', '\u{943}', 'क', '\u{1}', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['ब', '\u{1}', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['य', '\u{943}', 'त', 'ि', 'प', '\u{1}', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['\u{941}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							match s {
								[] => true,
								['घ', '\u{1}', s @ ..] => {
									super::push(out, 1803);
									s.is_empty()
								}
								['त', s @ ..] => {
									super::push(out, 1804);
									s.is_empty()
								}
								['प', s @ ..] => {
									super::push(out, 1805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', '\u{1}', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			match s {
				[] => true,
				['क', '\u{1}', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['र', 'ह', '\u{1}', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				['ष', '\u{94d}', 'क', 'द', '\u{1}', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', 'म', 'ा', 'र', '\u{94d}', 'ग', '\u{1}', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		['\u{941}', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['र', s @ ..] => {
					match s {
						[] => true,
						['ध', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{942}', 'त', '\u{1}', s @ ..] => {
			super::push(out, 129);
			s.is_empty()
		}
		['\u{943}', 'घ', '\u{1}', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['य', '\u{94d}', 'र', '\u{941}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['ग', '\u{941}', 'र', '\u{1}', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['म', '\u{943}', 'न', '\u{1}', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						['प', '\u{1}', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{941}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 1806);
									s.is_empty()
								}
								['ौ', 'स', '\u{1}', s @ ..] => {
									super::push(out, 1807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{942}', s @ ..] => {
							match s {
								[] => true,
								['ग', '\u{94d}', 'द', 'उ', '\u{1}', s @ ..] => {
									super::push(out, 1808);
									s.is_empty()
								}
								['प', '\u{1}', s @ ..] => {
									super::push(out, 1809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -त.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['ऋ', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['क', '\u{94d}', 'ङ', 'क', '\u{1}', s @ ..] => {
			super::push(out, 134);
			s.is_empty()
		}
		['ग', s @ ..] => {
			match s {
				[] => true,
				['\u{902}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['ा', s @ ..] => {
					match s {
						[] => true,
						['ण', 'र', 'श', '\u{1}', s @ ..] => {
							super::push(out, 1302);
							s.is_empty()
						}
						['थ', 'त', '\u{1}', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['त', '\u{1}', s @ ..] => {
			super::push(out, 135);
			s.is_empty()
		}
		['प', '\u{941}', 'श', 'ा', 'प', '\u{1}', s @ ..] => {
			super::push(out, 136);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 137);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['ा', 'भ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 139);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['ख', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['ज', 'ि', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['ड', s @ ..] => {
					match s {
						[] => true,
						['\u{947}', 'व', s @ ..] => {
							super::push(out, 1306);
							match s {
								[] => true,
								['\u{94d}', 'ष', '\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'ण', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ण', s @ ..] => {
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 1308);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1309);
							match s {
								[] => true,
								['फ', '\u{1}', s @ ..] => {
									super::push(out, 1811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['व', '\u{94d}', 'स', s @ ..] => {
							super::push(out, 1310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ह', '\u{1}', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['ष', s @ ..] => {
					match s {
						[] => true,
						['\u{941}', 'त', '\u{1}', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['\u{94d}', 'क', s @ ..] => {
							match s {
								[] => true,
								['र', '\u{1}', s @ ..] => {
									super::push(out, 1813);
									s.is_empty()
								}
								['ी', 'द', '\u{1}', s @ ..] => {
									super::push(out, 1814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['स', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						['ो', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['न', 'ि', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['व', '\u{1}', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 1318);
							match s {
								[] => true,
								['न', 'ी', 'च', 'ा', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['भ', '\u{94d}', 'द', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['य', 'न', s @ ..] => {
					super::push(out, 618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			match s {
				[] => true,
				['द', '\u{1}', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				['भ', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['य', '\u{94d}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{943}', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['ा', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						['\u{94d}', 'स', '\u{902}', 'स', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1816);
									s.is_empty()
								}
								['\u{941}', s @ ..] => {
									super::push(out, 1817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['भ', '\u{1}', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				['म', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['\u{948}', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['द', 'द', '\u{94d}', 'न', 'न', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1323);
							s.is_empty()
						}
						['\u{941}', 'ब', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						['\u{943}', 'व', '\u{1}', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['ङ', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						['ब', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						['ा', s @ ..] => {
							match s {
								[] => true,
								['प', s @ ..] => {
									super::push(out, 1818);
									s.is_empty()
								}
								['व', s @ ..] => {
									super::push(out, 1819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['\u{942}', 'प', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					match s {
						[] => true,
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 1329);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						['व', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1331);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 1332);
							s.is_empty()
						}
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
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['प', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['ा', 'न', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['अ', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['य', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['ि', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['ध', s @ ..] => {
									super::push(out, 1820);
									match s {
										[] => true,
										['\u{94d}', 'द', 'ि', 'स', '\u{1}', s @ ..] => {
											super::push(out, 2037);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ी', s @ ..] => {
							super::push(out, 1337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 1338);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -द.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['ा', 'क', '\u{1}', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				['\u{94d}', 'ङ', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['न', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['र', '\u{94d}', 'द', 'ा', 'भ', '\u{1}', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['ा', 'व', '\u{94d}', 'श', '\u{1}', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['ि', 'व', '\u{94d}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['\u{941}', 'म', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['\u{941}', 'क', '\u{1}', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 158);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ब', 'ा', 'र', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['\u{94d}', 'स', 'ह', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', 'ा', 'ह', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ध.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['ष', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['औ', '\u{1}', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', 'ग', '\u{1}', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['\u{941}', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['य', 'आ', '\u{1}', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['ौ', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['ब', 'र', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['\u{94d}', 'प', s @ ..] => {
									super::push(out, 1821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', 'अ', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
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
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['अ', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['इ', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['क', '\u{94d}', 'ङ', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['ट', '\u{1}', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', '\u{94d}', 'ङ', 'व', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['घ', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['च', s @ ..] => {
			match s {
				[] => true,
				['ो', 'र', 'ि', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['व', '\u{1}', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'ञ', 'ा', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['र', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['ञ', 'र', '\u{1}', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						['र', '\u{941}', s @ ..] => {
							super::push(out, 1348);
							match s {
								[] => true,
								['द', '\u{1}', s @ ..] => {
									super::push(out, 1822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ट', '\u{94d}', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['ट', 'प', '\u{1}', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['ी', 'र', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', '\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['द', s @ ..] => {
			match s {
				[] => true,
				['ओ', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				['ौ', 'स', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['\u{902}', 'ा', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'न', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['च', '\u{1}', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', '\u{94d}', 'र', 'व', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['ग', '\u{1}', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			match s {
				[] => true,
				['ज', '\u{1}', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['ह', '\u{1}', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			match s {
				[] => true,
				['त', '\u{1}', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['\u{94d}', 'म', 'क', '\u{1}', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['व', '\u{1}', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['ा', 'त', 'ा', 'व', '\u{1}', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['व', s @ ..] => {
			match s {
				[] => true,
				['प', '\u{1}', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', '\u{94d}', 'र', 'द', s @ ..] => {
			match s {
				[] => true,
				['र', '\u{942}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['स', '\u{1}', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', 'ो', 'म', '\u{1}', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['ञ', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['थ', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['\u{94d}', 'स', 'आ', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['द', 'उ', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					match s {
						[] => true,
						['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1359);
							s.is_empty()
						}
						['ि', 'व', '\u{902}', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['स', '\u{1}', s @ ..] => {
									super::push(out, 1824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['\u{94d}', 'व', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['व', '\u{94d}', 'श', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					match s {
						[] => true,
						['ई', '\u{1}', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['र', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1826);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['ज', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', 'म', '\u{1}', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			match s {
				[] => true,
				['च', 'ा', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', '\u{1}', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1367);
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
				['क', 'श', '\u{1}', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['थ', s @ ..] => {
					match s {
						[] => true,
						['ि', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						['\u{948}', 'म', 'त', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['\u{94d}', 'स', 'ह', '\u{1}', s @ ..] => {
									super::push(out, 1827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['फ', 'ि', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{948}', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['ज', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => super::t0::n210(s, out),
		_ => false,
	}
}

/// Words ending -प.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['ड', s @ ..] => {
			super::push(out, 181);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 183);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			match s {
				[] => true,
				['ध', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 186);
			s.is_empty()
		}
		['\u{941}', s @ ..] => {
			match s {
				[] => true,
				['ड', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{943}', s @ ..] => {
			super::push(out, 188);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['ष', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['क', 'र', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['\u{942}', 'श', '\u{1}', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['श', '\u{1}', s @ ..] => {
									super::push(out, 1828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['श', '\u{1}', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['\u{941}', 'प', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1829);
									s.is_empty()
								}
								['क', 'ा', 'श', '\u{1}', s @ ..] => {
									super::push(out, 1830);
									s.is_empty()
								}
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
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['\u{947}', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['श', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ब.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['म', s @ ..] => {
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1381);
							match s {
								[] => true,
								['\u{947}', 'ह', '\u{1}', s @ ..] => {
									super::push(out, 1831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 1382);
							match s {
								[] => true,
								['ब', 'ि', 'त', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{941}', 'ट', s @ ..] => {
							super::push(out, 1383);
							match s {
								[] => true,
								['\u{941}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['उ', '\u{1}', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -भ.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['इ', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['ट', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['\u{948}', 'क', '\u{1}', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['\u{941}', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['श', '\u{1}', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', 'र', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['म', s @ ..] => {
					match s {
						[] => true,
						['क', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						['ज', '\u{1}', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						['त', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 1834);
									s.is_empty()
								}
								['स', '\u{941}', 'क', 'ि', 'र', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -म.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['च', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['\u{94d}', 'ञ', 'प', '\u{1}', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['र', '\u{94d}', 'त', '\u{947}', 'ष', '\u{94d}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['ल', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['\u{94d}', 'क', 'ि', 'व', '\u{1}', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['ह', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						['ो', 'य', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['ग', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['\u{941}', 'य', '\u{1}', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ज', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 749);
					s.is_empty()
				}
				['ध', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['र', '\u{94d}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['र', '\u{94d}', 'ब', '\u{1}', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
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
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['अ', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['आ', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['च', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['ज', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['द', s @ ..] => {
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['\u{943}', 'ह', '\u{1}', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['ा', 'र', '\u{94d}', 'त', '\u{94d}', 'न', 'म', '\u{1}', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['\u{94d}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['\u{94d}', 'र', s @ ..] => {
							super::push(out, 1403);
							match s {
								[] => true,
								['प', '\u{1}', s @ ..] => {
									super::push(out, 1836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['त', '\u{94d}', 'न', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['\u{941}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 1837);
									s.is_empty()
								}
								['ट', '\u{94d}', 'ष', 'ा', 'र', '\u{1}', s @ ..] => {
									super::push(out, 1838);
									s.is_empty()
								}
								['त', 'ष', '\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1839);
									s.is_empty()
								}
								['द', s @ ..] => {
									super::push(out, 1840);
									s.is_empty()
								}
								_ => false,
							}
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
				['त', 'ि', 'व', '\u{94d}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 1409);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['\u{941}', 'ग', '\u{94d}', 'ङ', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', 'ध', 'ा', 'म', '\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['त', '\u{1}', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				['प', '\u{1}', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['ध', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['\u{94d}', 'ख', '\u{902}', 'स', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['\u{94d}', 'त', '\u{948}', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', 'त', '\u{1}', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => super::t0::n273(s, out),
		_ => false,
	}
}

/// Words ending -र.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['आ', '\u{1}', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['क', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['\u{94d}', 'ष', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['\u{941}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['श', '\u{1}', s @ ..] => {
							super::push(out, 1468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ग', 'न', s @ ..] => {
			super::push(out, 241);
			s.is_empty()
		}
		['च', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', '\u{94d}', 'ञ', 'प', '\u{1}', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['ट', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['ठ', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 1470);
							match s {
								[] => true,
								['य', '\u{94d}', 'व', '\u{1}', s @ ..] => {
									super::push(out, 1872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 1471);
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
				['उ', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', 'ा', 'र', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['ब', '\u{94d}', 'म', 'अ', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['ा', 'त', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			match s {
				[] => true,
				['ि', 'व', 'र', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['\u{94d}', 'श', 'ऐ', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['स', s @ ..] => {
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['व', '\u{94d}', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', 'ो', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['ध', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ग', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['च', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['च', 'उ', '\u{1}', s @ ..] => {
									super::push(out, 1873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', '\u{1}', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['व', '\u{94d}', s @ ..] => {
					super::push(out, 826);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 1479);
							match s {
								[] => true,
								['\u{94d}', 'द', 'म', '\u{1}', s @ ..] => {
									super::push(out, 1874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['ज', 'ऽ', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				['द', '\u{94d}', 'न', 'म', '\u{1}', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['ध', '\u{941}', 'र', '\u{1}', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			match s {
				[] => true,
				['त', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 836);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						['श', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1484);
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
					super::push(out, 839);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				['ध', '\u{94d}', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['\u{947}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['\u{948}', s @ ..] => {
			super::push(out, 253);
			s.is_empty()
		}
		['ो', 'छ', s @ ..] => {
			super::push(out, 254);
			s.is_empty()
		}
		['ौ', 'ष', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => super::t0::n300(s, out),
		_ => false,
	}
}

/// Words ending -ल.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['अ', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['ख', '\u{942}', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		['ग', '\u{94d}', 'ङ', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['र', '\u{1}', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['ा', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['छ', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		['ज', '\u{94d}', s @ ..] => {
			super::push(out, 262);
			s.is_empty()
		}
		['ट', s @ ..] => {
			match s {
				[] => true,
				['प', '\u{1}', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				['ा', 'प', '\u{1}', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['ी', 'प', '\u{1}', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				['\u{94d}', 'त', '\u{941}', 'प', '\u{1}', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['थ', '\u{94d}', 'स', '\u{1}', s @ ..] => {
			super::push(out, 265);
			s.is_empty()
		}
		['द', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 861);
					s.is_empty()
				}
				['क', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['\u{94d}', 'प', 'ि', 'प', '\u{1}', s @ ..] => {
					super::push(out, 863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['फ', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['\u{94d}', 'त', 'स', '\u{1}', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['ा', 'क', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 269);
			s.is_empty()
		}
		['ल', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['प', '\u{1}', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 270);
			s.is_empty()
		}
		['ह', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['क', '\u{902}', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['य', 'ा', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ज', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['ट', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['ि', 'न', '\u{1}', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ड', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['ढ', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['थ', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['थ', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['ब', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['ब', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ग', s @ ..] => {
					super::push(out, 888);
					s.is_empty()
				}
				['ब', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['ब', '\u{94d}', 'म', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['श', 'ि', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 274);
			s.is_empty()
		}
		['\u{948}', 'त', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['\u{94d}', 'ल', s @ ..] => {
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -व.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['घ', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['ज', '\u{94d}', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['ट', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['ड', s @ ..] => {
			super::push(out, 280);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['द', '\u{94d}', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['\u{948}', 'क', '\u{1}', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['ौ', 'ग', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', '\u{948}', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['ी', s @ ..] => {
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['\u{94d}', 'ग', 'य', 'ह', '\u{1}', s @ ..] => {
							super::push(out, 1520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['\u{948}', 'द', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 908);
					match s {
						[] => true,
						['प', '\u{1}', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 909);
					match s {
						[] => true,
						['ि', 'न', 'ज', s @ ..] => {
							super::push(out, 1522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'त', 'स', s @ ..] => {
							super::push(out, 1523);
							match s {
								[] => true,
								['ि', s @ ..] => {
									super::push(out, 1902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['ख', s @ ..] => {
							super::push(out, 1524);
							match s {
								[] => true,
								['ि', s @ ..] => {
									super::push(out, 1903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ल', 'ि', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['श', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['ऽ', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['र', '\u{94d}', 'ह', '\u{1}', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -श.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{902}', 'ि', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['ि', 'न', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['\u{94d}', 'र', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['\u{942}', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['द', '\u{1}', s @ ..] => {
							super::push(out, 1528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ष.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['ि', s @ ..] => {
			match s {
				[] => true,
				['म', 'आ', s @ ..] => {
					super::push(out, 918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', '\u{1}', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['र', s @ ..] => {
					match s {
						[] => true,
						['\u{941}', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						['ौ', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', 'क', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['य', '\u{1}', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1904);
									s.is_empty()
								}
								['त', s @ ..] => {
									super::push(out, 1905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ी', s @ ..] => {
							super::push(out, 1534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -स.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['\u{902}', 'ा', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['म', '\u{1}', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', 'म', '\u{1}', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['ा', 'प', '\u{1}', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['म', 'ा', 'त', '\u{1}', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['स', '\u{1}', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['ि', 'ग', '\u{94d}', 'ङ', 'ा', 'व', '\u{94d}', 'र', 'थ', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['य', '\u{1}', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', 'ा', 'स', '\u{1}', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['म', '\u{94d}', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', 'ि', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', '\u{1}', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['\u{941}', 'फ', '\u{941}', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['फ', '\u{1}', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ह.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['ग', 'व', s @ ..] => {
							super::push(out, 1538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', 'र', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['\u{943}', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['द', '\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['म', '\u{1}', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['ल', '\u{1}', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', 'ळ', 'ी', 'म', '\u{1}', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ा.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				['ि', s @ ..] => {
					match s {
						[] => true,
						['ट', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 1543);
							match s {
								[] => true,
								['र', '\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 1906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						['ष', s @ ..] => {
							super::push(out, 1546);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 1907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ी', 'ष', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['\u{942}', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', 'र', s @ ..] => {
					super::push(out, 949);
					match s {
						[] => true,
						['\u{947}', 'ष', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['त', 'ा', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'र', s @ ..] => {
					super::push(out, 951);
					match s {
						[] => true,
						['त', '\u{1}', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ख', '\u{947}', 'र', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', '\u{94d}', 'ङ', 'र', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['\u{941}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		['ज', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ञ', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		['ठ', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['\u{94d}', 'ष', '\u{947}', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['थ', 'र', '\u{1}', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['थ', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['\u{94d}', 'न', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['प', '\u{1}', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['ग', '\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['\u{94d}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['म', 'र', '\u{1}', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['ष', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['ा', 'र', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 1557);
							match s {
								[] => true,
								['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'र', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['त', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1559);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['त', '\u{943}', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['त', '\u{1}', s @ ..] => {
									super::push(out, 1909);
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
						['ख', s @ ..] => {
							super::push(out, 1562);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 1564);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						['व', '\u{943}', s @ ..] => {
							super::push(out, 1566);
							match s {
								[] => true,
								['त', 'ि', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ष', s @ ..] => {
							super::push(out, 1567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 340);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['श', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['र', '\u{94d}', 'ग', '\u{1}', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1570);
							match s {
								[] => true,
								['\u{942}', 'द', '\u{1}', s @ ..] => {
									super::push(out, 1911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ष', s @ ..] => {
							super::push(out, 1571);
							match s {
								[] => true,
								['\u{941}', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 343);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['\u{902}', 'ि', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['ह', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ि.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		['ख', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['स', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['ी', s @ ..] => {
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						['व', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					match s {
						[] => true,
						['च', '\u{1}', s @ ..] => {
							super::push(out, 1575);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1576);
							s.is_empty()
						}
						['श', '\u{1}', s @ ..] => {
							super::push(out, 1577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'र', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['\u{94d}', 'ष', s @ ..] => {
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 1579);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 1580);
							match s {
								[] => true,
								['भ', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 1913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{941}', 'म', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ढ', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['र', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				['ा', 'व', '\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['\u{943}', 'श', '\u{1}', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1582);
							match s {
								[] => true,
								['\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 1915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', 'ष', '\u{94d}', 'र', 'ा', 'प', '\u{1}', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['ऋ', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['च', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['\u{94d}', 'ञ', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 987);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1585);
							match s {
								[] => true,
								['स', 'ह', s @ ..] => {
									super::push(out, 1916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['य', '\u{1}', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', 'ञ', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['\u{94d}', 'ज', '\u{1}', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', 'ध', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['ी', 'द', 'र', '\u{94d}', 'स', 'ह', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['\u{942}', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1589);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['ह', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 997);
					match s {
						[] => true,
						['क', 'ो', s @ ..] => {
							super::push(out, 1591);
							match s {
								[] => true,
								['य', 'श', s @ ..] => {
									super::push(out, 1918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['त', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						['न', 'व', s @ ..] => {
							super::push(out, 1593);
							match s {
								[] => true,
								['भ', '\u{1}', s @ ..] => {
									super::push(out, 1919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 1594);
							match s {
								[] => true,
								['ग', s @ ..] => {
									super::push(out, 1920);
									s.is_empty()
								}
								['भ', s @ ..] => {
									super::push(out, 1921);
									s.is_empty()
								}
								['व', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2056);
											s.is_empty()
										}
										['\u{94d}', s @ ..] => {
											super::push(out, 2057);
											s.is_empty()
										}
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
		['थ', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['त', '\u{1}', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1596);
							match s {
								[] => true,
								['स', '\u{1}', s @ ..] => {
									super::push(out, 1922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 1598);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1923);
									s.is_empty()
								}
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
			super::push(out, 354);
			match s {
				[] => true,
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['\u{943}', 'ग', 'ि', 'द', '\u{94d}', 'र', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					match s {
						[] => true,
						['छ', '\u{1}', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						['व', '\u{1}', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['न', s @ ..] => {
							match s {
								[] => true,
								['न', '\u{1}', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								['\u{941}', 'त', '\u{1}', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', 'छ', '\u{1}', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['\u{902}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', 'म', 'स', '\u{1}', s @ ..] => super::t0::n1115(s, out),
				['ि', s @ ..] => super::t0::n1116(s, out),
				['ो', s @ ..] => super::t0::n1117(s, out),
				['\u{94d}', s @ ..] => super::t0::n1118(s, out),
				_ => false,
			}
		}
		['न', s @ ..] => super::t0::n414(s, out),
		['प', s @ ..] => super::t0::n415(s, out),
		['भ', s @ ..] => super::t0::n416(s, out),
		['म', s @ ..] => super::t0::n417(s, out),
		['य', s @ ..] => super::t0::n418(s, out),
		['र', s @ ..] => super::t0::n419(s, out),
		['ल', s @ ..] => super::t0::n420(s, out),
		['व', s @ ..] => super::t0::n421(s, out),
		['श', s @ ..] => super::t0::n422(s, out),
		['ष', s @ ..] => super::t0::n423(s, out),
		['स', 'अ', s @ ..] => super::t0::n424(s, out),
		['ह', s @ ..] => super::t0::n425(s, out),
		_ => false,
	}
}

/// Words ending -ी.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['ा', 'व', 'र', '\u{94d}', 'क', 'च', '\u{1}', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['\u{941}', 'ष', '\u{94d}', 'क', '\u{948}', 'भ', '\u{1}', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				['\u{943}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['झ', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['ठ', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['\u{94d}', 'ण', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['\u{941}', 'श', '\u{1}', s @ ..] => {
							super::push(out, 1630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['\u{94d}', 'म', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['ष', '\u{94d}', 'क', 'ल', '\u{1}', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['ा', 'ष', 'ा', 'क', '\u{1}', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['क', 'च', '\u{1}', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 1634);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									match s {
										[] => true,
										['न', s @ ..] => {
											super::push(out, 2059);
											match s {
												[] => true,
												['त', '\u{1}', s @ ..] => {
													super::push(out, 2093);
													s.is_empty()
												}
												_ => false,
											}
										}
										['स', s @ ..] => {
											super::push(out, 2060);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2094);
													s.is_empty()
												}
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
							super::push(out, 1635);
							match s {
								[] => true,
								['\u{94d}', 'न', 'त', '\u{1}', s @ ..] => {
									super::push(out, 1938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['श', s @ ..] => {
							super::push(out, 1636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ह', s @ ..] => {
							super::push(out, 1637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ल', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['घ', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['र', '\u{1}', s @ ..] => {
									super::push(out, 1941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['द', '\u{1}', s @ ..] => {
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
		['श', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['द', 'ो', 'र', '\u{1}', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ु.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['य', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 1943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['ग', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['ङ', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['य', s @ ..] => {
									super::push(out, 1944);
									s.is_empty()
								}
								['ि', 'ह', '\u{1}', s @ ..] => {
									super::push(out, 1945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ल', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['घ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', '\u{1}', s @ ..] => {
					super::push(out, 1077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['\u{94d}', 'ष', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['ो', 'र', '\u{94d}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['व', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['\u{947}', 'र', 'ह', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1084);
					match s {
						[] => true,
						['व', 'ी', 'ज', '\u{1}', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 1652);
							match s {
								[] => true,
								['म', '\u{1}', s @ ..] => {
									super::push(out, 1946);
									s.is_empty()
								}
								['व', '\u{1}', s @ ..] => {
									super::push(out, 1947);
									s.is_empty()
								}
								['ा', 'व', '\u{1}', s @ ..] => {
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
		['द', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['\u{943}', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['ज', '\u{1}', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						['द', '\u{1}', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						['भ', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 1659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['\u{94d}', 'त', '\u{1}', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['य', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1101);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 1662);
							match s {
								[] => true,
								['\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 1950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1664);
							match s {
								[] => true,
								['व', '\u{94d}', 'ध', 'अ', '\u{1}', s @ ..] => {
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
		['र', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['द', '\u{1}', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['श', s @ ..] => {
							super::push(out, 1666);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1953);
									s.is_empty()
								}
								['म', '\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 1954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				['ा', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ळ', 'ी', 'व', '\u{1}', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['श', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['इ', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 410);
			s.is_empty()
		}
		['ह', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['\u{902}', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['ा', 'र', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ू.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['ज', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['व', '\u{94d}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['\u{94d}', 'भ', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ृ.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['ट', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['ध', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['म', s @ ..] => {
							super::push(out, 1674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1956);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 1957);
									s.is_empty()
								}
								['ि', s @ ..] => {
									super::push(out, 1958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['य', '\u{1}', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['\u{94d}', 'भ', s @ ..] => {
									super::push(out, 1959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 1678);
							match s {
								[] => true,
								['\u{941}', 'द', '\u{1}', s @ ..] => {
									super::push(out, 1960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['स', '\u{1}', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['\u{94d}', 'न', 'ा', 'न', 'न', '\u{1}', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['\u{947}', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ै.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ो.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ौ.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['न', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['च', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['\u{94d}', 'ञ', 'य', '\u{94d}', 'र', 'ि', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['ण', '\u{94d}', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						['फ', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['स', s @ ..] => {
							super::push(out, 1685);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['ल', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['ई', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['ज', '\u{1}', s @ ..] => {
							super::push(out, 1686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['घ', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['आ', '\u{1}', s @ ..] => {
							super::push(out, 1688);
							s.is_empty()
						}
						['\u{94d}', 'र', s @ ..] => {
							super::push(out, 1689);
							match s {
								[] => true,
								['अ', '\u{1}', s @ ..] => {
									super::push(out, 1962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['\u{943}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 1151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['म', s @ ..] => {
							super::push(out, 1693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{943}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1964);
									s.is_empty()
								}
								['य', s @ ..] => {
									super::push(out, 1965);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2061);
											s.is_empty()
										}
										_ => false,
									}
								}
								['श', s @ ..] => {
									super::push(out, 1966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['प', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['क', 'न', '\u{1}', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 1158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1701);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['म', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1967);
									s.is_empty()
								}
								['\u{941}', s @ ..] => {
									super::push(out, 1968);
									s.is_empty()
								}
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
			super::push(out, 439);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['ब', s @ ..] => {
							super::push(out, 1705);
							match s {
								[] => true,
								['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 1969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => super::t0::n498(s, out),
		['प', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['ह', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['व', '\u{94d}', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => super::t0::n505(s, out),
		['ह', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['ा', 'प', 'उ', '\u{1}', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => super::t0::n1319(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -्न.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['ग', 'न', '\u{1}', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['घ', 'ि', 'व', '\u{1}', s @ ..] => {
			super::push(out, 703);
			s.is_empty()
		}
		['त', s @ ..] => {
			match s {
				[] => true,
				['य', s @ ..] => {
					super::push(out, 1370);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			match s {
				[] => true,
				['र', '\u{94d}', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 705);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 706);
			s.is_empty()
		}
		['श', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['ि', 'श', '\u{1}', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 708);
			s.is_empty()
		}
		['ह', 'ा', s @ ..] => {
			super::push(out, 709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्य.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 784);
			s.is_empty()
		}
		['ख', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['ा', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['य', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['ा', 'र', 'म', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['ा', 'र', '\u{1}', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ठ', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['ण', s @ ..] => {
							super::push(out, 1845);
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 2038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['प', s @ ..] => {
					match s {
						[] => true,
						['ण', 'ा', 'ग', '\u{1}', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						['ह', '\u{94d}', 'र', 'ा', 'ग', '\u{1}', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['थ', '\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 2039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['ा', 'म', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					match s {
						[] => true,
						['क', '\u{1}', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['भ', '\u{1}', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{948}', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['र', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1851);
							s.is_empty()
						}
						['स', 'ग', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				['र', '\u{1}', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 1428);
					match s {
						[] => true,
						['व', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 1853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['\u{948}', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['म', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['ा', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['ज', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						['\u{94d}', 'र', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						['\u{94d}', 'र', '\u{942}', 'म', '\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['\u{942}', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['न', '\u{1}', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['\u{94d}', 'र', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['आ', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						['च', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						['न', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						['व', '\u{94d}', 'ह', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 1862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2040);
									s.is_empty()
								}
								['\u{94d}', 'द', 'ष', 'ई', '\u{1}', s @ ..] => {
									super::push(out, 2041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{942}', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['स', '\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{948}', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['ौ', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['ट', 'ौ', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1454);
					match s {
						[] => true,
						['भ', '\u{94d}', 'म', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['ऋ', '\u{1}', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['व', '\u{1}', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				['ा', 'र', '\u{948}', 'न', '\u{1}', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				['\u{943}', 'द', '\u{1}', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					match s {
						[] => true,
						['द', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
								['\u{94d}', 'द', 'उ', '\u{1}', s @ ..] => {
									super::push(out, 2043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2044);
									s.is_empty()
								}
								['ि', 'त', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 2045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{948}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['ि', 'श', s @ ..] => {
					super::push(out, 1461);
					s.is_empty()
				}
				['\u{941}', 'न', 'म', s @ ..] => {
					super::push(out, 1462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => super::t0::n895(s, out),
		['ह', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['\u{943}', s @ ..] => super::t0::n1600(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -्र.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['क', s @ ..] => {
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['न', '\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					match s {
						[] => true,
						['च', '\u{1}', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						['श', '\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['घ', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		['ज', s @ ..] => {
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				['ऋ', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['व', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['\u{94d}', 'ष', s @ ..] => {
					match s {
						[] => true,
						['\u{902}', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						['उ', '\u{1}', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						['र', '\u{94d}', 'भ', '\u{1}', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['ढ', s @ ..] => {
			match s {
				[] => true,
				['\u{947}', 'म', 'ा', 'च', 'ी', 'न', '\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['\u{94d}', 'ण', '\u{947}', 'म', '\u{1}', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['ा', 'प', '\u{1}', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', '\u{94d}', 'क', 'न', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				['ा', s @ ..] => {
					match s {
						[] => true,
						['छ', s @ ..] => {
							super::push(out, 1882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['प', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['म', 'ा', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['ह', 'म', '\u{1}', s @ ..] => {
									super::push(out, 2047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['ी', 'श', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 2048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['म', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2049);
									s.is_empty()
								}
								['अ', s @ ..] => {
									super::push(out, 2050);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2090);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['र', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 1887);
							match s {
								[] => true,
								['\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 2051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ह', 'ौ', s @ ..] => {
							super::push(out, 1888);
							match s {
								[] => true,
								['द', '\u{1}', s @ ..] => {
									super::push(out, 2052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{942}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['\u{943}', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['\u{947}', 'व', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{948}', 'म', '\u{1}', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['त', s @ ..] => {
							match s {
								[] => true,
								['प', 'ज', s @ ..] => {
									super::push(out, 2053);
									match s {
										[] => true,
										['\u{94d}', 'र', '\u{942}', 'भ', '\u{1}', s @ ..] => {
											super::push(out, 2091);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ा', s @ ..] => {
									super::push(out, 2054);
									match s {
										[] => true,
										['छ', '\u{1}', s @ ..] => {
											super::push(out, 2092);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['न', 'म', s @ ..] => {
							super::push(out, 1891);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
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
			super::push(out, 852);
			match s {
				[] => true,
				['भ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						['अ', '\u{1}', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					match s {
						[] => true,
						['छ', '\u{1}', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						['र', 'ा', 'द', '\u{1}', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', 'म', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['न', '\u{947}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						['र', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			match s {
				[] => true,
				['\u{943}', 'ग', '\u{1}', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['ो', s @ ..] => {
					match s {
						[] => true,
						['र', '\u{1}', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						['ल', '\u{1}', s @ ..] => {
							super::push(out, 1899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', 'अ', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		['म', s @ ..] => {
			match s {
				[] => true,
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				['व', '\u{1}', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				['\u{942}', 'ध', '\u{1}', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['ी', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			match s {
				[] => true,
				['\u{902}', 'ि', 'ह', '\u{1}', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['घ', '\u{1}', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word समाधि.
pub(crate) fn n1115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	s.is_empty()
}

/// Words ending -िधि.
pub(crate) fn n1116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	s.is_empty()
}

/// Words ending -ोधि.
pub(crate) fn n1117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	match s {
		[] => true,
		['ब', '\u{1}', s @ ..] => {
			super::push(out, 1603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्धि.
pub(crate) fn n1118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	match s {
		[] => true,
		['द', s @ ..] => {
			match s {
				[] => true,
				['\u{941}', 'ब', '\u{94d}', s @ ..] => {
					super::push(out, 1926);
					match s {
						[] => true,
						['र', 'ि', 'न', '\u{1}', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{943}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['त', '\u{94d}', 'न', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -नि.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		['ज', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', '\u{94d}', 'ध', '\u{1}', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 1607);
					match s {
						[] => true,
						['इ', '\u{1}', s @ ..] => {
							super::push(out, 1930);
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
				['क', 'श', '\u{1}', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				['म', '\u{1}', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['य', '\u{1}', s @ ..] => {
					super::push(out, 1610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -पि.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		['ा', s @ ..] => {
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -भि.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['ा', 'न', '\u{1}', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -मि.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	match s {
		[] => true,
		['ि', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['त', '\u{1}', s @ ..] => {
					super::push(out, 1613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['भ', s @ ..] => {
					match s {
						[] => true,
						['\u{941}', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							super::push(out, 1932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{943}', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['क', '\u{1}', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['र', 'ऊ', '\u{1}', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				['श', 'र', '\u{1}', s @ ..] => {
					super::push(out, 1616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -यि.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['र', '\u{1}', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -रि.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['अ', '\u{1}', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		['क', '\u{1}', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['ह', '\u{1}', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['व', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1933);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			match s {
				[] => true,
				['ग', '\u{1}', s @ ..] => {
					super::push(out, 1617);
					s.is_empty()
				}
				['त', '\u{94d}', 'त', 'ि', 'त', '\u{1}', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['त', 'ा', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 1620);
					s.is_empty()
				}
				['भ', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				['श', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -लि.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['अ', '\u{1}', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['क', '\u{1}', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['ज', '\u{94d}', 'ञ', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 1032);
			s.is_empty()
		}
		['ट', s @ ..] => {
			super::push(out, 1033);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['ब', '\u{1}', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['म', '\u{94d}', 'ल', 'श', '\u{1}', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		['व', '\u{1}', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['ग', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['श', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						['म', 'ा', 'क', '\u{1}', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', '\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -वि.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		['छ', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['ट', s @ ..] => {
			super::push(out, 1042);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['र', 'द', '\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -शि.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -षि.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['म', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['ि', s @ ..] => {
			super::push(out, 1048);
			s.is_empty()
		}
		['\u{943}', s @ ..] => {
			super::push(out, 1049);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['क', 'अ', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -असि.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	s.is_empty()
}

/// Words ending -हि.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['अ', s @ ..] => {
			super::push(out, 1052);
			s.is_empty()
		}
		['ी', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -न्.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['ज', s @ ..] => {
			super::push(out, 1163);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 1164);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 1167);
			match s {
				[] => true,
				['ओ', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 1712);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 1971);
							match s {
								[] => true,
								['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 2062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['द', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['घ', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						['ण', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['थ', s @ ..] => {
							super::push(out, 1979);
							match s {
								[] => true,
								['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 2065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 1980);
							match s {
								[] => true,
								['ज', '\u{1}', s @ ..] => {
									super::push(out, 2066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{942}', s @ ..] => {
					super::push(out, 1715);
					match s {
						[] => true,
						['भ', '\u{1}', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						['य', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['ज', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1989);
							match s {
								[] => true,
								['प', s @ ..] => {
									super::push(out, 2067);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 2068);
									match s {
										[] => true,
										['प', '\u{1}', s @ ..] => {
											super::push(out, 2095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 1991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ो', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						['य', s @ ..] => {
							super::push(out, 1993);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 1994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ल', s @ ..] => {
							super::push(out, 1995);
							match s {
								[] => true,
								['\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 2071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						['घ', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						['ज', s @ ..] => {
							super::push(out, 1998);
							match s {
								[] => true,
								['अ', s @ ..] => {
									super::push(out, 2072);
									s.is_empty()
								}
								['ओ', s @ ..] => {
									super::push(out, 2073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['त', s @ ..] => {
							super::push(out, 1999);
							match s {
								[] => true,
								['ा', s @ ..] => {
									super::push(out, 2074);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 2075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['द', s @ ..] => {
							super::push(out, 2000);
							match s {
								[] => true,
								['ा', s @ ..] => {
									super::push(out, 2076);
									match s {
										[] => true,
										['व', '\u{94d}', 'स', '\u{1}', s @ ..] => {
											super::push(out, 2096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['न', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 2002);
							match s {
								[] => true,
								['द', s @ ..] => {
									super::push(out, 2077);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ध', '\u{1}', s @ ..] => {
									super::push(out, 2078);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 2079);
									match s {
										[] => true,
										['भ', '\u{1}', s @ ..] => {
											super::push(out, 2098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['श', s @ ..] => {
							super::push(out, 2003);
							match s {
								[] => true,
								['अ', s @ ..] => {
									super::push(out, 2080);
									s.is_empty()
								}
								['र', s @ ..] => {
									super::push(out, 2081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ष', s @ ..] => {
							super::push(out, 2004);
							match s {
								[] => true,
								['\u{947}', s @ ..] => {
									super::push(out, 2082);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 2083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 2005);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => super::t0::n1283(s, out),
		['ष', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['\u{943}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => super::t0::n1873(s, out),
				_ => false,
			}
		}
		['ह', s @ ..] => super::t0::n1285(s, out),
		['ि', s @ ..] => super::t0::n1286(s, out),
		_ => false,
	}
}

/// Words ending -स्.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 1183);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['आ', s @ ..] => {
			super::push(out, 1184);
			s.is_empty()
		}
		['ज', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 1186);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['ऊ', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 1188);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 1190);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['\u{94d}', 'द', '\u{94d}', 'न', 'च', '\u{1}', s @ ..] => {
							super::push(out, 2024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 1191);
			match s {
				[] => true,
				['\u{947}', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['र', '\u{94d}', 'श', '\u{1}', s @ ..] => {
							super::push(out, 2025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['स', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['\u{94d}', 'प', 'अ', s @ ..] => {
							super::push(out, 2026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['श', 'ग', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 1193);
			s.is_empty()
		}
		['श', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 1749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'क', 'र', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 1196);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 1197);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ि', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['श', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['आ', '\u{1}', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['ध', '\u{1}', s @ ..] => {
							super::push(out, 2033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['य', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 1759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ुह्.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1204);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 1765);
			s.is_empty()
		}
		['ड', s @ ..] => {
			super::push(out, 1766);
			s.is_empty()
		}
		['द', 'ि', 'त', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
			super::push(out, 1767);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 2035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -स्य.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	match s {
		[] => true,
		['द', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['स', '\u{1}', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', 'ह', '\u{902}', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['ि', 'स', '\u{1}', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['त', '\u{94d}', 'र', 'व', '\u{1}', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ृह्य.
pub(crate) fn n1600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1466);
	s.is_empty()
}

/// Words ending -वन्.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1168);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1719);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 1720);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				['य', '\u{1}', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['य', '\u{1}', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 1722);
			match s {
				[] => true,
				['द', '\u{1}', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 1723);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 2012);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				['ध', s @ ..] => {
					super::push(out, 2014);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 2015);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 2016);
					match s {
						[] => true,
						['थ', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 2017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -्षन्.
pub(crate) fn n1873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1725);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 2018);
			match s {
				[] => true,
				['उ', '\u{1}', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -हन्.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1170);
	match s {
		[] => true,
		['ो', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['ष', '\u{94d}', 'क', 'र', '\u{1}', s @ ..] => {
					super::push(out, 2020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -िन्.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1171);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 1727);
			s.is_empty()
		}
		['ग', 'ो', 'र', s @ ..] => {
			super::push(out, 1728);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 1730);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 1731);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 1732);
			s.is_empty()
		}
		['म', 'ा', s @ ..] => {
			match s {
				[] => true,
				['ग', 'र', s @ ..] => {
					super::push(out, 2021);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 1733);
			s.is_empty()
		}
		['ल', s @ ..] => {
			super::push(out, 1734);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 1735);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 1736);
			s.is_empty()
		}
		_ => false,
	}
}

