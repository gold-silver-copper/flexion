use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['n', s @ ..] => super::t0::n1(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['e', s @ ..] => super::t0::n2(s, out),
		['i', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n4(s, out),
		['r', s @ ..] => super::t0::n5(s, out),
		['u', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 36);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 994);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 282);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 283);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 996);
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
						['d', 'n', 'u', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 284);
							s.is_empty()
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 285);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'h', 'o', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 286);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', 'h', 'c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 287);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 2415);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4708);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'v', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 288);
							match s {
								[] => true,
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -en.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['b', s @ ..] => super::t0::n7(s, out),
		['c', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['a', 'p', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n9(s, out),
		['e', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['n', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1145);
									match s {
										[] => true,
										['i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1146);
									match s {
										[] => true,
										['e', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['f', s @ ..] => super::t0::n11(s, out),
		['g', s @ ..] => super::t0::n12(s, out),
		['h', s @ ..] => super::t0::n13(s, out),
		['i', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 99);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 100);
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 475);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 476);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 477);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1456);
									match s {
										[] => true,
										['c', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5337);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 5338);
													match s {
														[] => true,
														['i', 'e', s @ ..] => super::t0::n10811(s, out),
														_ => false,
													}
												}
												['u', s @ ..] => {
													super::push(out, 5339);
													match s {
														[] => true,
														['z', '\u{1}', s @ ..] => super::t0::n10812(s, out),
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
							super::push(out, 478);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3149);
											s.is_empty()
										}
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3150);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3151);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 479);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1458);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3152);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 3153);
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 1459);
									match s {
										[] => true,
										['c', 's', s @ ..] => super::t0::n3950(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t0::n495(s, out),
						['w', s @ ..] => super::t0::n496(s, out),
						['z', s @ ..] => super::t0::n497(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n101(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t0::n15(s, out),
		['k', s @ ..] => super::t0::n16(s, out),
		['l', s @ ..] => super::t0::n17(s, out),
		['m', s @ ..] => super::t0::n18(s, out),
		['n', s @ ..] => super::t0::n19(s, out),
		['p', s @ ..] => super::t0::n20(s, out),
		['r', s @ ..] => super::t0::n21(s, out),
		['s', s @ ..] => super::t0::n22(s, out),
		['t', s @ ..] => super::t0::n23(s, out),
		['u', s @ ..] => super::t0::n24(s, out),
		['v', s @ ..] => super::t0::n25(s, out),
		['w', s @ ..] => super::t0::n26(s, out),
		['x', s @ ..] => super::t0::n27(s, out),
		['y', s @ ..] => super::t0::n28(s, out),
		['z', s @ ..] => super::t0::n29(s, out),
		['ß', s @ ..] => super::t0::n30(s, out),
		['ä', s @ ..] => super::t0::n31(s, out),
		_ => false,
	}
}

/// Words ending -ln.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['e', s @ ..] => super::t0::n33(s, out),
		['r', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rn.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['e', s @ ..] => super::t0::n35(s, out),
		_ => false,
	}
}

/// Words ending -ben.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 38);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 998);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 999);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1000);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2416);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4709);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 4710);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 4711);
											match s {
												[] => true,
												['e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 4712);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['t', s @ ..] => super::t0::n12512(s, out),
														['z', s @ ..] => super::t0::n12513(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 4713);
											match s {
												[] => true,
												['u', 'a', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n12514(s, out),
														['r', s @ ..] => super::t0::n12515(s, out),
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
							super::push(out, 1001);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1002);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2417);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2418);
									s.is_empty()
								}
								['n', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1003);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'i', 'r', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1004);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => super::t0::n1093(s, out),
						['m', s @ ..] => super::t0::n1094(s, out),
						['n', s @ ..] => super::t0::n1095(s, out),
						['r', s @ ..] => super::t0::n1096(s, out),
						['s', 'u', 'a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => super::t0::n1098(s, out),
						['u', s @ ..] => super::t0::n1099(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n301(s, out),
				['r', s @ ..] => super::t0::n302(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t0::n39(s, out),
		['e', s @ ..] => super::t0::n40(s, out),
		['i', s @ ..] => super::t0::n41(s, out),
		['l', s @ ..] => super::t0::n42(s, out),
		['m', s @ ..] => super::t0::n43(s, out),
		['o', s @ ..] => super::t0::n44(s, out),
		['r', s @ ..] => super::t0::n45(s, out),
		['u', s @ ..] => super::t0::n46(s, out),
		['w', s @ ..] => super::t0::n47(s, out),
		['ö', 'n', 's', '\u{1}', s @ ..] => super::t0::n48(s, out),
		['ü', s @ ..] => super::t0::n49(s, out),
		_ => false,
	}
}

/// Words ending -den.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1082);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2492);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2493);
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
						['e', s @ ..] => {
							super::push(out, 1084);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1085);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1086);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1087);
							match s {
								[] => true,
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1088);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2494);
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
						['r', s @ ..] => {
							super::push(out, 1089);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2495);
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 4795);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4796);
											match s {
												[] => true,
												['n', 'u', 'r', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n12584(s, out),
														['e', 'h', '\u{1}', s @ ..] => super::t0::n12585(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2496);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1090);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1091);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1092);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2497);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2498);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 322);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 52);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1094);
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => super::t0::n6437(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n1204(s, out),
						['s', s @ ..] => super::t0::n1205(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n337(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n54(s, out),
		['i', s @ ..] => super::t0::n55(s, out),
		['l', s @ ..] => super::t0::n56(s, out),
		['m', s @ ..] => super::t0::n57(s, out),
		['n', s @ ..] => super::t0::n58(s, out),
		['o', s @ ..] => super::t0::n59(s, out),
		['r', s @ ..] => super::t0::n60(s, out),
		['u', s @ ..] => super::t0::n61(s, out),
		['y', s @ ..] => super::t0::n62(s, out),
		['ö', s @ ..] => super::t0::n63(s, out),
		['ü', s @ ..] => super::t0::n64(s, out),
		_ => false,
	}
}

/// Words ending -fen.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2605);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 2606);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['o', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'u', 'd', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 2607);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2608);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2609);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2610);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1147);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4838);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 4839);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 4840);
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
			super::push(out, 67);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2612);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4841);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n367(s, out),
				['e', s @ ..] => super::t0::n368(s, out),
				['i', s @ ..] => {
					super::push(out, 355);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['c', 's', s @ ..] => super::t0::n3208(s, out),
								_ => false,
							}
						}
						['k', s @ ..] => super::t0::n1282(s, out),
						['s', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n370(s, out),
				['u', s @ ..] => super::t0::n371(s, out),
				['ä', s @ ..] => super::t0::n372(s, out),
				['ü', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n69(s, out),
		['l', s @ ..] => super::t0::n70(s, out),
		['p', s @ ..] => super::t0::n71(s, out),
		['r', s @ ..] => super::t1::n72(s, out),
		['u', s @ ..] => super::t1::n73(s, out),
		['ü', s @ ..] => super::t1::n74(s, out),
		_ => false,
	}
}

