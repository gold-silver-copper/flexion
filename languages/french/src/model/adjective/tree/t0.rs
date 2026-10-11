use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 232);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', 'm', 'a', 'n', 'i', 'p', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 29);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 235);
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 236);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 839);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 1104);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 840);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 31);
					match s {
						[] => true,
						['\u{13}', s @ ..] => {
							super::push(out, 237);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 467);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 841);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 33);
					s.is_empty()
				}
				['m', '\u{12}', 'r', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 34);
					s.is_empty()
				}
				['n', '\u{13}', 'a', 'r', 'g', 'u', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 35);
					s.is_empty()
				}
				['p', '\u{12}', 'm', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 36);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 239);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 240);
							match s {
								[] => true,
								['a', 'g', 'n', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 469);
									s.is_empty()
								}
								['r', 'a', 't', 'é', '\u{1}', s @ ..] => {
									super::push(out, 470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{13}', 'o', 'l', 'l', 'a', 'c', 'a', 'r', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 38);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 241);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['t', 'o', 'c', 'a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 471);
									s.is_empty()
								}
								['ê', 'z', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['i', '\u{12}', 'l', 'é', 'c', '\u{1}', s @ ..] => {
					super::push(out, 41);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['\u{11}', 'é', 'r', '\u{1}', s @ ..] => {
							super::push(out, 242);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 243);
							match s {
								[] => true,
								['b', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 473);
									s.is_empty()
								}
								['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 244);
							match s {
								[] => true,
								['b', 'a', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 475);
									s.is_empty()
								}
								['i', 'n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 43);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 245);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 477);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							super::push(out, 246);
							match s {
								[] => true,
								['p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 479);
									s.is_empty()
								}
								['r', 'g', '-', 'o', 'é', 'n', '\u{1}', s @ ..] => {
									super::push(out, 480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 247);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 481);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 842);
											match s {
												[] => true,
												['e', 't', 'è', 'p', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['\u{11}', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 248);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 249);
							match s {
								[] => true,
								['l', 'b', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 482);
									s.is_empty()
								}
								['t', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 250);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 484);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 843);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 1106);
													match s {
														[] => true,
														['p', s @ ..] => super::t0::n1308(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['c', 'a', 't', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 45);
					match s {
						[] => true,
						['\u{11}', s @ ..] => super::t0::n255(s, out),
						['\u{12}', 'a', 'r', 'f', 'u', 'a', 'm', '\u{1}', s @ ..] => super::t0::n256(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n46(s, out),
				['p', '\u{11}', 'i', '\u{1}', s @ ..] => super::t0::n47(s, out),
				['q', '\u{10}', 'p', 'f', '\u{1}', s @ ..] => super::t0::n48(s, out),
				['r', s @ ..] => super::t0::n49(s, out),
				['s', '\u{13}', 'i', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t0::n50(s, out),
				['u', '\u{12}', 'd', 'a', 'c', '\u{1}', s @ ..] => super::t0::n51(s, out),
				['ï', '\u{11}', 'a', 'l', '\u{1}', s @ ..] => super::t0::n52(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['j', s @ ..] => super::t0::n10(s, out),
		['k', s @ ..] => super::t0::n11(s, out),
		['l', s @ ..] => super::t0::n12(s, out),
		['m', s @ ..] => super::t0::n13(s, out),
		['n', s @ ..] => super::t0::n14(s, out),
		['o', s @ ..] => super::t0::n15(s, out),
		['p', s @ ..] => super::t0::n16(s, out),
		['r', s @ ..] => super::t0::n17(s, out),
		['s', s @ ..] => super::t0::n18(s, out),
		['t', s @ ..] => super::t0::n19(s, out),
		['u', s @ ..] => super::t0::n20(s, out),
		['v', 'o', '\u{11}', 's', '\u{1}', s @ ..] => super::t0::n21(s, out),
		['x', s @ ..] => super::t0::n22(s, out),
		['y', s @ ..] => super::t0::n23(s, out),
		['z', s @ ..] => super::t0::n24(s, out),
		['è', 'b', '\u{13}', 'a', 'n', 'i', 'k', 'r', 'u', 'b', '\u{1}', s @ ..] => super::t0::n25(s, out),
		['é', s @ ..] => super::t0::n26(s, out),
		['ï', s @ ..] => super::t0::n27(s, out),
		_ => false,
	}
}

/// Words ending -public (3 syllables).
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	s.is_empty()
}

/// Words ending -nc (1 syllables).
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word maufranc (2 syllables).
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	s.is_empty()
}

/// Words ending -oc.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 't', 's', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 254);
			s.is_empty()
		}
		['\u{13}', 'h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ipc (1 syllables).
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	s.is_empty()
}

/// The word fpqc (0 syllables).
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -rc.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	s.is_empty()
}

/// The word antifisc (3 syllables).
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	s.is_empty()
}

/// The word caduc (2 syllables).
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	s.is_empty()
}

