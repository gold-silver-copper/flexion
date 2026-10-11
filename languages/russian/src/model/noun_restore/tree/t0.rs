use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['а', s @ ..] => super::t0::n1(s, out),
		['б', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 443);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 444);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 445);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 446);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 447);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1204);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 1205);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 1206);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 1207);
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2457);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 2458);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 2459);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 2460);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 2461);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 448);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 449);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 1208);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 2462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'ж', 'о', s @ ..] => {
									super::push(out, 1209);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 1210);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 3960);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 3961);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 1211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 1212);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 1213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 450);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 451);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 57);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 453);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 454);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 62);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 63);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 456);
							s.is_empty()
						}
						['д', 'д', s @ ..] => {
							super::push(out, 457);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 458);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 1215);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 1216);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 1217);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 1218);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 2463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'р', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 2464);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 2465);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 1219);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 1220);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 1221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1222);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 1223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 459);
							s.is_empty()
						}
						['р', 'г', s @ ..] => {
							super::push(out, 460);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 461);
							s.is_empty()
						}
						['ч', s @ ..] => super::t0::n467(s, out),
						['ь', s @ ..] => super::t0::n468(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t0::n66(s, out),
				['о', s @ ..] => super::t0::n67(s, out),
				['р', s @ ..] => super::t0::n68(s, out),
				['ы', s @ ..] => super::t0::n69(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t0::n4(s, out),
		['д', s @ ..] => super::t0::n5(s, out),
		['е', s @ ..] => super::t0::n6(s, out),
		['ж', s @ ..] => super::t0::n7(s, out),
		['з', s @ ..] => super::t0::n8(s, out),
		['и', s @ ..] => super::t0::n9(s, out),
		['й', s @ ..] => super::t0::n10(s, out),
		['к', s @ ..] => super::t0::n11(s, out),
		['л', s @ ..] => super::t0::n12(s, out),
		['м', s @ ..] => super::t0::n13(s, out),
		['н', s @ ..] => super::t0::n14(s, out),
		['о', s @ ..] => super::t0::n15(s, out),
		['п', s @ ..] => super::t0::n16(s, out),
		['р', s @ ..] => super::t0::n17(s, out),
		['с', s @ ..] => super::t0::n18(s, out),
		['т', s @ ..] => super::t0::n19(s, out),
		['у', s @ ..] => super::t0::n20(s, out),
		['ф', s @ ..] => super::t0::n21(s, out),
		['х', s @ ..] => super::t0::n22(s, out),
		['ц', s @ ..] => super::t0::n23(s, out),
		['ч', s @ ..] => super::t0::n24(s, out),
		['ш', s @ ..] => super::t0::n25(s, out),
		['щ', s @ ..] => super::t0::n26(s, out),
		['ы', s @ ..] => super::t0::n27(s, out),
		['ь', s @ ..] => super::t0::n28(s, out),
		['э', s @ ..] => super::t0::n29(s, out),
		['ю', s @ ..] => super::t0::n30(s, out),
		['я', s @ ..] => super::t0::n31(s, out),
		_ => false,
	}
}

/// Words ending -а.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 965);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 966);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 967);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 969);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 970);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 971);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 972);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['р', '-', s @ ..] => {
							super::push(out, 974);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2123);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 975);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 325);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 327);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 328);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 980);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 3484);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 3485);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', 'д', s @ ..] => {
							super::push(out, 981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 329);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 982);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', s @ ..] => {
					super::push(out, 331);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 332);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 333);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 335);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 986);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 337);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 988);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2130);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 340);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 989);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 992);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['е', s @ ..] => super::t0::n348(s, out),
				['и', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t0::n38(s, out),
		['л', s @ ..] => super::t0::n39(s, out),
		['м', s @ ..] => super::t0::n40(s, out),
		['н', s @ ..] => super::t0::n41(s, out),
		['о', s @ ..] => {
			super::push(out, 42);
			s.is_empty()
		}
		['п', s @ ..] => super::t0::n43(s, out),
		['р', s @ ..] => super::t0::n44(s, out),
		['с', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['е', s @ ..] => super::t0::n414(s, out),
				['с', s @ ..] => super::t0::n415(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t0::n46(s, out),
		['у', s @ ..] => super::t0::n47(s, out),
		['ф', s @ ..] => super::t0::n48(s, out),
		['х', s @ ..] => super::t0::n49(s, out),
		['ц', s @ ..] => super::t0::n50(s, out),
		['ч', s @ ..] => super::t0::n51(s, out),
		['ш', s @ ..] => super::t0::n52(s, out),
		['щ', s @ ..] => super::t0::n53(s, out),
		_ => false,
	}
}

/// Words ending -чев.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	s.is_empty()
}

/// Words ending -ьев.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	s.is_empty()
}

/// Words ending -ив.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// Words ending -ов.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рв.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['е', 'к', s @ ..] => {
			super::push(out, 466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыв.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	s.is_empty()
}