/// Words ending -gen.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => super::t1::n75(s, out),
		['e', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['f', s @ ..] => super::t1::n401(s, out),
				['h', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['i', s @ ..] => super::t1::n403(s, out),
				['l', s @ ..] => super::t1::n404(s, out),
				['r', s @ ..] => super::t1::n405(s, out),
				['w', s @ ..] => super::t1::n406(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n77(s, out),
		['i', s @ ..] => super::t1::n78(s, out),
		['l', s @ ..] => super::t1::n79(s, out),
		['n', s @ ..] => super::t1::n80(s, out),
		['o', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['r', s @ ..] => super::t1::n82(s, out),
		['t', s @ ..] => {
			super::push(out, 83);
			s.is_empty()
		}
		['u', s @ ..] => super::t1::n84(s, out),
		['ä', s @ ..] => super::t1::n85(s, out),
		['ö', s @ ..] => super::t1::n86(s, out),
		['ü', s @ ..] => super::t1::n87(s, out),
		_ => false,
	}
}

/// Words ending -hen.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['f', 'p', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1351);
							match s {
								[] => true,
								['a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n89(s, out),
		['e', s @ ..] => super::t1::n90(s, out),
		['i', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1432);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3118);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3119);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1434);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3120);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3121);
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
						['s', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3122);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3123);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 3124);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3125);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3126);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3127);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3128);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 92);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1439);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3130);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3131);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 5333);
											s.is_empty()
										}
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
			super::push(out, 93);
			match s {
				[] => true,
				['a', 'c', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3132);
									s.is_empty()
								}
								['e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3133);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', 'w', '\u{1}', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1440);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1442);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => super::t1::n3911(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => super::t1::n96(s, out),
		['ö', s @ ..] => super::t1::n97(s, out),
		['ü', s @ ..] => super::t1::n98(s, out),
		_ => false,
	}
}