/// The word laïc (1 syllables).
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	s.is_empty()
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['\u{11}', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				['\u{13}', 'i', 'p', '-', 'e', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['\u{11}', 'd', 'j', '\u{1}', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['o', 'r', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['\u{10}', '\u{1}', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['o', 'b', s @ ..] => {
							super::push(out, 489);
							match s {
								[] => true,
								['e', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'o', 'r', 'g', 'r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 491);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 492);
							s.is_empty()
						}
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 263);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 494);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 495);
							match s {
								[] => true,
								['b', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 846);
									s.is_empty()
								}
								['c', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'a', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 't', 's', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 496);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 497);
							match s {
								[] => true,
								['l', 'i', 'u', s @ ..] => {
									super::push(out, 848);
									match s {
										[] => true,
										['o', 'r', s @ ..] => {
											super::push(out, 1107);
											match s {
												[] => true,
												['b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1280);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 849);
									match s {
										[] => true,
										['o', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1108);
											s.is_empty()
										}
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
		['u', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ï', '\u{12}', 'o', 'l', 'b', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 58);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{12}', 'r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				['\u{13}', 'n', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['a', 'l', '-', 'e', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 499);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 500);
							match s {
								[] => true,
								['e', 'v', 'i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 501);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 851);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1109);
											match s {
												[] => true,
												['e', 'm', 'é', '\u{1}', s @ ..] => {
													super::push(out, 1281);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ï', s @ ..] => {
							super::push(out, 502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 503);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['c', 'o', 'r', 'c', 'é', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1110);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 1111);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'ô', 'h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 852);
									s.is_empty()
								}
								['r', 't', 'é', 'm', '-', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 504);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 854);
									match s {
										[] => true,
										['r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1112);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 505);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 855);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 856);
									match s {
										[] => true,
										['r', '-', 'u', 'a', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1113);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['â', '-', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', '\u{13}', 'c', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 268);
					match s {
						[] => true,
						['o', 'd', '-', 'n', 'i', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 'a', 'r', 'g', 'o', 'n', 'r', 'o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 508);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 509);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 857);
									match s {
										[] => true,
										['n', 'i', 't', 'u', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1114);
											s.is_empty()
										}
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
		['l', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 510);
							s.is_empty()
						}
						['c', 'è', 'i', 's', '-', 'e', 'd', '-', 'n', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 511);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 512);
							match s {
								[] => true,
								['e', 'n', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 858);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 859);
									match s {
										[] => true,
										['a', 'c', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1115);
											s.is_empty()
										}
										['u', 'o', s @ ..] => {
											match s {
												[] => true,
												['c', '-', 'e', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 1282);
													s.is_empty()
												}
												['r', 't', 'i', 'c', '\u{1}', s @ ..] => {
													super::push(out, 1283);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', s @ ..] => {
							super::push(out, 513);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 861);
									match s {
										[] => true,
										['g', '-', 'e', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1116);
											s.is_empty()
										}
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
		['m', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 269);
					match s {
						[] => true,
						['è', s @ ..] => {
							super::push(out, 514);
							match s {
								[] => true,
								['r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 515);
							s.is_empty()
						}
						['è', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 863);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 864);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 1117);
											match s {
												[] => true,
												['y', s @ ..] => {
													super::push(out, 1284);
													match s {
														[] => true,
														['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t0::n1400(s, out),
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
		['n', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 516);
							match s {
								[] => true,
								['c', s @ ..] => super::t0::n891(s, out),
								['m', s @ ..] => super::t0::n892(s, out),
								_ => false,
							}
						}
						['g', 'i', 'l', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => super::t0::n540(s, out),
						['i', s @ ..] => super::t0::n541(s, out),
						['o', s @ ..] => super::t0::n542(s, out),
						['è', s @ ..] => super::t0::n543(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n67(s, out),
		['s', s @ ..] => super::t0::n68(s, out),
		['t', s @ ..] => super::t0::n69(s, out),
		['u', s @ ..] => super::t0::n70(s, out),
		['x', s @ ..] => super::t0::n71(s, out),
		['é', s @ ..] => super::t0::n72(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', '\u{11}', 'p', '\u{1}', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 280);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 549);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 898);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 1148);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 1149);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', 'w', 'a', 'h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 282);
					match s {
						[] => true,
						['k', '-', 'f', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['\u{12}', 'l', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['\u{13}', 'o', 'r', 'p', 'r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{11}', 'm', 'u', 'o', 'r', 't', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 285);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 552);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 899);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'œ', 'b', '-', 'e', 'd', '-', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['b', '\u{10}', 'c', 'b', '\u{1}', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['e', '\u{12}', 'r', 'a', 'u', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 79);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 554);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 901);
									s.is_empty()
								}
								['m', 'h', '\u{1}', s @ ..] => {
									super::push(out, 902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 288);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 555);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 556);
							match s {
								[] => true,
								['a', 'p', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 557);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 905);
									match s {
										[] => true,
										['b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1150);
											s.is_empty()
										}
										['r', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 289);
					match s {
						[] => true,
						['i', 'p', 'm', 'u', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', '\u{11}', 's', '\u{1}', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['l', '\u{11}', 'u', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		['n', '\u{12}', 'i', 'm', 't', 'e', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 86);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 559);
							match s {
								[] => true,
								['d', 'd', 'i', 'y', '\u{1}', s @ ..] => {
									super::push(out, 906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['\u{11}', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				['\u{13}', 'o', 'g', 'i', 's', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 560);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 907);
									match s {
										[] => true,
										['h', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 296);
					s.is_empty()
				}
				['\u{13}', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['\u{11}', '\u{1}', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 561);
							s.is_empty()
						}
						['r', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 92);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 563);
							s.is_empty()
						}
						['d', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', '\u{12}', 'd', 'i', 'j', 'd', '\u{1}', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 565);
							match s {
								[] => true,
								['h', 'k', '\u{1}', s @ ..] => {
									super::push(out, 908);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'i', 'k', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 566);
							s.is_empty()
						}
						['u', 'o', 'a', 'j', 'd', '\u{1}', s @ ..] => {
							super::push(out, 567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 307);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 568);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 910);
									s.is_empty()
								}
								['t', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['\u{13}', 'i', s @ ..] => {
					super::push(out, 310);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 569);
							s.is_empty()
						}
						['m', 'i', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', '\u{11}', 'c', '\u{1}', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['u', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['o', 'd', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 572);
							s.is_empty()
						}
						['s', 'a', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 574);
							match s {
								[] => true,
								['v', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['\u{11}', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['\u{13}', 'q', 'i', 'u', 'q', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['l', '\u{10}', 't', '\u{1}', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['u', '\u{11}', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 575);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 913);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 1153);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1295);
													s.is_empty()
												}
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
		['i', '\u{13}', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', 'c', 'l', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 576);
							s.is_empty()
						}
						['é', 'h', 'c', 'l', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 578);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 579);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 580);
							match s {
								[] => true,
								['o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 581);
							match s {
								[] => true,
								['i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 915);
									s.is_empty()
								}
								['m', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 582);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 583);
							match s {
								[] => true,
								['p', 'm', 'y', 'n', '\u{1}', s @ ..] => {
									super::push(out, 917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 585);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 918);
									s.is_empty()
								}
								['i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 919);
									s.is_empty()
								}
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 587);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 921);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 1154);
											s.is_empty()
										}
										['n', '\u{1}', s @ ..] => {
											super::push(out, 1155);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 588);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 590);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 924);
									match s {
										[] => true,
										['o', 'm', 'é', 'r', 'é', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1156);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 925);
									match s {
										[] => true,
										['a', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1157);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 't', s @ ..] => {
							super::push(out, 591);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 592);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 593);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 927);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1158);
											match s {
												[] => true,
												['i', 'r', 'é', 'p', '\u{1}', s @ ..] => {
													super::push(out, 1296);
													s.is_empty()
												}
												['o', 'é', 'n', '\u{1}', s @ ..] => {
													super::push(out, 1297);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 594);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 929);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1159);
											match s {
												[] => true,
												['o', 'r', 'é', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1298);
													s.is_empty()
												}
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
		['e', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['k', 'c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 595);
							s.is_empty()
						}
						['s', 'é', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 596);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 598);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 930);
									match s {
										[] => true,
										['-', 'n', 'e', '-', 'c', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1160);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 1161);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'r', 'u', 'o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 931);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 599);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 933);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1162);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 1299);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 1300);
													match s {
														[] => true,
														['r', 'e', 'p', 'i', s @ ..] => super::t0::n1412(s, out),
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
						['u', s @ ..] => super::t0::n624(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n116(s, out),
		['l', '\u{11}', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		['o', s @ ..] => super::t0::n118(s, out),
		['t', '\u{12}', 'a', 'u', 'h', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['u', s @ ..] => super::t0::n120(s, out),
		['û', '\u{11}', 'o', 's', '\u{1}', s @ ..] => super::t0::n121(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', '\u{11}', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 120);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', '\u{11}', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 334);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 620);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 621);
							match s {
								[] => true,
								['n', 'g', s @ ..] => {
									super::push(out, 942);
									match s {
										[] => true,
										['-', 'n', 'a', 'i', 'n', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1168);
											s.is_empty()
										}
										['n', 'a', 'i', 'n', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1169);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 622);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 943);
									match s {
										[] => true,
										['-', 'n', 'a', 'n', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1170);
											s.is_empty()
										}
										['n', 'a', 'n', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 623);
							match s {
								[] => true,
								['b', 'é', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 944);
									s.is_empty()
								}
								['f', 'é', 'c', '\u{1}', s @ ..] => {
									super::push(out, 945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['k', 'a', 'm', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 337);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 625);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['e', 'b', '-', 's', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 627);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 628);
							s.is_empty()
						}
						['r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 630);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 947);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1172);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 1304);
													match s {
														[] => true,
														['l', 'o', 'p', 'a', '\u{1}', s @ ..] => super::t0::n1414(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 1305);
													match s {
														[] => true,
														['k', 'c', 'a', 'j', '\u{1}', s @ ..] => super::t0::n1415(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 948);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 949);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 1173);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1306);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 1307);
													match s {
														[] => true,
														['a', s @ ..] => super::t0::n1416(s, out),
														['r', s @ ..] => super::t0::n1417(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['é', s @ ..] => {
											super::push(out, 1174);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'y', 'a', 't', 'u', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 632);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 633);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t0::n658(s, out),
						['n', s @ ..] => super::t0::n659(s, out),
						['p', s @ ..] => super::t0::n660(s, out),
						['r', s @ ..] => super::t0::n661(s, out),
						['u', s @ ..] => super::t0::n662(s, out),
						['z', 'n', 'i', 'z', '\u{1}', s @ ..] => super::t0::n663(s, out),
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t0::n355(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n128(s, out),
		['r', s @ ..] => super::t0::n129(s, out),
		['u', s @ ..] => super::t0::n130(s, out),
		['y', '\u{13}', 'd', 'o', 'n', 'a', '\u{1}', s @ ..] => super::t0::n131(s, out),
		['ï', s @ ..] => super::t0::n132(s, out),
		['û', '\u{12}', 'o', 't', 'h', 's', 'a', 'p', '\u{1}', s @ ..] => super::t0::n133(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['\u{11}', 'b', '\u{1}', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				['\u{12}', 'd', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 355);
					match s {
						[] => true,
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 672);
							s.is_empty()
						}
						['é', 'm', '\u{1}', s @ ..] => {
							super::push(out, 673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 674);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 675);
							match s {
								[] => true,
								['c', 'é', '\u{1}', s @ ..] => {
									super::push(out, 986);
									s.is_empty()
								}
								['g', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 356);
					match s {
						[] => true,
						['i', 'k', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 676);
							s.is_empty()
						}
						['y', 'r', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{12}', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 678);
							s.is_empty()
						}
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 679);
							s.is_empty()
						}
						['t', 'é', 'r', '\u{1}', s @ ..] => {
							super::push(out, 680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 681);
							s.is_empty()
						}
						['é', 't', 'é', 'h', '\u{1}', s @ ..] => {
							super::push(out, 682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['\u{13}', 'r', 'e', 'v', '-', 'o', 't', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 360);
					match s {
						[] => true,
						['h', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', 'u', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', '\u{12}', 'd', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', '\u{12}', 'r', 't', 'u', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['m', '\u{11}', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 684);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', 'b', '-', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{11}', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['u', '\u{12}', 's', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['\u{11}', 'd', '\u{1}', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['b', 'k', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 686);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['d', 'a', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 688);
							s.is_empty()
						}
						['h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						['k', 'l', 'a', 'b', '-', 'ï', 'a', 'h', 'c', 't', 'a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						['v', 'o', 's', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 366);
					match s {
						[] => true,
						['e', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 692);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 694);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 989);
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1196);
											s.is_empty()
										}
										['s', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 1197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 990);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1198);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 1315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 695);
							s.is_empty()
						}
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 696);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 697);
							match s {
								[] => true,
								['é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 991);
									s.is_empty()
								}
								['ô', 's', '\u{1}', s @ ..] => {
									super::push(out, 992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 368);
					match s {
						[] => true,
						['d', 'n', 'a', 'k', 'i', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 698);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 700);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 993);
									match s {
										[] => true,
										['u', 'c', 'æ', 's', '\u{1}', s @ ..] => {
											super::push(out, 1199);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 994);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 995);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1200);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 1316);
													match s {
														[] => true,
														['o', s @ ..] => super::t0::n1421(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 1201);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 701);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 997);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 1202);
											match s {
												[] => true,
												['x', 'u', 'o', 'd', '\u{1}', s @ ..] => {
													super::push(out, 1317);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', 'r', 't', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'k', 'i', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 370);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 704);
							match s {
								[] => true,
								['l', 'c', 'é', '\u{1}', s @ ..] => {
									super::push(out, 999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'p', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						['o', 's', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 371);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 707);
							match s {
								[] => true,
								['l', 'u', 'o', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['\u{12}', 'i', 'n', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				['\u{13}', 'l', 'o', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n156(s, out),
		['û', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 715);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 716);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', '\u{13}', 'u', 'l', 'c', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 157);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 718);
							s.is_empty()
						}
						['t', 'n', 'a', 'd', 'n', 'e', 'c', 's', 'e', 'd', 'o', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', '\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 380);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 720);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 381);
					match s {
						[] => true,
						['a', 'p', 'é', '\u{1}', s @ ..] => {
							super::push(out, 721);
							s.is_empty()
						}
						['l', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 724);
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1029);
									s.is_empty()
								}
								['é', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						['l', 'u', 'o', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 727);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1031);
									match s {
										[] => true,
										['m', 'o', 'c', '-', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['\u{11}', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				['\u{12}', 'e', 'p', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['i', 'c', 'c', 'a', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						['o', 'b', 'n', 'o', 'b', '-', 'e', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['l', 'c', '-', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 732);
							s.is_empty()
						}
						['p', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 388);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 734);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', '\u{11}', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 389);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 735);
							match s {
								[] => true,
								['i', 't', '\u{1}', s @ ..] => {
									super::push(out, 1033);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 391);
					match s {
						[] => true,
						['u', 'e', 's', 's', 'a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['\u{11}', 'a', 'l', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['a', 's', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						['e', 'r', 'p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 739);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1034);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1236);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 1353);
													match s {
														[] => true,
														['u', s @ ..] => super::t0::n1443(s, out),
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
		['t', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 740);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1035);
									match s {
										[] => true,
										['d', 'n', 'e', 'c', 's', 'e', 'd', 'o', 'r', 'f', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1237);
											s.is_empty()
										}
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
		['u', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						['o', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 743);
							s.is_empty()
						}
						['g', 'l', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 744);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 745);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 746);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1037);
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1238);
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 398);
					match s {
						[] => true,
						['b', 'i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', '\u{10}', 'x', '\u{1}', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['è', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 749);
							s.is_empty()
						}
						['r', 'p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', '\u{11}', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['j', 'u', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['n', 'e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['d', 'n', 'e', 'w', '-', 'n', 'o', 'r', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 754);
							match s {
								[] => true,
								['o', 'c', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', '\u{10}', 'g', 'l', '\u{1}', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 404);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 759);
							match s {
								[] => true,
								['s', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', 'q', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 761);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1040);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1240);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 1041);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1241);
											s.is_empty()
										}
										['m', 'o', s @ ..] => {
											super::push(out, 1242);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 1354);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 762);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1042);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 763);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1044);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1243);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', '-', 'd', 'r', 'u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1045);
									s.is_empty()
								}
								['q', s @ ..] => {
									super::push(out, 1046);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 1244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['j', 'e', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 765);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1048);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'e', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1049);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1050);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1051);
									s.is_empty()
								}
								['p', 'm', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 766);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1053);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1246);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 1355);
													match s {
														[] => true,
														['-', s @ ..] => super::t0::n1444(s, out),
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
		['f', '\u{11}', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['h', '\u{11}', 'g', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['u', 'r', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 408);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 768);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 769);
							match s {
								[] => true,
								['o', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t0::n427(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['\u{11}', s @ ..] => super::t0::n428(s, out),
				['\u{12}', s @ ..] => super::t0::n429(s, out),
				['\u{13}', s @ ..] => super::t0::n430(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n182(s, out),
		['s', s @ ..] => super::t0::n183(s, out),
		['t', s @ ..] => super::t0::n184(s, out),
		['u', s @ ..] => super::t0::n185(s, out),
		['â', '\u{13}', 'm', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => super::t0::n186(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['\u{11}', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 418);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 419);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 783);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1068);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1069);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1258);
											s.is_empty()
										}
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1259);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'u', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 784);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1071);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 1260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'd', 'n', 'a', 'v', 'r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 785);
							s.is_empty()
						}
						['m', 'i', 'u', 'q', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 422);
					match s {
						[] => true,
						['s', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['\u{12}', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 426);
					match s {
						[] => true,
						['j', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 788);
							s.is_empty()
						}
						['p', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 789);
							s.is_empty()
						}
						['r', 'b', 'é', 'h', '\u{1}', s @ ..] => {
							super::push(out, 790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 791);
							match s {
								[] => true,
								['-', 'u', 'a', '-', 't', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1072);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 792);
							s.is_empty()
						}
						['n', 'g', 'u', 'e', 'n', 'g', 'o', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 428);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 429);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 794);
							s.is_empty()
						}
						['u', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['g', 'n', 'o', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 434);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 797);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 798);
							s.is_empty()
						}
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 799);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 435);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 801);
							s.is_empty()
						}
						['f', 'u', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 802);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 803);
							match s {
								[] => true,
								['c', 't', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 804);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1075);
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 805);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1076);
									match s {
										[] => true,
										['g', 'i', 't', '\u{1}', s @ ..] => {
											super::push(out, 1262);
											s.is_empty()
										}
										['u', 'g', 'i', 't', '\u{1}', s @ ..] => {
											super::push(out, 1263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'a', 'd', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['\u{12}', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 439);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['t', 'e', 'k', 'k', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['y', '\u{12}', 'u', 'k', 'i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['ç', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sov (1 syllables).
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	s.is_empty()
}

/// Words ending -x.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['d', 'n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						['l', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						['m', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 810);
							s.is_empty()
						}
						['r', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['f', 'é', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 812);
							s.is_empty()
						}
						['o', 'h', 'c', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', '\u{12}', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 815);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1077);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 1264);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 1078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 816);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1079);
									s.is_empty()
								}
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1080);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 1081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 447);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 817);
							match s {
								[] => true,
								['e', 'd', 'r', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1082);
									s.is_empty()
								}
								['h', 'c', 'é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['a', 'f', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 819);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 820);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1084);
									match s {
										[] => true,
										['i', 'm', s @ ..] => {
											super::push(out, 1265);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 1362);
													match s {
														[] => true,
														['e', s @ ..] => super::t0::n1449(s, out),
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
						['o', s @ ..] => {
							super::push(out, 821);
							match s {
								[] => true,
								['d', '-', 'e', 'r', 'g', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1085);
									s.is_empty()
								}
								['p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1086);
									s.is_empty()
								}
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

/// Words ending -y.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', '\u{12}', 'l', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['b', '\u{12}', 'â', 'b', '\u{1}', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 822);
							match s {
								[] => true,
								['o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['\u{13}', 'd', 'n', 'e', 'i', 'r', 'f', '-', 'y', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', '\u{12}', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['\u{11}', 'p', '\u{1}', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', '\u{12}', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['e', '\u{11}', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['i', '\u{12}', 'h', 'g', 'r', 'i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word burkinabè (3 syllables).
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	s.is_empty()
}

/// Words ending -é.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['\u{12}', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['\u{12}', 'é', 'v', '\u{1}', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 824);
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1088);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1089);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 1266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 825);
							s.is_empty()
						}
						['s', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 827);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1090);
									match s {
										[] => true,
										['b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1267);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'i', s @ ..] => {
							super::push(out, 828);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1091);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 1268);
											match s {
												[] => true,
												['i', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 1363);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'b', 'r', 'a', 'c', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['\u{13}', 'é', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 830);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1093);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 1269);
											match s {
												[] => true,
												['m', 'i', '\u{1}', s @ ..] => {
													super::push(out, 1364);
													s.is_empty()
												}
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
		['s', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 831);
							match s {
								[] => true,
								['i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 832);
							match s {
								[] => true,
								['a', 'p', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 833);
							match s {
								[] => true,
								['i', 'r', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 835);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1097);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1270);
											match s {
												[] => true,
												['d', 'n', 'e', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1365);
													s.is_empty()
												}
												['l', 'b', 'u', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1366);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1271);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['u', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1272);
											s.is_empty()
										}
										['é', s @ ..] => {
											super::push(out, 1273);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1098);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1274);
											match s {
												[] => true,
												['n', 'i', 'm', 'i', 'r', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1367);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 1275);
											match s {
												[] => true,
												['b', 'a', 'v', 'r', 'e', 's', 'b', 'o', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 1368);
													s.is_empty()
												}
												['r', 'é', 't', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1369);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1099);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 836);
							match s {
								[] => true,
								['o', 'm', s @ ..] => {
									super::push(out, 1101);
									match s {
										[] => true,
										['-', 't', 'e', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1276);
											s.is_empty()
										}
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
		['u', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['\u{13}', s @ ..] => super::t0::n484(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ï.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word antisystème (3 syllables).
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	s.is_empty()
}

/// Words ending -cane (3 syllables).
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['i', 't', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mane (3 syllables).
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 866);
	s.is_empty()
}

/// The word hors-ligne (3 syllables).
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	s.is_empty()
}

/// Words ending -ine (3 syllables).
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['a', 'm', '-', 'e', 'u', 'g', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -one (3 syllables).
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	s.is_empty()
}

/// Words ending -ène (3 syllables).
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	s.is_empty()
}

