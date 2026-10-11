use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 141);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 142);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 417);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 927);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 928);
													match s {
														[] => true,
														['q', 's', 'u', s @ ..] => super::t0::n3884(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 418);
											s.is_empty()
										}
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
			super::push(out, 3);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n5(s, out),
		['s', s @ ..] => super::t0::n6(s, out),
		['t', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n26(s, out),
				['e', s @ ..] => {
					super::push(out, 27);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 127);
							match s {
								[] => true,
								['p', s @ ..] => super::t0::n484(s, out),
								['x', s @ ..] => {
									super::push(out, 407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 128);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 408);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n28(s, out),
				['n', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				['o', s @ ..] => super::t0::n30(s, out),
				['u', s @ ..] => super::t0::n31(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usquisque.
pub(crate) fn n3884(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2173);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2174);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143);
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 144);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 145);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 146);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 419);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 420);
									s.is_empty()
								}
								['l', s @ ..] => super::t0::n505(s, out),
								_ => false,
							}
						}
						['b', s @ ..] => super::t0::n156(s, out),
						['e', s @ ..] => super::t0::n157(s, out),
						['i', s @ ..] => super::t0::n158(s, out),
						['m', s @ ..] => super::t0::n159(s, out),
						['o', s @ ..] => super::t0::n160(s, out),
						['u', s @ ..] => super::t0::n161(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n37(s, out),
				['f', s @ ..] => super::t0::n38(s, out),
				['g', s @ ..] => super::t0::n39(s, out),
				['h', s @ ..] => super::t0::n40(s, out),
				['l', s @ ..] => super::t0::n41(s, out),
				['n', s @ ..] => super::t0::n42(s, out),
				['p', s @ ..] => super::t0::n43(s, out),
				['s', s @ ..] => super::t0::n44(s, out),
				['t', s @ ..] => super::t0::n45(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n15(s, out),
		['u', s @ ..] => super::t0::n16(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 483);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 183);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 184);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 485);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 486);
									match s {
										[] => true,
										['p', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 185);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 187);
							match s {
								[] => true,
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', s @ ..] => {
							super::push(out, 188);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 488);
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
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 18);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 189);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 190);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 970);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 1597);
													s.is_empty()
												}
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
				['d', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 191);
							match s {
								[] => true,
								['o', s @ ..] => super::t0::n605(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 489);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 490);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 60);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 193);
							s.is_empty()
						}
						['e', s @ ..] => super::t0::n212(s, out),
						['p', s @ ..] => super::t0::n213(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n63(s, out),
				['p', s @ ..] => super::t0::n64(s, out),
				['r', s @ ..] => super::t0::n65(s, out),
				['s', s @ ..] => super::t0::n66(s, out),
				['t', s @ ..] => super::t0::n67(s, out),
				['u', s @ ..] => super::t0::n68(s, out),
				['v', s @ ..] => super::t0::n69(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n20(s, out),
		['n', s @ ..] => super::t0::n21(s, out),
		['o', s @ ..] => super::t0::n22(s, out),
		['p', s @ ..] => super::t0::n23(s, out),
		['r', s @ ..] => super::t0::n24(s, out),
		['u', s @ ..] => super::t0::n25(s, out),
		_ => false,
	}
}

/// Words ending -ax.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1563);
									match s {
										[] => true,
										['l', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 910);
							match s {
								[] => true,
								['f', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 912);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['g', 'u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['t', 'r', s @ ..] => {
							super::push(out, 914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1564);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 916);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 124);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['q', 'o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 918);
							s.is_empty()
						}
						['e', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plex.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['t', 'l', 'u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['c', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2606);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2607);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 't', 'c', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ix.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1569);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1570);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1571);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1572);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1573);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1574);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 923);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								['u', 'c', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 925);
							match s {
								[] => true,
								['y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
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

/// Words ending -ox.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 135);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 137);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 926);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -ux.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 139);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -laber.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	s.is_empty()
}

/// Words ending -bber.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	s.is_empty()
}

/// Words ending -eber.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iber.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mber.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['t', 'p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['v', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ober.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['t', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uber.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cer.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 433);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['l', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fer.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 936);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'n', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 937);
							s.is_empty()
						}
						['a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 938);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1579);
									match s {
										[] => true,
										['e', 's', '\u{1}', s @ ..] => {
											super::push(out, 2175);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 939);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1580);
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 1581);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 940);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
				['p', s @ ..] => {
					super::push(out, 438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 941);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 942);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 439);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -her.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['c', 'l', 'u', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 442);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ler.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['e', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 443);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 444);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 943);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 944);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['e', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 447);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	s.is_empty()
}