/// Words ending -einschneien.
pub(crate) fn n10811(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word zuschneien.
pub(crate) fn n10812(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -schreien.
pub(crate) fn n3950(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5340);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5341);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5342);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5343);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7499);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7500);
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
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5344);
			match s {
				[] => true,
				['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -teien.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 480);
	s.is_empty()
}

/// Words ending -weien.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	s.is_empty()
}

/// Words ending -zeien.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	s.is_empty()
}

/// Words ending -nien.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1462);
					match s {
						[] => true,
						['e', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jen.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ken.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['b', 'o', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1463);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1464);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1465);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3154);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3155);
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
				['t', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n104(s, out),
		['e', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3259);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3260);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1523);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3262);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 5436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3264);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 1525);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3265);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n108(s, out),
		['r', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3311);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3312);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3313);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 504);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3314);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3315);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 3316);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => super::t1::n4181(s, out),
								['r', s @ ..] => super::t1::n4182(s, out),
								_ => false,
							}
						}
						['w', s @ ..] => super::t1::n1742(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n520(s, out),
				['o', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['u', s @ ..] => super::t1::n522(s, out),
				['ä', s @ ..] => super::t1::n523(s, out),
				['ü', 't', '\u{1}', s @ ..] => super::t1::n524(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n110(s, out),
		['ä', s @ ..] => super::t1::n111(s, out),
		['ö', s @ ..] => super::t1::n112(s, out),
		_ => false,
	}
}

/// Words ending -len.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1552);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1553);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1556);
							match s {
								[] => true,
								['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 114);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['g', 'g', 'ö', 't', '\u{1}', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5466);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 5467);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 5468);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 5469);
											match s {
												[] => true,
												['c', s @ ..] => {
													match s {
														[] => true,
														['a', 'n', '\u{1}', s @ ..] => super::t1::n13061(s, out),
														['o', 'h', '\u{1}', s @ ..] => super::t1::n13062(s, out),
														['r', 'u', 'd', '\u{1}', s @ ..] => super::t1::n13063(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 5470);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 6883);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n13064(s, out),
														_ => false,
													}
												}
												['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 6884);
													match s {
														[] => true,
														['e', s @ ..] => super::t1::n13065(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 5471);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 6885);
													match s {
														[] => true,
														['b', 'ü', '\u{1}', s @ ..] => super::t1::n13066(s, out),
														['t', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => super::t1::n13067(s, out),
														['v', s @ ..] => super::t1::n13068(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 6886);
													match s {
														[] => true,
														['v', '\u{1}', s @ ..] => super::t1::n13069(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5472);
											match s {
												[] => true,
												['i', 'm', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 5473);
											match s {
												[] => true,
												['z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 1559);
							s.is_empty()
						}
						['z', s @ ..] => super::t1::n1756(s, out),
						_ => false,
					}
				}
				['w', s @ ..] => super::t1::n534(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n116(s, out),
		['h', s @ ..] => super::t1::n117(s, out),
		['i', s @ ..] => super::t1::n118(s, out),
		['l', s @ ..] => super::t1::n119(s, out),
		['o', s @ ..] => super::t1::n120(s, out),
		['r', s @ ..] => super::t1::n121(s, out),
		['u', s @ ..] => super::t1::n122(s, out),
		['w', s @ ..] => super::t1::n123(s, out),
		['y', s @ ..] => super::t1::n124(s, out),
		['ä', s @ ..] => super::t1::n125(s, out),
		['ö', s @ ..] => super::t1::n126(s, out),
		['ü', s @ ..] => super::t1::n127(s, out),
		_ => false,
	}
}

/// Words ending -men.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 3475);
									match s {
										[] => true,
										['c', 'o', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 548);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3476);
									s.is_empty()
								}
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1655);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 3478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3479);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 5557);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3480);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 556);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1659);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t1::n1872(s, out),
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3494);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3496);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => super::t1::n578(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n132(s, out),
		['l', s @ ..] => super::t1::n133(s, out),
		['m', s @ ..] => super::t1::n134(s, out),
		['o', s @ ..] => super::t1::n135(s, out),
		['r', s @ ..] => super::t1::n136(s, out),
		['t', s @ ..] => super::t1::n137(s, out),
		['u', s @ ..] => super::t1::n138(s, out),
		['ä', s @ ..] => super::t1::n139(s, out),
		['ö', s @ ..] => super::t1::n140(s, out),
		_ => false,
	}
}