/// Words ending -г.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1228);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2466);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2467);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 3962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 1230);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 2468);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2469);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2470);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 4963);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 4964);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', 'м', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2471);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 3964);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 3965);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 2472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1233);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1234);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2473);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 3966);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 3967);
											match s {
												[] => true,
												['н', 'е', s @ ..] => {
													super::push(out, 4965);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 3968);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 2474);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2475);
									match s {
										[] => true,
										['а', 'ж', s @ ..] => {
											super::push(out, 3969);
											match s {
												[] => true,
												['д', '-', s @ ..] => {
													super::push(out, 4966);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 2476);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 2477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 1238);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2478);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2479);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2480);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 2481);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2482);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3970);
											match s {
												[] => true,
												['с', 'т', s @ ..] => {
													super::push(out, 4967);
													match s {
														[] => true,
														['й', s @ ..] => super::t0::n5830(s, out),
														['р', s @ ..] => super::t0::n5831(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 3971);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 2483);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 2484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 1239);
							match s {
								[] => true,
								['т', 'о', s @ ..] => {
									super::push(out, 2485);
									match s {
										[] => true,
										['в', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 4968);
													s.is_empty()
												}
												['к', s @ ..] => {
													super::push(out, 4969);
													s.is_empty()
												}
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
				['о', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 74);
			s.is_empty()
		}
		['р', s @ ..] => super::t0::n75(s, out),
		['у', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -д.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1252);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2489);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 79);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						['о', 'б', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2491);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1260);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2493);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 2494);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 3972);
											s.is_empty()
										}
										['й', s @ ..] => {
											super::push(out, 3973);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 3974);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				['у', 'р', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 490);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1267);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 2495);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 3975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['н', 'с', s @ ..] => {
							super::push(out, 1270);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2496);
									s.is_empty()
								}
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

/// Words ending -е.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['н', 'е', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 89);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 90);
			match s {
				[] => true,
				['д', 'е', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t0::n91(s, out),
		['к', s @ ..] => {
			super::push(out, 92);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'е', s @ ..] => {
					super::push(out, 515);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1293);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 2533);
									match s {
										[] => true,
										['л', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 5017);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 5018);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 2534);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1295);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2536);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1296);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1301);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 2539);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 2540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2541);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2542);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4008);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 4009);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1305);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2543);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2544);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 4010);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 4011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 2545);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2546);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 2547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1308);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t0::n104(s, out),
		['ь', s @ ..] => super::t0::n105(s, out),
		_ => false,
	}
}