/// Words ending -ter.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 948);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 168);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 453);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 44);
			s.is_empty()
		}
		['i', s @ ..] => super::t0::n47(s, out),
		['l', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 480);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'g', 'i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 967);
									match s {
										[] => true,
										['r', 'e', 'v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2182);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 2183);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t0::n14106(s, out),
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
							super::push(out, 481);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 968);
									s.is_empty()
								}
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
			super::push(out, 47);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 180);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 969);
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 181);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 48);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 49);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 51);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 52);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oides.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'r', 'y', 'h', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'y', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'y', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eles.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ples.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	s.is_empty()
}

/// Words ending -nes.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	s.is_empty()
}

/// Words ending -pes.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 198);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -res.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ses.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	s.is_empty()
}

/// Words ending -tes.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['s', 'r', 'e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['t', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ues.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// Words ending -ves.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['i', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['b', 'm', 'u', 'l', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 202);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 203);
					match s {
						[] => true,
						['u', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 69);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 204);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 205);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 975);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 976);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['g', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', 'r', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 504);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'u', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n74(s, out),
		['m', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['a', 'f', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 210);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 524);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 211);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 212);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 527);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1646);
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1014);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1647);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 1648);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1015);
									match s {
										[] => true,
										['b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', 'i', 'f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['b', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 't', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 213);
					match s {
						[] => true,
						['l', 'p', 'm', s @ ..] => {
							super::push(out, 528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n76(s, out),
		['p', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 222);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 534);
							s.is_empty()
						}
						['l', 'o', 'r', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => super::t0::n1708(s, out),
								['i', s @ ..] => super::t0::n1709(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n675(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n254(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n78(s, out),
		['s', s @ ..] => super::t0::n79(s, out),
		['t', s @ ..] => super::t0::n80(s, out),
		['u', s @ ..] => super::t0::n81(s, out),
		['v', s @ ..] => super::t0::n82(s, out),
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n83(s, out),
		['e', s @ ..] => super::t0::n84(s, out),
		['o', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1153);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['b', 'a', 'l', 'l', 'y', 's', 'a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'o', 'c', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 633);
							s.is_empty()
						}
						['s', 's', 'o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 634);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1155);
									s.is_empty()
								}
								['s', 'u', 'o', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1156);
									s.is_empty()
								}
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
			super::push(out, 84);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['y', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 635);
							match s {
								[] => true,
								['i', 'l', 'l', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 87);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'u', 'd', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 89);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 277);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 278);
					s.is_empty()
				}
				['r', 'a', 'c', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 280);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 281);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				['p', 'm', 'a', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 286);
					s.is_empty()
				}
				['s', 'u', 'a', 'c', 'o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ps.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1158);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1159);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1160);
									match s {
										[] => true,
										['a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 1764);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1765);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1766);
											match s {
												[] => true,
												['a', 'p', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n15086(s, out),
														['m', 'o', 'c', '\u{1}', s @ ..] => super::t0::n15087(s, out),
														['n', 'o', 'c', '\u{1}', s @ ..] => super::t0::n15088(s, out),
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
							super::push(out, 639);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1161);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1162);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1767);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1768);
											s.is_empty()
										}
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
			super::push(out, 95);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 288);
					s.is_empty()
				}
				['l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['r', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 640);
							s.is_empty()
						}
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 642);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 644);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 645);
							match s {
								[] => true,
								['r', 'e', 's', 'i', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1769);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 1770);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 647);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 648);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1163);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1164);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1165);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 297);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1167);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1168);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1169);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n104(s, out),
		['d', s @ ..] => super::t0::n105(s, out),
		['e', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['p', 'o', 'r', 'u', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => super::t0::n2796(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t0::n1043(s, out),
						_ => false,
					}
				}
				['b', s @ ..] => super::t0::n366(s, out),
				['c', s @ ..] => super::t0::n367(s, out),
				['d', 'i', 'o', s @ ..] => super::t0::n368(s, out),
				['g', 'n', 'y', 'r', 'a', 'h', 'p', s @ ..] => super::t0::n369(s, out),
				['n', s @ ..] => super::t0::n370(s, out),
				['r', s @ ..] => super::t0::n371(s, out),
				['s', s @ ..] => super::t0::n372(s, out),
				['t', s @ ..] => super::t0::n373(s, out),
				_ => false,
			}
		}
		['f', 'u', 'r', s @ ..] => super::t0::n107(s, out),
		['g', s @ ..] => super::t0::n108(s, out),
		['h', s @ ..] => super::t0::n109(s, out),
		['i', s @ ..] => super::t0::n110(s, out),
		['l', s @ ..] => super::t0::n111(s, out),
		['m', s @ ..] => super::t0::n112(s, out),
		['n', s @ ..] => super::t0::n113(s, out),
		['p', s @ ..] => super::t0::n114(s, out),
		['r', s @ ..] => super::t0::n115(s, out),
		['s', s @ ..] => super::t0::n116(s, out),
		['t', s @ ..] => super::t0::n117(s, out),
		['u', s @ ..] => super::t0::n118(s, out),
		['v', s @ ..] => super::t0::n119(s, out),
		['x', s @ ..] => super::t0::n120(s, out),
		_ => false,
	}
}