/// Words ending -nen.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 583);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3562);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3563);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3564);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3565);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5622);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 5623);
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
								['r', s @ ..] => {
									super::push(out, 3566);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 6967);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n13197(s, out),
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
									super::push(out, 3567);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 144);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3568);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 3569);
									match s {
										[] => true,
										['c', 'o', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3570);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3571);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3572);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3573);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ö', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3574);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'ä', '\u{1}', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['e', s @ ..] => super::t1::n612(s, out),
				['i', s @ ..] => super::t1::n613(s, out),
				['u', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3586);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'l', s @ ..] => super::t1::n1948(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t1::n147(s, out),
		['i', s @ ..] => super::t1::n148(s, out),
		['k', s @ ..] => super::t1::n149(s, out),
		['m', s @ ..] => super::t1::n150(s, out),
		['n', s @ ..] => super::t1::n151(s, out),
		['o', s @ ..] => super::t1::n152(s, out),
		['p', s @ ..] => super::t1::n153(s, out),
		['r', s @ ..] => super::t1::n154(s, out),
		['s', 'u', 'a', 'j', '\u{1}', s @ ..] => super::t1::n155(s, out),
		['u', s @ ..] => super::t1::n156(s, out),
		['ä', s @ ..] => super::t1::n157(s, out),
		['ö', s @ ..] => super::t1::n158(s, out),
		['ü', s @ ..] => super::t1::n159(s, out),
		_ => false,
	}
}