/// Words ending -ж.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'и', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1398);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 109);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['р', 'о', 'д', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 114);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -з.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1405);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 2703);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2705);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4096);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 2706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1409);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2707);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							super::push(out, 1413);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -и.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 124);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		['з', 'е', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 129);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1415);
							match s {
								[] => true,
								['р', 'к', 'с', 'ы', s @ ..] => {
									super::push(out, 2711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', 'е', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1416);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'е', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1418);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2712);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 2713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['а', 'н', 'т', 'е', 'ь', 'в', '\u{1}', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						['е', 'т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2714);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 2715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1420);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1421);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2716);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 2717);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 2718);
									s.is_empty()
								}
								['ж', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4098);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 4099);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'а', s @ ..] => {
									super::push(out, 2719);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2720);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2721);
									s.is_empty()
								}
								['с', 'о', s @ ..] => {
									super::push(out, 2722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1422);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['п', 'е', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['р', 'е', 'з', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1424);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2723);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1425);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1426);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1428);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 2725);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2726);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 2727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1429);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 2728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1430);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1431);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2729);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4100);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 4101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 2730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1432);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1433);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 2731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 131);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 1434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['а', 'т', '-', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2732);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['с', s @ ..] => super::t0::n605(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -й.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1439);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2738);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4102);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 5042);
													s.is_empty()
												}
												['л', 'е', s @ ..] => {
													super::push(out, 5043);
													s.is_empty()
												}
												['м', s @ ..] => {
													super::push(out, 5044);
													match s {
														[] => true,
														['е', s @ ..] => super::t0::n5909(s, out),
														['и', s @ ..] => super::t0::n5910(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 5045);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 2739);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 5046);
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 5047);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 2740);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2741);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4103);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 5048);
													s.is_empty()
												}
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
				['н', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 1442);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									super::push(out, 2742);
									match s {
										[] => true,
										['о', 'м', 'с', 'д', s @ ..] => {
											super::push(out, 4104);
											s.is_empty()
										}
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
		['о', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['о', 'б', s @ ..] => {
							super::push(out, 1444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1446);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2743);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 4105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['л', s @ ..] => super::t0::n615(s, out),
				['м', s @ ..] => super::t0::n616(s, out),
				['н', s @ ..] => super::t0::n617(s, out),
				['р', s @ ..] => super::t0::n618(s, out),
				['т', s @ ..] => super::t0::n619(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t0::n144(s, out),
		_ => false,
	}
}

/// Words ending -к.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1464);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 1468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 1473);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 2768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1474);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 2769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1480);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 2770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t0::n632(s, out),
				['п', s @ ..] => super::t0::n633(s, out),
				['р', s @ ..] => super::t0::n634(s, out),
				['с', s @ ..] => super::t0::n635(s, out),
				['т', s @ ..] => super::t0::n636(s, out),
				['у', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['х', 'д', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['ч', s @ ..] => super::t0::n639(s, out),
				['ш', s @ ..] => super::t0::n640(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t0::n147(s, out),
		['й', s @ ..] => super::t0::n148(s, out),
		['к', s @ ..] => super::t0::n149(s, out),
		['л', s @ ..] => super::t0::n150(s, out),
		['н', s @ ..] => super::t0::n151(s, out),
		['о', s @ ..] => super::t0::n152(s, out),
		['р', s @ ..] => super::t0::n153(s, out),
		['с', s @ ..] => super::t0::n154(s, out),
		['у', s @ ..] => super::t0::n155(s, out),
		['ц', s @ ..] => super::t0::n156(s, out),
		['ы', s @ ..] => super::t0::n157(s, out),
		['э', s @ ..] => super::t0::n158(s, out),
		['ю', s @ ..] => super::t0::n159(s, out),
		['я', s @ ..] => super::t0::n160(s, out),
		_ => false,
	}
}

/// Words ending -л.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2996);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 2997);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 4345);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 4346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['е', 'г', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -м.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 1653);
							match s {
								[] => true,
								['с', 'г', s @ ..] => {
									super::push(out, 3000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 698);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1657);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3001);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 3002);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4347);
											s.is_empty()
										}
										['й', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 5205);
													s.is_empty()
												}
												['э', s @ ..] => {
													super::push(out, 5206);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1659);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3003);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 4348);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 5207);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 5208);
													s.is_empty()
												}
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
		['и', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 177);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 179);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -н.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1662);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1663);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3004);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3005);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 3006);
									match s {
										[] => true,
										['м', 'о', s @ ..] => {
											super::push(out, 4349);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 1664);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3007);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1673);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 1674);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3009);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3010);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3012);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 3013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 3014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1677);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 3015);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1678);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1679);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3018);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 3019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'к', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1684);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3020);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 4350);
											s.is_empty()
										}
										['ф', s @ ..] => {
											super::push(out, 4351);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1685);
							match s {
								[] => true,
								['ч', 'в', s @ ..] => {
									super::push(out, 3021);
									match s {
										[] => true,
										['о', 'м', s @ ..] => {
											super::push(out, 4352);
											s.is_empty()
										}
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
		['й', s @ ..] => {
			super::push(out, 184);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1686);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 3022);
									match s {
										[] => true,
										['-', 'т', s @ ..] => {
											super::push(out, 4353);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1687);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1688);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t0::n188(s, out),
		['ь', s @ ..] => super::t0::n189(s, out),
		['э', s @ ..] => super::t0::n190(s, out),
		['я', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -о.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1697);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3031);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 3032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3034);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 4354);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4355);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 4356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						['к', 'л', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t0::n752(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1706);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 3047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['к', s @ ..] => super::t0::n198(s, out),
		['л', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['н', s @ ..] => super::t0::n201(s, out),
		['р', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t0::n203(s, out),
		['т', s @ ..] => super::t0::n204(s, out),
		['х', s @ ..] => super::t0::n205(s, out),
		['ц', s @ ..] => super::t0::n206(s, out),
		_ => false,
	}
}

/// Words ending -п.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1724);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3060);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['т', '-', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 3062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -р.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['а', 'м', 'р', s @ ..] => {
									super::push(out, 3063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['е', s @ ..] => super::t1::n217(s, out),
		['и', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1858);
							match s {
								[] => true,
								['а', 'к', '-', s @ ..] => {
									super::push(out, 3246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['к', 'е', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['и', 'в', s @ ..] => {
							super::push(out, 1860);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									super::push(out, 3247);
									match s {
										[] => true,
										['-', 'р', 'е', 'л', s @ ..] => {
											super::push(out, 4475);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1861);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							super::push(out, 1862);
							match s {
								[] => true,
								['и', 'ф', s @ ..] => {
									super::push(out, 3250);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4476);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4477);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1865);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -с.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1868);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 3253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1876);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 3254);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 3255);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3256);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						['с', 'о', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 234);
			s.is_empty()
		}
		['р', 'е', 'в', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1882);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3260);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
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