/// Words ending -re.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['f', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['r', 'u', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['î', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['p', 'i', 'm', s @ ..] => {
							super::push(out, 870);
							match s {
								[] => true,
								['m', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 871);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1121);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 1285);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1372);
													match s {
														[] => true,
														['c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t0::n1450(s, out),
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
						['o', s @ ..] => {
							super::push(out, 872);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1286);
											s.is_empty()
										}
										['é', s @ ..] => {
											super::push(out, 1287);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 1373);
													match s {
														[] => true,
														['c', 'é', s @ ..] => super::t0::n1451(s, out),
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
						['r', '-', 's', 'n', 'a', 's', '-', 'e', 'c', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 874);
							match s {
								[] => true,
								['-', 'f', 'f', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 875);
							match s {
								[] => true,
								['t', '-', 'à', '-', 'e', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 1123);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1124);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 1288);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 1374);
													s.is_empty()
												}
												['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1375);
													s.is_empty()
												}
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
				['v', 'i', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 876);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 878);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 879);
							match s {
								[] => true,
								['c', 'a', 'r', 'a', 'c', '-', 'i', 'm', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1126);
									s.is_empty()
								}
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

/// Words ending -se.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 881);
							match s {
								[] => true,
								['b', 'm', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1127);
									s.is_empty()
								}
								['d', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1128);
									s.is_empty()
								}
								['u', 'q', 'r', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 1129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 883);
							match s {
								[] => true,
								['d', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['a', 'h', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1131);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1132);
									s.is_empty()
								}
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