/// Words ending -pen.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 5685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1779);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5686);
											match s {
												[] => true,
												['e', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 1780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3696);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3697);
									match s {
										[] => true,
										['l', 'o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 164);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 1781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3698);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1782);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3699);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 3700);
									s.is_empty()
								}
								['k', s @ ..] => super::t1::n4744(s, out),
								['r', s @ ..] => {
									super::push(out, 3702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t1::n2033(s, out),
						['r', s @ ..] => super::t1::n2034(s, out),
						['t', s @ ..] => super::t1::n2035(s, out),
						['z', s @ ..] => super::t1::n2036(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n654(s, out),
				['i', s @ ..] => super::t1::n655(s, out),
				['o', s @ ..] => super::t1::n656(s, out),
				['u', s @ ..] => super::t1::n657(s, out),
				['ö', s @ ..] => super::t1::n658(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n166(s, out),
		['u', s @ ..] => super::t1::n167(s, out),
		['y', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ren.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3715);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 629);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['k', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3716);
									match s {
										[] => true,
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 5701);
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
								['r', s @ ..] => {
									super::push(out, 3717);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3718);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3719);
									match s {
										[] => true,
										['t', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n170(s, out),
		['h', s @ ..] => super::t1::n171(s, out),
		['i', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['c', 'u', 'd', 'o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1833);
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['m', 'h', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n682(s, out),
				['e', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['p', 's', s @ ..] => super::t1::n2110(s, out),
						['z', s @ ..] => {
							super::push(out, 1838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3865);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n684(s, out),
				['o', s @ ..] => super::t1::n685(s, out),
				['u', s @ ..] => super::t1::n686(s, out),
				['ö', s @ ..] => super::t1::n687(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n175(s, out),
		['ä', s @ ..] => super::t1::n176(s, out),
		['ö', s @ ..] => super::t1::n177(s, out),
		['ü', s @ ..] => super::t1::n178(s, out),
		_ => false,
	}
}

/// Words ending -sen.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3894);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3895);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 3896);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3897);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5806);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 5807);
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
				['r', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1862);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 180);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 1863);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5808);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 5809);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 1864);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1866);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1868);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1869);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 3899);
									match s {
										[] => true,
										['g', 'e', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3900);
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
						['r', s @ ..] => {
							super::push(out, 1870);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5810);
											s.is_empty()
										}
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 5811);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3902);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1871);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['h', s @ ..] => super::t2::n183(s, out),
		['i', s @ ..] => super::t2::n184(s, out),
		['k', s @ ..] => super::t2::n185(s, out),
		['l', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n187(s, out),
		['n', s @ ..] => super::t2::n188(s, out),
		['o', s @ ..] => super::t2::n189(s, out),
		['p', s @ ..] => super::t2::n190(s, out),
		['r', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['s', s @ ..] => super::t2::n192(s, out),
		['t', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		['u', s @ ..] => super::t2::n194(s, out),
		['w', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['ä', s @ ..] => super::t2::n196(s, out),
		['ö', s @ ..] => super::t2::n197(s, out),
		['ü', s @ ..] => super::t2::n198(s, out),
		_ => false,
	}
}

/// Words ending -ten.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4025);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4026);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 4027);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4028);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4029);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1946);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 4031);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 5901);
											match s {
												[] => true,
												['c', 'a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 1947);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4032);
									match s {
										[] => true,
										['h', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7134);
													s.is_empty()
												}
												['r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 7135);
													s.is_empty()
												}
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
							super::push(out, 1948);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1949);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1950);
							match s {
								[] => true,
								['s', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ß', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1951);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'ö', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n201(s, out),
		['f', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n768(s, out),
				['e', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['h', s @ ..] => super::t2::n2275(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n770(s, out),
				['u', s @ ..] => super::t2::n771(s, out),
				['ä', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['ü', s @ ..] => super::t2::n773(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t2::n203(s, out),
		['i', s @ ..] => super::t2::n204(s, out),
		['k', s @ ..] => super::t2::n205(s, out),
		['l', s @ ..] => super::t2::n206(s, out),
		['m', s @ ..] => super::t2::n207(s, out),
		['n', s @ ..] => super::t2::n208(s, out),
		['o', s @ ..] => super::t2::n209(s, out),
		['p', s @ ..] => super::t2::n210(s, out),
		['r', s @ ..] => super::t2::n211(s, out),
		['s', s @ ..] => super::t2::n212(s, out),
		['t', s @ ..] => super::t2::n213(s, out),
		['u', s @ ..] => super::t2::n214(s, out),
		['x', s @ ..] => super::t2::n215(s, out),
		['ä', s @ ..] => super::t2::n216(s, out),
		['ö', s @ ..] => super::t2::n217(s, out),
		['ü', s @ ..] => super::t2::n218(s, out),
		_ => false,
	}
}

/// Words ending -uen.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2065);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2066);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2067);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4250);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2068);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2069);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2070);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 4251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2071);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4253);
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
						['r', s @ ..] => super::t2::n2419(s, out),
						['s', s @ ..] => super::t2::n2420(s, out),
						['u', s @ ..] => super::t2::n2421(s, out),
						['ß', 'u', 'a', '\u{1}', s @ ..] => super::t2::n2422(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n839(s, out),
				['h', s @ ..] => super::t2::n840(s, out),
				['i', s @ ..] => super::t2::n841(s, out),
				['k', s @ ..] => super::t2::n842(s, out),
				['l', s @ ..] => super::t2::n843(s, out),
				['m', '\u{1}', s @ ..] => super::t2::n844(s, out),
				['r', s @ ..] => super::t2::n845(s, out),
				['s', s @ ..] => super::t2::n846(s, out),
				['t', s @ ..] => super::t2::n847(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n220(s, out),
		['ä', s @ ..] => super::t2::n221(s, out),
		_ => false,
	}
}

/// Words ending -ven.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wen.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['e', 'i', 'v', 'r', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['t', 'i', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xen.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2105);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2106);
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
				['s', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['j', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['l', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['b', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yen.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	s.is_empty()
}

/// Words ending -zen.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 6061);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 6062);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4285);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 233);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4286);
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
						['g', s @ ..] => {
							super::push(out, 2108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4287);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4288);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4289);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 4290);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4291);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6063);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 6064);
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
								['r', s @ ..] => {
									super::push(out, 4292);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6065);
											match s {
												[] => true,
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4293);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4294);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4295);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4296);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => super::t2::n2471(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t2::n873(s, out),
				['i', s @ ..] => super::t2::n874(s, out),
				['o', s @ ..] => super::t2::n875(s, out),
				['ä', s @ ..] => super::t2::n876(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n238(s, out),
		['r', s @ ..] => super::t2::n239(s, out),
		['t', s @ ..] => super::t2::n240(s, out),
		['u', s @ ..] => super::t2::n241(s, out),
		['z', s @ ..] => super::t2::n242(s, out),
		_ => false,
	}
}

/// Words ending -ßen.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['i', s @ ..] => super::t2::n901(s, out),
				['m', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n245(s, out),
		['o', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['s', s @ ..] => super::t2::n2561(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2188);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['e', s @ ..] => super::t2::n908(s, out),
				_ => false,
			}
		}
		['ö', s @ ..] => super::t2::n249(s, out),
		['ü', s @ ..] => super::t2::n250(s, out),
		_ => false,
	}
}