/// Words ending -т.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1888);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['а', 'н', 'о', 'м', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['с', 'о', 'к', '-', 'к', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 815);
					match s {
						[] => true,
						['а', s @ ..] => super::t1::n1973(s, out),
						['б', s @ ..] => {
							super::push(out, 1890);
							match s {
								[] => true,
								['а', s @ ..] => super::t1::n3457(s, out),
								['о', s @ ..] => super::t1::n3458(s, out),
								['у', s @ ..] => super::t1::n3459(s, out),
								_ => false,
							}
						}
						['г', s @ ..] => super::t1::n1975(s, out),
						['д', s @ ..] => super::t1::n1976(s, out),
						['е', s @ ..] => super::t1::n1977(s, out),
						['з', s @ ..] => super::t1::n1978(s, out),
						['и', s @ ..] => super::t1::n1979(s, out),
						['к', s @ ..] => super::t1::n1980(s, out),
						['л', s @ ..] => super::t1::n1981(s, out),
						['м', s @ ..] => super::t1::n1982(s, out),
						['н', s @ ..] => super::t1::n1983(s, out),
						['о', s @ ..] => super::t1::n1984(s, out),
						['п', s @ ..] => super::t1::n1985(s, out),
						['с', s @ ..] => super::t1::n1986(s, out),
						['т', s @ ..] => super::t1::n1987(s, out),
						['у', s @ ..] => super::t1::n1988(s, out),
						['ф', s @ ..] => super::t1::n1989(s, out),
						['ы', s @ ..] => super::t1::n1990(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t1::n838(s, out),
				['н', s @ ..] => super::t1::n839(s, out),
				['п', s @ ..] => super::t1::n840(s, out),
				['р', s @ ..] => super::t1::n841(s, out),
				['с', s @ ..] => super::t1::n842(s, out),
				['т', s @ ..] => super::t1::n843(s, out),
				['ф', s @ ..] => super::t1::n844(s, out),
				['ц', s @ ..] => super::t1::n845(s, out),
				['ч', s @ ..] => super::t1::n846(s, out),
				['ш', s @ ..] => super::t1::n847(s, out),
				['ь', s @ ..] => super::t1::n848(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n243(s, out),
		['й', s @ ..] => super::t1::n244(s, out),
		['к', s @ ..] => super::t1::n245(s, out),
		['л', s @ ..] => super::t1::n246(s, out),
		['н', s @ ..] => super::t1::n247(s, out),
		['о', s @ ..] => super::t1::n248(s, out),
		['п', s @ ..] => super::t1::n249(s, out),
		['р', s @ ..] => super::t1::n250(s, out),
		['с', s @ ..] => super::t1::n251(s, out),
		['т', s @ ..] => super::t1::n252(s, out),
		['у', s @ ..] => super::t1::n253(s, out),
		['ф', s @ ..] => super::t1::n254(s, out),
		['х', s @ ..] => super::t1::n255(s, out),
		['ы', s @ ..] => super::t1::n256(s, out),
		['ь', s @ ..] => super::t1::n257(s, out),
		['э', s @ ..] => super::t1::n258(s, out),
		['ю', s @ ..] => super::t1::n259(s, out),
		['я', s @ ..] => super::t1::n260(s, out),
		_ => false,
	}
}

/// Words ending -у.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	s.is_empty()
}

/// Words ending -ф.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['с', 'т', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -х.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 265);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 266);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 267);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ц.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2014);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3378);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3379);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3380);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 5262);
													s.is_empty()
												}
												['г', s @ ..] => {
													super::push(out, 5263);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 5264);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 5265);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 4522);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 4523);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 4524);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 3381);
									match s {
										[] => true,
										['е', 'в', s @ ..] => {
											super::push(out, 4525);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4526);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3382);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3383);
									s.is_empty()
								}
								['р', s @ ..] => super::t1::n3579(s, out),
								['ч', s @ ..] => super::t1::n3580(s, out),
								['щ', s @ ..] => super::t1::n3581(s, out),
								['ь', s @ ..] => super::t1::n3582(s, out),
								_ => false,
							}
						}
						['и', s @ ..] => super::t1::n2106(s, out),
						['о', s @ ..] => super::t1::n2107(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t1::n904(s, out),
				['д', s @ ..] => super::t1::n905(s, out),
				['е', s @ ..] => super::t1::n906(s, out),
				['ж', s @ ..] => super::t1::n907(s, out),
				['з', s @ ..] => super::t1::n908(s, out),
				['и', s @ ..] => super::t1::n909(s, out),
				['л', s @ ..] => super::t1::n910(s, out),
				['м', s @ ..] => super::t1::n911(s, out),
				['н', s @ ..] => super::t1::n912(s, out),
				['р', s @ ..] => super::t1::n913(s, out),
				['с', s @ ..] => super::t1::n914(s, out),
				['т', s @ ..] => super::t1::n915(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ч.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 270);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2036);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 3417);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4559);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 3418);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4561);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 4562);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 5279);
													s.is_empty()
												}
												['м', s @ ..] => {
													super::push(out, 5280);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 5281);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 5282);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 4563);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 3419);
									match s {
										[] => true,
										['о', 'д', s @ ..] => {
											super::push(out, 4564);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 5283);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 5284);
													s.is_empty()
												}
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
		['ы', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ш.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 273);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 901);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2047);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 3420);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2053);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 3422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 3423);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щ.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	s.is_empty()
}

/// Words ending -ы.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3425);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['б', 'ы', 'в', '-', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							super::push(out, 2065);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3427);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3428);
									s.is_empty()
								}
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