/// Words ending -ior.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['b', 'o', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 460);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['l', 'e', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1582);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1583);
									s.is_empty()
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 954);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1584);
									s.is_empty()
								}
								['q', s @ ..] => {
									super::push(out, 1585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', 'u', 'q', 'e', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'd', 'i', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'l', 'o', 't', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', s @ ..] => {
													match s {
														[] => true,
														['a', 'r', 'p', '\u{1}', s @ ..] => super::t0::n19261(s, out),
														['t', 'e', 'd', '\u{1}', s @ ..] => super::t0::n19262(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'r', 'i', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', 'u', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'l', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n190(s, out),
		['s', s @ ..] => super::t0::n191(s, out),
		['t', s @ ..] => super::t1::n192(s, out),
		['u', s @ ..] => super::t1::n193(s, out),
		['v', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', s @ ..] => super::t1::n591(s, out),
				['n', s @ ..] => super::t1::n592(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word diversicolor.
pub(crate) fn n14106(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lis.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['i', 'p', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'e', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t1::n8497(s, out),
										['h', '\u{1}', s @ ..] => super::t1::n8498(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'v', s @ ..] => super::t1::n1614(s, out),
						_ => false,
					}
				}
				['d', 'e', 'p', s @ ..] => super::t1::n630(s, out),
				['e', s @ ..] => super::t1::n631(s, out),
				['g', s @ ..] => super::t1::n632(s, out),
				['i', s @ ..] => super::t1::n633(s, out),
				['m', s @ ..] => super::t1::n634(s, out),
				['n', s @ ..] => super::t1::n635(s, out),
				['p', 'o', 'c', 's', 'i', 'p', 'e', s @ ..] => super::t1::n636(s, out),
				['r', s @ ..] => super::t1::n637(s, out),
				['t', s @ ..] => super::t1::n638(s, out),
				['u', s @ ..] => super::t1::n639(s, out),
				['v', 'r', 'e', 'n', s @ ..] => super::t1::n640(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n236(s, out),
		['i', s @ ..] => super::t1::n237(s, out),
		['l', s @ ..] => super::t1::n238(s, out),
		['u', 'd', 'e', s @ ..] => super::t1::n239(s, out),
		_ => false,
	}
}

/// Words ending -nis.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 219);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1019);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'i', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 220);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['g', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'n', 'e', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 221);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1020);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1651);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word acrolepis.
pub(crate) fn n1708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	s.is_empty()
}

/// Words ending -icrolepis.
pub(crate) fn n1709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -repis.
pub(crate) fn n675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	s.is_empty()
}

/// Words ending -rpis.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['u', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'u', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1653);
									match s {
										[] => true,
										['a', 'f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2635);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 2636);
													match s {
														[] => true,
														['e', 'p', '\u{1}', s @ ..] => super::t1::n19379(s, out),
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
				['l', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t1::n1716(s, out),
						['l', s @ ..] => super::t1::n1717(s, out),
						['o', s @ ..] => super::t1::n1718(s, out),
						['u', s @ ..] => super::t1::n1719(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n681(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['u', s @ ..] => super::t1::n682(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n257(s, out),
		['o', s @ ..] => super::t1::n258(s, out),
		['t', s @ ..] => super::t1::n259(s, out),
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['d', 'l', 'e', 'f', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'r', 'u', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1029);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1657);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 2228);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['c', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1030);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', 'o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', 'g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', 'a', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'c', 'e', 'i', 'a', 'r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1031);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1658);
									match s {
										[] => true,
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2229);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'a', 'l', 'a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -tis.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['m', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1033);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['l', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 235);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1034);
							match s {
								[] => true,
								['e', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1661);
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uis.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1037);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1040);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -vis.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1662);
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1663);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1664);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 240);
			match s {
				[] => true,
				['r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1041);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 241);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1042);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1043);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1666);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'v', 'i', 't', 'u', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['n', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1045);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ans.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['a', 'm', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', 'd', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 551);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n278(s, out),
		['d', s @ ..] => super::t1::n279(s, out),
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'l', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'l', s @ ..] => super::t1::n729(s, out),
				['r', 'c', s @ ..] => super::t1::n730(s, out),
				['u', 'q', 'a', 'l', s @ ..] => super::t1::n731(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t1::n281(s, out),
		['g', s @ ..] => super::t1::n282(s, out),
		['h', 'c', 'c', 'a', 'b', s @ ..] => super::t1::n283(s, out),
		['i', s @ ..] => super::t1::n284(s, out),
		['l', s @ ..] => super::t1::n285(s, out),
		['m', s @ ..] => super::t1::n286(s, out),
		['n', s @ ..] => super::t1::n287(s, out),
		['o', 'b', s @ ..] => super::t1::n288(s, out),
		['p', s @ ..] => super::t1::n289(s, out),
		['r', s @ ..] => super::t1::n290(s, out),
		['s', s @ ..] => super::t1::n291(s, out),
		['t', s @ ..] => super::t1::n292(s, out),
		['u', s @ ..] => super::t1::n293(s, out),
		['v', s @ ..] => super::t1::n294(s, out),
		['x', s @ ..] => super::t1::n295(s, out),
		['z', 'i', 't', 'a', 'b', 'b', 'a', s @ ..] => super::t1::n296(s, out),
		_ => false,
	}
}