/// Words ending -äen.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 855);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eln.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4439);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 857);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2200);
							match s {
								[] => true,
								['r', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 6168);
											match s {
												[] => true,
												['u', 'r', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n13746(s, out),
														['e', 'h', '\u{1}', s @ ..] => super::t2::n13747(s, out),
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
							super::push(out, 2201);
							match s {
								[] => true,
								['n', 'b', s @ ..] => {
									super::push(out, 4440);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6169);
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
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 858);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4441);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2202);
							match s {
								[] => true,
								['w', 'z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6170);
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
								['e', s @ ..] => {
									super::push(out, 4442);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4443);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 859);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4444);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6171);
											s.is_empty()
										}
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
					super::push(out, 860);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2203);
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6172);
											s.is_empty()
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t2::n2586(s, out),
						['u', s @ ..] => super::t2::n2587(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n921(s, out),
				['ä', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['ö', s @ ..] => super::t2::n923(s, out),
				['ü', 'r', 'e', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'y', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['d', s @ ..] => super::t2::n254(s, out),
		['f', s @ ..] => super::t2::n255(s, out),
		['g', s @ ..] => super::t2::n256(s, out),
		['h', s @ ..] => super::t2::n257(s, out),
		['k', s @ ..] => super::t2::n258(s, out),
		['m', s @ ..] => super::t2::n259(s, out),
		['n', 'n', 'u', 't', s @ ..] => super::t2::n260(s, out),
		['p', s @ ..] => super::t2::n261(s, out),
		['s', s @ ..] => super::t2::n262(s, out),
		['t', s @ ..] => super::t2::n263(s, out),
		['v', s @ ..] => super::t2::n264(s, out),
		['x', 'i', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['z', s @ ..] => super::t2::n266(s, out),
		['ß', s @ ..] => super::t2::n267(s, out),
		_ => false,
	}
}

/// Words ending -ern.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4573);
									s.is_empty()
								}
								['u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 4574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 2298);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 6312);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 6313);
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['i', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4575);
									match s {
										[] => true,
										['e', 'g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 927);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 2299);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2300);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 6314);
											match s {
												[] => true,
												['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 929);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4576);
									s.is_empty()
								}
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 6315);
											match s {
												[] => true,
												['e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 7333);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', 's', '\u{1}', s @ ..] => {
							super::push(out, 2301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4577);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6316);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 4578);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['d', s @ ..] => super::t2::n270(s, out),
		['f', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => super::t2::n6084(s, out),
								['l', s @ ..] => super::t2::n6085(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t2::n1010(s, out),
				['g', s @ ..] => super::t2::n1011(s, out),
				['i', s @ ..] => super::t2::n1012(s, out),
				['p', s @ ..] => super::t2::n1013(s, out),
				['u', s @ ..] => super::t2::n1014(s, out),
				['ä', s @ ..] => super::t2::n1015(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t3::n272(s, out),
		['h', s @ ..] => super::t3::n273(s, out),
		['i', s @ ..] => super::t3::n274(s, out),
		['k', s @ ..] => super::t3::n275(s, out),
		['l', s @ ..] => super::t3::n276(s, out),
		['m', s @ ..] => super::t3::n277(s, out),
		['n', s @ ..] => super::t3::n278(s, out),
		['p', s @ ..] => super::t3::n279(s, out),
		['s', s @ ..] => super::t3::n280(s, out),
		['t', s @ ..] => super::t3::n281(s, out),
		['u', s @ ..] => super::t3::n282(s, out),
		['v', 'l', 'u', 'p', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t3::n283(s, out),
		['w', s @ ..] => super::t3::n284(s, out),
		['z', s @ ..] => super::t3::n285(s, out),
		['ß', s @ ..] => super::t3::n286(s, out),
		_ => false,
	}
}

/// Words ending -terschaben.
pub(crate) fn n12512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7378);
	match s {
		[] => true,
		['n', 'u', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zerschaben.
pub(crate) fn n12513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7379);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ausschaben.
pub(crate) fn n12514(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rausschaben.
pub(crate) fn n12515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7380);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lhaben.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	match s {
		[] => true,
		['i', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mhaben.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2419);
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

/// Words ending -nhaben.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2420);
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

/// Words ending -rhaben.
pub(crate) fn n1096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	match s {
		[] => true,
		['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2421);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -thaben.
pub(crate) fn n1098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	match s {
		[] => true,
		['t', 'a', 't', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uhaben.
pub(crate) fn n1099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1010);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 2422);
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

/// Words ending -laben.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1012);
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

/// Words ending -raben.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2423);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2424);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2426);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2428);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bben.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eben.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1017);
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
		['g', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2432);
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
				['d', 'n', 'u', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1020);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2433);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4715);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['i', 'e', 'h', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2434);
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
				['n', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2436);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2437);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 4716);
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
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4717);
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 4718);
									s.is_empty()
								}
								['d', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 4719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2439);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1029);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2440);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'a', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1030);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n307(s, out),
		['i', s @ ..] => super::t3::n308(s, out),
		['l', s @ ..] => super::t3::n309(s, out),
		['r', s @ ..] => super::t3::n310(s, out),
		['w', s @ ..] => super::t3::n311(s, out),
		_ => false,
	}
}