/// Words ending -ь.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['е', 'ь', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 918);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 919);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2069);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 2074);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2075);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3430);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 4565);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 5285);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 5286);
													match s {
														[] => true,
														['з', s @ ..] => super::t1::n6026(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 5287);
													match s {
														[] => true,
														['т', s @ ..] => super::t1::n6027(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ж', s @ ..] => {
											super::push(out, 4566);
											match s {
												[] => true,
												['р', 'е', 'д', 'о', 'к', s @ ..] => {
													super::push(out, 5288);
													match s {
														[] => true,
														['т', s @ ..] => super::t1::n6028(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 4567);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 5289);
													match s {
														[] => true,
														['п', 'с', 'и', '-', 'к', s @ ..] => super::t1::n6029(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => super::t1::n3628(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3432);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['н', s @ ..] => super::t1::n301(s, out),
		['п', s @ ..] => super::t1::n302(s, out),
		['р', s @ ..] => super::t1::n303(s, out),
		['с', s @ ..] => super::t1::n304(s, out),
		['т', s @ ..] => super::t1::n305(s, out),
		['ч', s @ ..] => super::t1::n306(s, out),
		['ш', s @ ..] => super::t1::n307(s, out),
		['щ', s @ ..] => super::t1::n308(s, out),
		_ => false,
	}
}

/// Words ending -э.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	s.is_empty()
}

/// Words ending -ю.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['е', 'к', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3450);
									match s {
										[] => true,
										['е', 'щ', s @ ..] => {
											super::push(out, 4652);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'е', 'т', 'х', s @ ..] => {
									super::push(out, 3451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['ж', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4653);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 4654);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 3452);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 3453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', 'е', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 3454);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'е', s @ ..] => {
							super::push(out, 2097);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 3456);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2099);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3458);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 2100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3460);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3461);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 4655);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 4656);
											s.is_empty()
										}
										['ж', s @ ..] => {
											super::push(out, 4657);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 4658);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 4659);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 4660);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 4661);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 4662);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 5382);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', 'и', s @ ..] => {
											super::push(out, 4663);
											s.is_empty()
										}
										['щ', s @ ..] => {
											super::push(out, 4664);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3463);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 4665);
											s.is_empty()
										}
										['м', 'и', s @ ..] => {
											super::push(out, 4666);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 4667);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'е', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 3465);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t1::n2198(s, out),
						['ч', s @ ..] => {
							super::push(out, 2103);
							match s {
								[] => true,
								['е', s @ ..] => super::t1::n3671(s, out),
								['о', s @ ..] => {
									super::push(out, 3470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => super::t1::n2200(s, out),
						['ь', s @ ..] => super::t1::n2201(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t1::n975(s, out),
				['ч', s @ ..] => super::t1::n976(s, out),
				['щ', s @ ..] => super::t1::n977(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t1::n311(s, out),
		['и', s @ ..] => super::t1::n312(s, out),
		['л', s @ ..] => super::t1::n313(s, out),
		['н', s @ ..] => super::t1::n314(s, out),
		['т', s @ ..] => super::t1::n315(s, out),
		['ь', s @ ..] => super::t1::n316(s, out),
		_ => false,
	}
}

/// Words ending -еза.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 993);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		['л', 'у', s @ ..] => {
			super::push(out, 995);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 996);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3486);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 998);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ка.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2139);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2140);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 2141);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', 'ы', s @ ..] => {
									super::push(out, 3488);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2146);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 3490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3491);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 3492);
									s.is_empty()
								}
								['д', s @ ..] => super::t1::n3695(s, out),
								['з', s @ ..] => super::t1::n3696(s, out),
								['о', 'н', 'е', 'л', 'ч', '\u{1}', s @ ..] => super::t1::n3697(s, out),
								['р', s @ ..] => super::t1::n3698(s, out),
								_ => false,
							}
						}
						['е', s @ ..] => super::t1::n2248(s, out),
						['ж', s @ ..] => super::t1::n2249(s, out),
						['з', 'я', 'р', s @ ..] => super::t1::n2250(s, out),
						['и', s @ ..] => super::t1::n2251(s, out),
						['л', s @ ..] => super::t1::n2252(s, out),
						['н', s @ ..] => super::t1::n2253(s, out),
						['о', s @ ..] => super::t1::n2254(s, out),
						['п', s @ ..] => super::t1::n2255(s, out),
						['р', s @ ..] => super::t1::n2256(s, out),
						['с', s @ ..] => super::t1::n2257(s, out),
						['т', s @ ..] => super::t1::n2258(s, out),
						['у', s @ ..] => super::t1::n2259(s, out),
						['ц', s @ ..] => super::t1::n2260(s, out),
						['ч', s @ ..] => super::t1::n2261(s, out),
						['ш', s @ ..] => super::t1::n2262(s, out),
						['щ', s @ ..] => super::t1::n2263(s, out),
						['ь', s @ ..] => super::t1::n2264(s, out),
						['ю', s @ ..] => super::t1::n2265(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t1::n1040(s, out),
				['л', s @ ..] => super::t1::n1041(s, out),
				['о', s @ ..] => super::t1::n1042(s, out),
				['у', s @ ..] => super::t1::n1043(s, out),
				['ы', s @ ..] => super::t1::n1044(s, out),
				['я', s @ ..] => super::t1::n1045(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t1::n355(s, out),
		['е', s @ ..] => super::t1::n356(s, out),
		['ж', s @ ..] => super::t1::n357(s, out),
		['з', s @ ..] => super::t1::n358(s, out),
		['и', s @ ..] => super::t1::n359(s, out),
		['й', s @ ..] => super::t1::n360(s, out),
		['к', s @ ..] => super::t1::n361(s, out),
		['л', s @ ..] => super::t1::n362(s, out),
		['м', s @ ..] => super::t1::n363(s, out),
		['н', s @ ..] => super::t1::n364(s, out),
		['о', s @ ..] => super::t1::n365(s, out),
		['п', s @ ..] => super::t1::n366(s, out),
		['р', s @ ..] => super::t1::n367(s, out),
		['с', s @ ..] => super::t1::n368(s, out),
		['т', s @ ..] => super::t1::n369(s, out),
		['у', s @ ..] => super::t1::n370(s, out),
		['ф', s @ ..] => super::t1::n371(s, out),
		['ц', s @ ..] => super::t1::n372(s, out),
		['ч', s @ ..] => super::t1::n373(s, out),
		['ш', s @ ..] => super::t1::n374(s, out),
		['ы', s @ ..] => super::t1::n375(s, out),
		['ь', s @ ..] => super::t1::n376(s, out),
		['ю', s @ ..] => super::t1::n377(s, out),
		['я', s @ ..] => super::t1::n378(s, out),
		_ => false,
	}
}