/// Words ending -ens.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 570);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1067);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1068);
							match s {
								[] => true,
								['c', 's', s @ ..] => super::t1::n5144(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => super::t1::n2195(s, out),
						['e', 'r', '\u{1}', s @ ..] => super::t1::n2196(s, out),
						['p', 'a', '\u{1}', s @ ..] => super::t1::n2197(s, out),
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t1::n2198(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t1::n2199(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n838(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n299(s, out),
		['d', s @ ..] => super::t1::n300(s, out),
		['g', s @ ..] => super::t1::n301(s, out),
		['h', s @ ..] => super::t1::n302(s, out),
		['i', s @ ..] => super::t1::n303(s, out),
		['l', s @ ..] => super::t1::n304(s, out),
		['m', s @ ..] => super::t1::n305(s, out),
		['n', s @ ..] => super::t1::n306(s, out),
		['o', 'c', '\u{1}', s @ ..] => super::t1::n307(s, out),
		['p', s @ ..] => super::t1::n308(s, out),
		['r', s @ ..] => super::t1::n309(s, out),
		['s', s @ ..] => super::t1::n310(s, out),
		['t', s @ ..] => super::t1::n311(s, out),
		['u', s @ ..] => super::t1::n312(s, out),
		['v', s @ ..] => super::t1::n313(s, out),
		['x', 'e', 't', s @ ..] => super::t1::n314(s, out),
		_ => false,
	}
}

/// The word particeps.
pub(crate) fn n15086(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word comparticeps.
pub(crate) fn n15087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	s.is_empty()
}

/// The word conparticeps.
pub(crate) fn n15088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	s.is_empty()
}

/// Words ending -cus.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'y', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'a', 'r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'e', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n353(s, out),
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['c', 'o', 't', 'p', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1191);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'u', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'o', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1789);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1790);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -dus.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['r', 'g', 'i', 's', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1194);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', s @ ..] => {
							super::push(out, 1196);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2325);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 2326);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
				['c', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								['c', 'l', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1792);
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 's', '\u{1}', s @ ..] => super::t1::n6110(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t1::n2702(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t1::n1020(s, out),
				['f', s @ ..] => super::t1::n1021(s, out),
				['g', s @ ..] => super::t1::n1022(s, out),
				['l', s @ ..] => super::t1::n1023(s, out),
				['m', s @ ..] => super::t1::n1024(s, out),
				['p', s @ ..] => super::t1::n1025(s, out),
				['r', s @ ..] => super::t1::n1026(s, out),
				['t', s @ ..] => super::t1::n1027(s, out),
				['u', s @ ..] => super::t1::n1028(s, out),
				['v', s @ ..] => super::t1::n1029(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n361(s, out),
		['o', s @ ..] => super::t1::n362(s, out),
		['r', s @ ..] => super::t1::n363(s, out),
		['u', s @ ..] => super::t1::n364(s, out),
		_ => false,
	}
}

/// The word hebraeus.
pub(crate) fn n2796(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -taeus.
pub(crate) fn n1043(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', 'h', 'p', 'u', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1244);
			match s {
				[] => true,
				['o', 'h', 't', 'e', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -beus.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ceus.
pub(crate) fn n367(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'b', 'm', 'e', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'l', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oideus.
pub(crate) fn n368(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'e', 't', 'y', 'r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'l', 'e', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'y', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pharyngeus.
pub(crate) fn n369(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'l', 'y', 't', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -neus.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['a', 'r', 'o', 'p', 'm', 'e', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', 'l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'u', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'g', 'n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', 't', 'l', 'u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reus.
pub(crate) fn n371(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'r', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'r', 'u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -seus.
pub(crate) fn n372(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -teus.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['n', 'o', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rufus.
pub(crate) fn n107(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gus.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['n', 'u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'l', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'o', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['u', 'b', 's', 'b', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hus.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['t', 'n', 'a', 'r', 'c', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ius.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['b', 'u', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'n', 'i', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'd', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'm', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 321);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['a', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1826);
									s.is_empty()
								}
								['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 't', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'c', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1827);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['o', 'f', 'i', 'g', 'n', 'o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n392(s, out),
		['t', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['r', 'e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', 'o', 'n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lus.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['o', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['r', 'o', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['b', 'u', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1267);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2367);
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 2368);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'b', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1833);
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['c', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['b', 'i', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1834);
									match s {
										[] => true,
										['i', 'r', s @ ..] => super::t2::n11413(s, out),
										_ => false,
									}
								}
								['p', 'r', 'u', 't', s @ ..] => super::t2::n6514(s, out),
								_ => false,
							}
						}
						['s', 'u', 'n', 'i', 'm', s @ ..] => super::t2::n2932(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n1130(s, out),
				['g', s @ ..] => super::t2::n1131(s, out),
				['l', 'l', 'e', 'b', s @ ..] => super::t2::n1132(s, out),
				['m', 'i', s @ ..] => super::t2::n1133(s, out),
				['t', s @ ..] => super::t2::n1134(s, out),
				['v', 'r', 'a', 'p', s @ ..] => super::t2::n1135(s, out),
				_ => false,
			}
		}
		['y', 't', 'c', 'a', 'd', s @ ..] => super::t2::n402(s, out),
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['e', 'r', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1272);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1273);
							match s {
								[] => true,
								['i', 'm', 'i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n1144(s, out),
				['p', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => super::t2::n1146(s, out),
				['s', s @ ..] => super::t2::n1147(s, out),
				['t', s @ ..] => super::t2::n1148(s, out),
				['x', s @ ..] => super::t2::n1149(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n405(s, out),
		['u', s @ ..] => super::t2::n406(s, out),
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1278);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2386);
											s.is_empty()
										}
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'e', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'o', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', 'a', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1279);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1843);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1844);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2387);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 2388);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'i', 'r', 'e', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1281);
							match s {
								[] => true,
								['r', 'e', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1845);
									match s {
										[] => true,
										['i', 'r', 'h', 'c', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2712);
													s.is_empty()
												}
												['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1282);
							match s {
								[] => true,
								['a', 'n', s @ ..] => {
									super::push(out, 1846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n1161(s, out),
				['m', s @ ..] => super::t2::n1162(s, out),
				['n', 'e', 'h', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t2::n1164(s, out),
				['t', s @ ..] => super::t2::n1165(s, out),
				['v', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t2::n2997(s, out),
						_ => false,
					}
				}
				['y', 'a', 'l', 'a', 'm', s @ ..] => super::t2::n1167(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n408(s, out),
		['g', s @ ..] => super::t2::n409(s, out),
		['i', s @ ..] => super::t2::n410(s, out),
		['n', s @ ..] => super::t2::n411(s, out),
		['o', s @ ..] => super::t2::n412(s, out),
		['r', s @ ..] => super::t2::n413(s, out),
		['u', s @ ..] => super::t2::n414(s, out),
		_ => false,
	}
}

/// Words ending -pus.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['o', 'r', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'c', 'o', 'r', 'c', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'i', 'r', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['t', 'o', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['r', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1877);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1878);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'l', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1321);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['i', 'd', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['b', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['n', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1880);
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1881);
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1323);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1324);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'n', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1882);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1328);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1884);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 2396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1885);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['t', s @ ..] => super::t2::n1221(s, out),
				['v', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n422(s, out),
		['o', s @ ..] => super::t2::n423(s, out),
		['r', 'u', 'b', s @ ..] => super::t2::n424(s, out),
		['u', s @ ..] => super::t2::n425(s, out),
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['m', 'u', 'c', 'r', 'i', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1895);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => super::t2::n1256(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n429(s, out),
		['n', s @ ..] => super::t2::n430(s, out),
		['o', s @ ..] => super::t2::n431(s, out),
		['p', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['a', 'l', s @ ..] => super::t2::n1279(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n433(s, out),
		['s', s @ ..] => super::t2::n434(s, out),
		['u', s @ ..] => super::t2::n435(s, out),
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['a', s @ ..] => super::t2::n436(s, out),
		['c', s @ ..] => super::t2::n437(s, out),
		['e', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2083);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2084);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2085);
									s.is_empty()
								}
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2086);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 2087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1478);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['d', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'c', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t2::n1331(s, out),
				['u', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['s', s @ ..] => super::t2::n3524(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t2::n1333(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n439(s, out),
		['l', s @ ..] => super::t2::n440(s, out),
		['n', s @ ..] => super::t2::n441(s, out),
		['o', s @ ..] => super::t2::n442(s, out),
		['p', s @ ..] => super::t2::n443(s, out),
		['r', s @ ..] => super::t2::n444(s, out),
		['s', s @ ..] => super::t2::n445(s, out),
		['u', s @ ..] => super::t2::n446(s, out),
		['x', s @ ..] => super::t2::n447(s, out),
		_ => false,
	}
}

/// Words ending -uus.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2161);
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'p', 's', 'n', 'o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['e', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1549);
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2162);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 379);
			match s {
				[] => true,
				['f', 'r', 'e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2164);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 381);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2166);
									match s {
										[] => true,
										['i', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 890);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2168);
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['i', s @ ..] => super::t2::n3793(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n454(s, out),
		['t', s @ ..] => super::t2::n455(s, out),
		_ => false,
	}
}

/// Words ending -vus.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2170);
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', 'i', 'r', 'i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'a', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 902);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n3812(s, out),
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'e', 'l', 'a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', s @ ..] => super::t2::n8377(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t2::n3814(s, out),
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t2::n8380(s, out),
								['d', 'a', '\u{1}', s @ ..] => super::t2::n8381(s, out),
								_ => false,
							}
						}
						['s', 'e', 'p', 'm', 'e', 't', s @ ..] => super::t2::n3816(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t2::n1418(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n459(s, out),
		['o', s @ ..] => super::t2::n460(s, out),
		['r', s @ ..] => super::t2::n461(s, out),
		_ => false,
	}
}

/// Words ending -xus.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 391);
			match s {
				[] => true,
				['d', 'o', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n466(s, out),
		_ => false,
	}
}

/// The word praestabilior.
pub(crate) fn n19261(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word detestabilior.
pub(crate) fn n19262(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rior.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							super::push(out, 1586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1587);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2176);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 2177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1588);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2178);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 468);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sior.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['l', 'e', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