/// Words ending -iben.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['b', s @ ..] => super::t3::n2974(s, out),
						['e', s @ ..] => {
							super::push(out, 2456);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2457);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4762);
									match s {
										[] => true,
										['v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4763);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n1153(s, out),
				['w', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lben.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2468);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -mben.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['o', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oben.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1065);
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
		['t', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rben.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2471);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2472);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4779);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4780);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 4781);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4782);
									match s {
										[] => true,
										['i', 'h', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 7398);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t3::n13873(s, out),
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
									super::push(out, 4783);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4784);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2474);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2475);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2476);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2477);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2478);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2479);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4785);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', s @ ..] => {
									super::push(out, 4786);
									match s {
										[] => true,
										['h', 'c', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2480);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2481);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uben.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4787);
									s.is_empty()
								}
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4789);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2484);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2485);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 6493);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 6494);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 6495);
											match s {
												[] => true,
												['c', 'o', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 6496);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 6497);
											match s {
												[] => true,
												['e', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 6498);
											match s {
												[] => true,
												['o', 'l', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 6499);
											match s {
												[] => true,
												['s', 'e', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 6500);
											match s {
												[] => true,
												['z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2486);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4790);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4791);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6501);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4792);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4793);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4794);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['ä', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2489);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -wben.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	s.is_empty()
}

/// The word snöben.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -üben.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 317);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word runterladen.
pub(crate) fn n12584(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herunterladen.
pub(crate) fn n12585(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -sschmieden.
pub(crate) fn n6437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4797);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rieden.
pub(crate) fn n1204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1095);
	match s {
		[] => true,
		['f', 'n', s @ ..] => {
			super::push(out, 2499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sieden.
pub(crate) fn n1205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1096);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reden.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1097);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'k', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', 'h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 6502);
									s.is_empty()
								}
								['i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4798);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 6503);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['o', 'l', 'f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2505);
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
		_ => false,
	}
}

/// Words ending -hden.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// Words ending -iden.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2506);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6504);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 6505);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 7399);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 6506);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['t', 'n', 'u', '\u{1}', s @ ..] => super::t3::n13874(s, out),
														['v', s @ ..] => super::t3::n13875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 6507);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 7400);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2508);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4799);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4800);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4801);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4802);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2510);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6508);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2511);
							match s {
								[] => true,
								['i', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6509);
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2513);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2515);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t3::n3072(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n1220(s, out),
				['w', s @ ..] => super::t3::n1221(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lden.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2520);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2521);
							match s {
								[] => true,
								['n', 'u', 's', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2522);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2523);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4804);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4805);
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
						['r', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 329);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2527);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2528);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2529);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2530);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2531);
							match s {
								[] => true,
								['a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4806);
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
						['r', s @ ..] => {
							super::push(out, 2532);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4807);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4808);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2533);
							match s {
								[] => true,
								['s', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4809);
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
						['t', s @ ..] => {
							super::push(out, 2534);
							match s {
								[] => true,
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 2535);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2538);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 4811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2539);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mden.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	s.is_empty()
}