/// Words ending -te.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['\u{12}', 't', 'i', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1133);
									match s {
										[] => true,
										['o', 'm', s @ ..] => {
											super::push(out, 1289);
											match s {
												[] => true,
												['é', 'd', '-', s @ ..] => {
													super::push(out, 1376);
													match s {
														[] => true,
														['l', s @ ..] => super::t0::n1452(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'p', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 887);
							match s {
								[] => true,
								['d', 'n', 'e', 'c', 's', 'e', 'd', 'o', 'r', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 888);
							match s {
								[] => true,
								['m', '-', 'e', 'l', 'l', 'i', 'u', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1136);
									s.is_empty()
								}
								['p', '-', 'à', '-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							super::push(out, 889);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1138);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1290);
											match s {
												[] => true,
												['é', s @ ..] => {
													super::push(out, 1377);
													match s {
														[] => true,
														['l', '-', 'e', 't', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => super::t0::n1453(s, out),
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
				['t', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 890);
							match s {
								[] => true,
								['m', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 1139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['e', 'm', 'é', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', 't', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				['ê', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['t', '-', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['q', 'a', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 894);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1140);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1141);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1291);
											match s {
												[] => true,
												['-', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1378);
													s.is_empty()
												}
												['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1379);
													s.is_empty()
												}
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
				['q', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 895);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1142);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 1292);
											match s {
												[] => true,
												['a', 't', 'u', 'p', '\u{1}', s @ ..] => {
													super::push(out, 1380);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 1293);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 1381);
													match s {
														[] => true,
														['t', s @ ..] => super::t0::n1454(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1143);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1294);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 1382);
													match s {
														[] => true,
														['p', 'r', 'o', 'm', '\u{1}', s @ ..] => super::t0::n1455(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1144);
									s.is_empty()
								}
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

/// Words ending -xe.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['d', 'o', 'h', s @ ..] => {
							super::push(out, 896);
							match s {
								[] => true,
								['t', 'r', 'o', '-', 'c', 'e', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1145);
									s.is_empty()
								}
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

/// Words ending -ée.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['\u{13}', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 897);
							match s {
								[] => true,
								['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1146);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1147);
									s.is_empty()
								}
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

/// Words ending -ipersonnel (3 syllables).
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1383);
	match s {
		[] => true,
		['t', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uel (3 syllables).
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['t', 'c', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['m', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['t', 'n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['a', 'r', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 935);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1164);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 1302);
											match s {
												[] => true,
												['n', 'o', 'n', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['o', 'p', '-', 'e', 's', 'r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 936);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1165);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1303);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['\u{11}', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['b', 'z', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['s', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 937);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 938);
							match s {
								[] => true,
								['a', 'b', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word soûl (1 syllables).
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	s.is_empty()
}

/// The word apollonien (3 syllables).
pub(crate) fn n1414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1385);
	s.is_empty()
}

/// The word jacksonien (3 syllables).
pub(crate) fn n1415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1386);
	s.is_empty()
}

/// Words ending -aussien (3 syllables).
pub(crate) fn n1416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	s.is_empty()
}

/// Words ending -russien (3 syllables).
pub(crate) fn n1417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['t', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -min (2 syllables).
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nin (2 syllables).
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['é', 'b', '\u{1}', s @ ..] => {
			super::push(out, 953);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pin (2 syllables).
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rin (2 syllables).
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uin (2 syllables).
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 638);
	s.is_empty()
}

/// The word zinzin (2 syllables).
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	s.is_empty()
}

/// Words ending -in (3 syllables).
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['i', 'u', 'q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1178);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1308);
									match s {
										[] => true,
										['t', '-', 't', 'u', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 1389);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1179);
							match s {
								[] => true,
								['v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['n', 'e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 644);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['-', 'e', 'd', '-', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['m', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 965);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 967);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1182);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1183);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['c', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['a', 'l', 'é', 'c', '\u{1}', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['é', 'h', 'p', 'o', 'r', 't', 's', 'u', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 970);
							match s {
								[] => true,
								['i', 'p', '-', 'e', 'd', '-', 'e', 'g', 'r', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 971);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['d', 'o', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 972);
							s.is_empty()
						}
						['s', 'n', 'e', 't', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 974);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1187);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 1310);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 1390);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', 'e', 't', 'n', 'o', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1311);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'n', 'o', 'f', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1188);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1189);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 1312);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 975);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 976);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['i', 'o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 980);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1192);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 1313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 981);
							match s {
								[] => true,
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1193);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 1194);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1314);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 1391);
													match s {
														[] => true,
														['a', s @ ..] => super::t0::n1458(s, out),
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
				['x', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rn.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['\u{10}', 'b', 'c', '\u{1}', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		['\u{13}', 'e', 't', 'n', 'i', 'm', 'o', 'k', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 984);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1195);
									s.is_empty()
								}
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

/// The word anodyn (3 syllables).
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	s.is_empty()
}

/// Words ending -ïn.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pashtoûn (2 syllables).
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	s.is_empty()
}

/// Words ending -ourrier (3 syllables).
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1001);
							s.is_empty()
						}
						['u', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 1002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['c', 's', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1003);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1004);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1203);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 1318);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1005);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1204);
									match s {
										[] => true,
										['é', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1006);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1007);
							match s {
								[] => true,
								['l', 'i', 'e', s @ ..] => {
									super::push(out, 1205);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 1320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1010);
							match s {
								[] => true,
								['c', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1206);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['g', 'ï', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1012);
							s.is_empty()
						}
						['h', 'g', 'ï', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						['m', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['t', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['e', s @ ..] => super::t0::n738(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usiness (3 syllables).
pub(crate) fn n1443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1413);
	s.is_empty()
}

/// Words ending --secret (3 syllables).
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1414);
	s.is_empty()
}

/// Words ending -nt (3 syllables).
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1247);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1356);
									match s {
										[] => true,
										['m', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1415);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1248);
							match s {
								[] => true,
								['b', 'r', 'a', 'c', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1358);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 1416);
											match s {
												[] => true,
												['i', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 1439);
													s.is_empty()
												}
												['t', 'i', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 1440);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1251);
							match s {
								[] => true,
								['i', 'u', s @ ..] => {
									super::push(out, 1359);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 1417);
											match s {
												[] => true,
												['-', 't', 'u', 'o', 't', '\u{1}', s @ ..] => {
													super::push(out, 1441);
													s.is_empty()
												}
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
				['t', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1252);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1360);
									match s {
										[] => true,
										['é', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1418);
											s.is_empty()
										}
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
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['a', 'v', 'i', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['i', 'h', 'c', 'n', 'a', 'l', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ot (1 syllables).
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['h', '\u{1}', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ot (2 syllables).
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		['j', 'r', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 775);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['â', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['g', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['u', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ot (3 syllables).
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['d', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						['r', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['\u{13}', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['p', 's', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['\u{11}', 'e', '\u{1}', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['\u{13}', 'u', 'r', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tt.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['\u{13}', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['t', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word monomât (3 syllables).
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	s.is_empty()
}

/// Words ending -ermineux (3 syllables).
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1419);
	match s {
		[] => true,
		['v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ué (3 syllables).
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['q', 'i', 't', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['s', 'i', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1277);
							match s {
								[] => true,
								['o', 'l', 'h', 'p', 'é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['i', 't', 's', 'b', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['u', 's', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word anticalcaire (3 syllables).
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1420);
	s.is_empty()
}

/// Words ending -écrétoire (3 syllables).
pub(crate) fn n1451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1421);
	match s {
		[] => true,
		['s', '\u{1}', s @ ..] => {
			super::push(out, 1443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l-démocrate (3 syllables).
pub(crate) fn n1452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1422);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1444);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word marxiste-léniniste (3 syllables).
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1423);
	s.is_empty()
}

/// Words ending -tholique (3 syllables).
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1424);
	match s {
		[] => true,
		['a', 'c', '-', 'c', 'e', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1445);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word morphinique (3 syllables).
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1425);
	s.is_empty()
}

/// Words ending -aroxyton (3 syllables).
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1428);
	s.is_empty()
}

/// Words ending -eur (3 syllables).
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	match s {
		[] => true,
		['c', 'n', 'a', 'n', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['n', 'a', 'h', 'c', 'é', '\u{1}', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1019);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 1321);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1324);
							match s {
								[] => true,
								['d', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'e', 's', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['a', 'r', 't', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['o', 'c', 's', 'e', 'l', 'é', 't', '\u{1}', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['p', 'i', 'r', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1329);
							match s {
								[] => true,
								['c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1398);
									s.is_empty()
								}
								['h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1330);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1400);
									match s {
										[] => true,
										['p', 'é', 'd', 'o', 'n', 'u', 'm', 'm', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1332);
							match s {
								[] => true,
								['a', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1333);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1334);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1402);
									match s {
										[] => true,
										['a', 'r', 'r', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1431);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1335);
							match s {
								[] => true,
								['u', 'g', s @ ..] => {
									super::push(out, 1403);
									match s {
										[] => true,
										['é', 'r', 'é', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1432);
											s.is_empty()
										}
										_ => false,
									}
								}
								['é', s @ ..] => {
									super::push(out, 1404);
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1433);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1337);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1338);
							match s {
								[] => true,
								['é', 'd', s @ ..] => {
									super::push(out, 1406);
									match s {
										[] => true,
										['é', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1434);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1341);
							match s {
								[] => true,
								['é', 'r', 'i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 1407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['a', 'h', 'c', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1344);
							match s {
								[] => true,
								['m', 'u', 'g', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1408);
									s.is_empty()
								}
								['p', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['c', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1346);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1410);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 1435);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n1259(s, out),
				['t', s @ ..] => super::t0::n1260(s, out),
				['u', s @ ..] => super::t0::n1261(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n1053(s, out),
		_ => false,
	}
}

/// Words ending -rteur (3 syllables).
pub(crate) fn n1259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1230);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['o', s @ ..] => {
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

/// Words ending -tteur (3 syllables).
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uteur (3 syllables).
pub(crate) fn n1261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1350);
			s.is_empty()
		}
		['h', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ueur (3 syllables).
pub(crate) fn n1053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1027);
	match s {
		[] => true,
		['o', 'j', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