/// Words ending -ла.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['д', 'р', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ма.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2350);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 3838);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2353);
							match s {
								[] => true,
								['к', 'и', s @ ..] => {
									super::push(out, 3840);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3841);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 2357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -на.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 2360);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3843);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 2361);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2362);
							match s {
								[] => true,
								['о', 'д', s @ ..] => {
									super::push(out, 3846);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4890);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4891);
											s.is_empty()
										}
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
		['е', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2365);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						['и', 'м', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						['с', 'а', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2375);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 3848);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3849);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 4892);
											s.is_empty()
										}
										['е', 'ч', s @ ..] => {
											super::push(out, 4893);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 3851);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 3852);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 3853);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 3854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2381);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 2382);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3856);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['с', 'е', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['е', s @ ..] => super::t1::n2516(s, out),
						_ => false,
					}
				}
				['ч', 'е', s @ ..] => super::t1::n1186(s, out),
				['ш', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				['щ', s @ ..] => super::t1::n1188(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['о', s @ ..] => super::t1::n400(s, out),
		['р', s @ ..] => super::t1::n401(s, out),
		['у', s @ ..] => super::t1::n402(s, out),
		_ => false,
	}
}

/// Words ending -па.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 2394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ра.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 406);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['н', 'е', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2396);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2398);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3865);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'м', s @ ..] => {
							super::push(out, 2399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 1164);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 410);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2401);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 2402);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3867);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 4904);
											s.is_empty()
										}
										['ф', '-', s @ ..] => {
											super::push(out, 4905);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 3868);
									s.is_empty()
								}
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

/// Words ending -еса.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1167);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сса.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	s.is_empty()
}

/// Words ending -та.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2403);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 3869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3870);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уа.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	s.is_empty()
}

/// Words ending -фа.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -ха.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['л', 'о', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2406);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2408);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							super::push(out, 2409);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2411);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 2412);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2413);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2414);
							s.is_empty()
						}
						['у', 'ш', s @ ..] => {
							super::push(out, 2415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 420);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2416);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 3872);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3873);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 3874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ца.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['о', 'з', s @ ..] => {
							super::push(out, 2417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['н', s @ ..] => super::t1::n1224(s, out),
				['п', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'е', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2436);
							match s {
								[] => true,
								['е', s @ ..] => super::t1::n4124(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t1::n2569(s, out),
						['с', s @ ..] => super::t1::n2570(s, out),
						['т', s @ ..] => super::t1::n2571(s, out),
						_ => false,
					}
				}
				['щ', s @ ..] => super::t1::n1230(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t1::n426(s, out),
		_ => false,
	}
}

/// Words ending -ча.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	s.is_empty()
}

/// Words ending -ша.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['т', 'а', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2448);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2449);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', 'я', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2451);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3927);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3928);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3929);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 3930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3931);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3932);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 3933);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3934);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 2452);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3936);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 3937);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 3938);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3939);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3940);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3941);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3942);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 3943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'и', 'р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3944);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['р', 'п', s @ ..] => {
											super::push(out, 4960);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', 'а', 'б', s @ ..] => {
									super::push(out, 3946);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3947);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'а', 'т', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 3949);
									match s {
										[] => true,
										['о', 'б', 'к', s @ ..] => {
											super::push(out, 4962);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 3950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3951);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3952);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 3953);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 3954);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3955);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3956);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3957);
									s.is_empty()
								}
								['ф', 'и', s @ ..] => {
									super::push(out, 3958);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 435);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ща.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йтсерфинг.
pub(crate) fn n5830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5497);
	s.is_empty()
}