/// Words ending -nden.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2540);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4812);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4813);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4814);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4815);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6520);
											match s {
												[] => true,
												['h', 'c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 6521);
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
								['r', s @ ..] => {
									super::push(out, 4816);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4817);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2544);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2545);
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
				['s', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2549);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6522);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6523);
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
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2550);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4818);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t3::n12604(s, out),
												['r', 'o', 'v', '\u{1}', s @ ..] => super::t3::n12605(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t3::n3116(s, out),
						['u', s @ ..] => super::t3::n3117(s, out),
						_ => false,
					}
				}
				['w', s @ ..] => super::t3::n1237(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n346(s, out),
		['i', s @ ..] => super::t3::n347(s, out),
		['o', s @ ..] => super::t3::n348(s, out),
		['u', s @ ..] => super::t3::n349(s, out),
		['ä', s @ ..] => super::t3::n350(s, out),
		['ü', s @ ..] => super::t3::n351(s, out),
		_ => false,
	}
}

/// Words ending -oden.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	s.is_empty()
}

/// Words ending -rden.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						['e', 'n', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'i', 't', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2599);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2601);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2602);
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
				['n', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uden.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	s.is_empty()
}

/// Words ending -yden.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öden.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['b', 'r', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üden.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['m', 'r', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -affen.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2613);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2614);
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
		['h', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4842);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4844);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4845);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4846);
							match s {
								[] => true,
								['e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4847);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4848);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7405);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 7406);
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
								_ => false,
							}
						}
						['s', 'u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4849);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4850);
							match s {
								[] => true,
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1154);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4851);
							match s {
								[] => true,
								['e', 'd', 'n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1155);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2617);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2618);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'w', 'n', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2619);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2620);
					match s {
						[] => true,
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'h', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2621);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2622);
					match s {
						[] => true,
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

/// Words ending -effen.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2623);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2624);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4852);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7407);
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
						['f', s @ ..] => {
							super::push(out, 4853);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4854);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6534);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6535);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4855);
							match s {
								[] => true,
								['e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4856);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -schiffen.
pub(crate) fn n3208(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4857);
			s.is_empty()
		}
		['m', 'u', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kiffen.
pub(crate) fn n1282(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2625);
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -offen.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2627);
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

/// Words ending -uffen.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2628);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äffen.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ifen.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2629);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4859);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4860);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 4861);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4862);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6536);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4863);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['h', 'r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 7408);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n13884(s, out),
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
									super::push(out, 4864);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1162);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2630);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6537);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 6538);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 6539);
											match s {
												[] => true,
												['e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 6540);
											match s {
												[] => true,
												['r', 'o', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 1163);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4865);
									match s {
										[] => true,
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4866);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n1292(s, out),
				['s', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4879);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2640);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => super::t3::n10080(s, out),
										['s', s @ ..] => super::t3::n10081(s, out),
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

/// Words ending -lfen.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2642);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2644);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2645);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 6548);
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
								['o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2647);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2648);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -pfen.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2649);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2651);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 363);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4881);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6549);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4882);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 6550);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4883);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4884);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
								['r', s @ ..] => {
									super::push(out, 4885);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2652);
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4886);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 4887);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4888);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6551);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2654);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2655);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4890);
									match s {
										[] => true,
										['c', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['d', 'n', 'u', 's', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 7411);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 7412);
													match s {
														[] => true,
														['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => super::t3::n13887(s, out),
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 6552);
											match s {
												[] => true,
												['e', 'b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4891);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4892);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2657);
							match s {
								[] => true,
								['s', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => super::t3::n1301(s, out),
				['ü', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t3::n378(s, out),
		['u', s @ ..] => super::t3::n379(s, out),
		['ö', s @ ..] => super::t3::n380(s, out),
		['ü', s @ ..] => super::t3::n381(s, out),
		_ => false,
	}
}