/// Words ending -ртсерфинг.
pub(crate) fn n5831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5498);
	s.is_empty()
}

/// Words ending -рг.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['б', 'е', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ие.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 499);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2497);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2498);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t1::n510(s, out),
		['р', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1279);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['л', 'о', 'г', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4001);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4002);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5013);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5014);
											s.is_empty()
										}
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
		['т', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4003);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4004);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4005);
									match s {
										[] => true,
										['о', 'с', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 5565);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 5566);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 4006);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5015);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1282);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2531);
							match s {
								[] => true,
								['е', 'п', 'х', s @ ..] => {
									super::push(out, 4007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['т', 'с', 'х', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ые.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1311);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2552);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 2553);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2554);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4013);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 5019);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 5020);
											s.is_empty()
										}
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
		['л', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1315);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['л', 'е', 'з', '-', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4014);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2557);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4016);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 4017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['е', 'в', 'ц', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4018);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ье.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 4020);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2575);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 5021);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 5022);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'у', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2577);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 4022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2578);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2579);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						['о', 'в', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 4023);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4025);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2588);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4028);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4030);
									s.is_empty()
								}
								['о', 'т', s @ ..] => {
									super::push(out, 4031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1340);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						['м', s @ ..] => super::t1::n2743(s, out),
						['т', s @ ..] => super::t1::n2744(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t1::n1386(s, out),
				['о', s @ ..] => super::t1::n1387(s, out),
				['у', s @ ..] => super::t1::n1388(s, out),
				['ы', s @ ..] => super::t1::n1389(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t1::n546(s, out),
		['н', s @ ..] => super::t1::n547(s, out),
		['п', s @ ..] => super::t1::n548(s, out),
		['р', s @ ..] => super::t1::n549(s, out),
		['с', s @ ..] => super::t1::n550(s, out),
		['т', s @ ..] => super::t1::n551(s, out),
		['ч', s @ ..] => super::t1::n552(s, out),
		['ш', 'у', s @ ..] => super::t1::n553(s, out),
		_ => false,
	}
}

/// Words ending -сти.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1436);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 2734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				['к', 'м', s @ ..] => {
					super::push(out, 2736);
					s.is_empty()
				}
				['н', 'е', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емовский.
pub(crate) fn n5909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5573);
	s.is_empty()
}

/// Words ending -имовский.
pub(crate) fn n5910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5574);
	s.is_empty()
}

/// Words ending -лый.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1449);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2746);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мый.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	s.is_empty()
}

/// Words ending -ный.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1451);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2748);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2749);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1452);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2751);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1453);
			s.is_empty()
		}
		['з', 'е', s @ ..] => {
			super::push(out, 1454);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 1455);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1456);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2753);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2755);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4106);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4107);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4108);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', 'ю', 'л', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 5575);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 5576);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 5049);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 5050);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5051);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5052);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 5053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 4109);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4110);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4111);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4112);
							s.is_empty()
						}
						['ч', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5054);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5055);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 5056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 4113);
							s.is_empty()
						}
						['щ', s @ ..] => {
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 5057);
									s.is_empty()
								}
								['я', 'в', 'с', 'о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5577);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 5578);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2759);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2760);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4114);
							s.is_empty()
						}
						['в', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 5579);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 5580);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 5581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 4115);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2762);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2763);
					s.is_empty()
				}
				['с', 'е', 'р', s @ ..] => {
					super::push(out, 2764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					super::push(out, 2765);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4117);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рый.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	s.is_empty()
}

/// Words ending -тый.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1461);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['с', 'х', s @ ..] => {
					super::push(out, 2766);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4119);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яй.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	s.is_empty()
}

/// Words ending -оек.
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1484);
			s.is_empty()
		}
		['л', 'с', 'о', s @ ..] => {
			super::push(out, 1485);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1486);
			s.is_empty()
		}
		['р', 'т', s @ ..] => {
			super::push(out, 1487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пек.
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1488);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1489);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1490);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1491);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1492);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рек.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1494);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1495);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 1496);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1497);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2772);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2773);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2774);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1498);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1499);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1500);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1501);
			match s {
				[] => true,
				['-', 'т', s @ ..] => {
					super::push(out, 2776);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5059);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1502);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1503);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1504);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сек.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2777);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тек.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1511);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1513);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1514);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2784);
					s.is_empty()
				}
				['п', 'а', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1517);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 1518);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1519);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1520);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 1521);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чек.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1524);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2788);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2791);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2792);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2793);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2794);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2795);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2796);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1525);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 4121);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5061);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 5582);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 5583);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 4122);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4123);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 4124);
							match s {
								[] => true,
								['л', 'о', 'г', s @ ..] => {
									super::push(out, 5063);
									match s {
										[] => true,
										['у', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 5857);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 5858);
													s.is_empty()
												}
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
		['о', s @ ..] => {
			super::push(out, 1526);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2798);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2799);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2800);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2801);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				['л', 'е', s @ ..] => {
					super::push(out, 2803);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4125);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 4126);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4127);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4128);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2804);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4130);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 4131);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5065);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 5066);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5584);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 5585);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 5067);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5068);
									s.is_empty()
								}
								['т', 'т', s @ ..] => {
									super::push(out, 5069);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 5070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4132);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4133);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4134);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4135);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 4136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2807);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2809);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4137);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5071);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4138);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4139);
							s.is_empty()
						}
						['с', 'р', s @ ..] => {
							super::push(out, 4140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шек.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1527);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1528);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['б', 'е', s @ ..] => {
					super::push(out, 2812);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2813);
					s.is_empty()
				}
				['м', 'е', s @ ..] => {
					super::push(out, 2814);
					s.is_empty()
				}
				['н', 'е', s @ ..] => {
					super::push(out, 2815);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ик.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2817);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4141);
									match s {
										[] => true,
										['у', 'р', 'х', s @ ..] => {
											super::push(out, 5073);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 5586);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 5587);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 4142);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 2818);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 2819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2820);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4144);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4145);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 2822);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['н', s @ ..] => super::t1::n648(s, out),
		['п', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1558);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							super::push(out, 2874);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1561);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 2879);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2880);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4218);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4219);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 5128);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 5605);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 5606);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', 'е', s @ ..] => {
									super::push(out, 4220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4221);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2885);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 4223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['ч', s @ ..] => super::t1::n654(s, out),
		['ш', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => super::t1::n656(s, out),
		_ => false,
	}
}

/// Words ending -йк.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	s.is_empty()
}

/// Words ending -кк.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	s.is_empty()
}

/// Words ending -лк.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['щ', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нк.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	s.is_empty()
}

/// Words ending -ок.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4294);
									s.is_empty()
								}
								['к', 'с', 'о', s @ ..] => {
									super::push(out, 4295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 2945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						['л', 'а', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4296);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 4297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 649);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 650);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2955);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2956);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5182);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 5183);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 5184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 4298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4299);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 2957);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 4301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2958);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t1::n1662(s, out),
				['и', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1607);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['л', 'ш', s @ ..] => {
							super::push(out, 2971);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 4327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['-', 'ф', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2973);
							s.is_empty()
						}
						['л', 'п', s @ ..] => {
							super::push(out, 2974);
							s.is_empty()
						}
						['п', 'о', s @ ..] => {
							super::push(out, 2975);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4328);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t1::n1674(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t1::n671(s, out),
		['ч', s @ ..] => super::t1::n672(s, out),
		['ш', s @ ..] => super::t1::n673(s, out),
		_ => false,
	}
}

/// Words ending -рк.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['в', 'т', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ск.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2985);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4341);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 4342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 2987);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4343);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'н', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 664);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['и', 'м', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2992);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 5203);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 5204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ук.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	s.is_empty()
}

/// Words ending -цк.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	s.is_empty()
}

/// Words ending -ык.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	s.is_empty()
}

/// Words ending -эк.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	s.is_empty()
}

/// Words ending -юк.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	s.is_empty()
}

/// Words ending -як.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	s.is_empty()
}

/// Words ending -ун.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 728);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьн.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', 'к', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эн.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тво.
pub(crate) fn n752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3036);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4358);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5209);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5210);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 5211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3038);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3039);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						['ч', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5212);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 5638);
											match s {
												[] => true,
												['т', 'о', 'к', s @ ..] => {
													super::push(out, 5870);
													match s {
														[] => true,
														['л', s @ ..] => super::t1::n6427(s, out),
														['п', s @ ..] => super::t1::n6428(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 5213);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 5871);
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n6429(s, out),
														['о', s @ ..] => super::t1::n6430(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 5872);
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n6431(s, out),
														_ => false,
													}
												}
												['т', 'е', s @ ..] => {
													super::push(out, 5873);
													match s {
														[] => true,
														['ч', s @ ..] => super::t1::n6432(s, out),
														_ => false,
													}
												}
												['ч', s @ ..] => super::t1::n6226(s, out),
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
				['й', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3041);
					s.is_empty()
				}
				['н', s @ ..] => super::t1::n3212(s, out),
				['п', 'е', s @ ..] => super::t1::n3213(s, out),
				['р', s @ ..] => super::t1::n3214(s, out),
				['т', s @ ..] => super::t1::n3215(s, out),
				['ь', s @ ..] => super::t1::n3216(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ко.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 739);
			s.is_empty()
		}
		['и', 'с', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 742);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 744);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['е', 'д', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 746);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 747);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1715);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 3051);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['у', 'н', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4377);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 4378);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4379);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4380);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5230);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 5231);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 4381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4382);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4383);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 4384);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4385);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4386);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3055);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5232);
											s.is_empty()
										}
										['я', s @ ..] => {
											super::push(out, 5233);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 4388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -но.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3056);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4389);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 4390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'е', s @ ..] => {
							super::push(out, 3057);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4391);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 4392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -со.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	s.is_empty()
}

/// Words ending -то.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['н', 'е', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хо.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	s.is_empty()
}

/// Words ending -цо.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	s.is_empty()
}

