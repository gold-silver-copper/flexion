use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => super::t0::n1(s, out),
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['m', '\u{11}', 'i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', '\u{12}', 'h', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['r', '\u{11}', 'a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 85);
					match s {
						[] => true,
						['\u{12}', 'p', 'p', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{12}', 'e', 'r', 'ë', 'b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 86);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 87);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 88);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 89);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 90);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 91);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 92);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 93);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 94);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', 'h', 'c', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 573);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 574);
									match s {
										[] => true,
										['p', 'o', s @ ..] => {
											super::push(out, 1296);
											s.is_empty()
										}
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
					super::push(out, 96);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 97);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 98);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 99);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 100);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 101);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 219);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', 'i', 's', 's', 'o', 'r', s @ ..] => {
											super::push(out, 1297);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1298);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 575);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 576);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 577);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 1299);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 578);
									match s {
										[] => true,
										['a', 's', s @ ..] => {
											super::push(out, 1300);
											s.is_empty()
										}
										['e', 'c', s @ ..] => {
											super::push(out, 1301);
											s.is_empty()
										}
										['o', 'f', s @ ..] => {
											super::push(out, 1302);
											match s {
												[] => true,
												['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 2216);
													s.is_empty()
												}
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
					super::push(out, 102);
					match s {
						[] => true,
						['\u{13}', 's', s @ ..] => {
							super::push(out, 220);
							match s {
								[] => true,
								['i', 'r', 'c', 'a', 'n', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 103);
					match s {
						[] => true,
						['\u{13}', 'a', 't', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', '\u{13}', s @ ..] => {
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
		['k', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['a', '\u{12}', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'b', '-', 'e', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 222);
							s.is_empty()
						}
						['l', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11);
			match s {
				[] => true,
				['i', '\u{12}', 'a', 'm', s @ ..] => {
					super::push(out, 106);
					s.is_empty()
				}
				['l', '\u{13}', 'e', 'r', 'a', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 107);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 108);
					match s {
						[] => true,
						['\u{12}', 'c', 'l', 'a', 'o', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['a', '\u{12}', 'c', s @ ..] => {
					super::push(out, 110);
					s.is_empty()
				}
				['u', '\u{12}', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 112);
					match s {
						[] => true,
						['\u{11}', 'f', '\u{1}', s @ ..] => {
							super::push(out, 225);
							s.is_empty()
						}
						['\u{13}', 'l', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['\u{11}', 't', 'y', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 227);
							s.is_empty()
						}
						['\u{12}', 'o', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 228);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 580);
									s.is_empty()
								}
								['l', 'i', s @ ..] => {
									super::push(out, 581);
									s.is_empty()
								}
								['r', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 582);
									s.is_empty()
								}
								['t', 'a', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n14(s, out),
		['p', s @ ..] => {
			super::push(out, 15);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 139);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							match s {
								[] => true,
								['h', 's', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 732);
									s.is_empty()
								}
								['l', 'i', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['d', 'i', 's', 't', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 734);
									s.is_empty()
								}
								['g', 'a', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 735);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1553);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 1554);
											s.is_empty()
										}
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
		['s', s @ ..] => super::t0::n17(s, out),
		['t', s @ ..] => super::t0::n18(s, out),
		['u', s @ ..] => {
			super::push(out, 19);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 20);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 21);
			s.is_empty()
		}
		['x', s @ ..] => super::t0::n22(s, out),
		['y', s @ ..] => super::t0::n23(s, out),
		['z', s @ ..] => super::t0::n24(s, out),
		['à', s @ ..] => super::t0::n25(s, out),
		['è', s @ ..] => super::t0::n26(s, out),
		['é', s @ ..] => super::t0::n27(s, out),
		['ì', s @ ..] => super::t0::n28(s, out),
		['ò', s @ ..] => super::t0::n29(s, out),
		['ù', s @ ..] => super::t0::n30(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 31);
			s.is_empty()
		}
		['a', '\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 32);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 737);
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 261);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 263);
							s.is_empty()
						}
						['m', 'o', 't', 'e', 'r', 't', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 264);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['d', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
													super::push(out, 2501);
													s.is_empty()
												}
												['v', 'l', 'a', 's', '\u{1}', s @ ..] => {
													super::push(out, 2502);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'c', 's', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1557);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', 'k', 'a', 'y', '\u{1}', s @ ..] => {
									super::push(out, 739);
									s.is_empty()
								}
								['t', 'i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 265);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1558);
											s.is_empty()
										}
										['r', 'p', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 1559);
											s.is_empty()
										}
										['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 's', 'i', 'u', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 742);
									s.is_empty()
								}
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 743);
									s.is_empty()
								}
								['t', 'o', 's', 's', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n34(s, out),
		['d', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 159);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 764);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'm', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 277);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 278);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 766);
									s.is_empty()
								}
								['i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 279);
							s.is_empty()
						}
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 280);
							s.is_empty()
						}
						['u', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => super::t0::n343(s, out),
						['i', s @ ..] => super::t0::n344(s, out),
						['l', s @ ..] => {
							super::push(out, 284);
							match s {
								[] => true,
								['a', 'c', 'a', 't', 's', 'e', 't', '\u{1}', s @ ..] => super::t0::n937(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n346(s, out),
						['o', s @ ..] => super::t0::n347(s, out),
						['r', s @ ..] => super::t0::n348(s, out),
						['u', 'c', 'a', 'r', 'r', 'a', 'b', '\u{1}', s @ ..] => super::t0::n349(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n36(s, out),
		['f', s @ ..] => super::t0::n37(s, out),
		['g', s @ ..] => super::t0::n38(s, out),
		['h', s @ ..] => super::t0::n39(s, out),
		['i', s @ ..] => super::t0::n40(s, out),
		['j', s @ ..] => super::t0::n41(s, out),
		['k', s @ ..] => super::t0::n42(s, out),
		['l', s @ ..] => super::t0::n43(s, out),
		['m', s @ ..] => super::t0::n44(s, out),
		['n', s @ ..] => super::t0::n45(s, out),
		['o', s @ ..] => super::t0::n46(s, out),
		['p', s @ ..] => super::t0::n47(s, out),
		['r', s @ ..] => super::t0::n48(s, out),
		['s', s @ ..] => super::t0::n49(s, out),
		['t', s @ ..] => super::t0::n50(s, out),
		['u', s @ ..] => super::t0::n51(s, out),
		['v', s @ ..] => super::t0::n52(s, out),
		['w', s @ ..] => super::t0::n53(s, out),
		['y', '\u{13}', 'a', 'b', 'a', '\u{1}', s @ ..] => super::t0::n54(s, out),
		['z', s @ ..] => super::t0::n55(s, out),
		['à', '\u{11}', '\u{1}', s @ ..] => super::t0::n56(s, out),
		['è', '\u{11}', 'd', '\u{1}', s @ ..] => super::t0::n57(s, out),
		['ì', s @ ..] => super::t0::n58(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['\u{11}', '\u{1}', s @ ..] => {
					super::push(out, 190);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 443);
							match s {
								[] => true,
								['o', 't', 'a', 'c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'a', 'v', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 444);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['\u{11}', '\u{1}', s @ ..] => {
					super::push(out, 191);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 192);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 446);
							s.is_empty()
						}
						['c', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 447);
							s.is_empty()
						}
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 448);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 193);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 450);
							match s {
								[] => true,
								['l', 'p', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 1071);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1072);
									match s {
										[] => true,
										['b', 'a', 'r', s @ ..] => {
											super::push(out, 1978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 451);
							match s {
								[] => true,
								['e', 'r', 't', 'a', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 452);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 453);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1074);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 1075);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 1076);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1077);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 1979);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 1078);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1079);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1980);
											s.is_empty()
										}
										['t', 's', s @ ..] => {
											super::push(out, 1981);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1080);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1081);
									match s {
										[] => true,
										['r', 'e', 's', '-', 'f', 'l', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 1982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 454);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'i', 'n', 'g', 'e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 1983);
											s.is_empty()
										}
										['f', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 455);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', 'p', 'é', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1985);
											s.is_empty()
										}
										['v', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 456);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 1987);
											s.is_empty()
										}
										['v', 'e', 'i', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['c', 's', 'e', 'p', '\u{1}', s @ ..] => {
													super::push(out, 2792);
													s.is_empty()
												}
												['r', 'r', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 2793);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'c', 's', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1989);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'c', s @ ..] => {
									match s {
										[] => true,
										['a', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 1990);
											s.is_empty()
										}
										['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1991);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['t', s @ ..] => super::t0::n3475(s, out),
												['v', 'i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 2794);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 1992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t0::n545(s, out),
						['u', s @ ..] => super::t0::n546(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n65(s, out),
		['e', s @ ..] => super::t0::n66(s, out),
		['f', s @ ..] => super::t0::n67(s, out),
		['g', s @ ..] => super::t0::n68(s, out),
		['h', s @ ..] => super::t0::n69(s, out),
		['i', s @ ..] => super::t0::n70(s, out),
		['k', s @ ..] => super::t0::n71(s, out),
		['l', s @ ..] => super::t0::n72(s, out),
		['m', s @ ..] => super::t0::n73(s, out),
		['n', s @ ..] => super::t0::n74(s, out),
		['o', s @ ..] => super::t0::n75(s, out),
		['p', s @ ..] => super::t0::n76(s, out),
		['r', s @ ..] => super::t0::n77(s, out),
		['s', s @ ..] => super::t0::n78(s, out),
		['t', s @ ..] => super::t0::n79(s, out),
		['u', s @ ..] => super::t0::n80(s, out),
		['v', s @ ..] => super::t0::n81(s, out),
		['x', s @ ..] => super::t0::n82(s, out),
		['z', s @ ..] => super::t0::n83(s, out),
		['é', s @ ..] => super::t0::n84(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['\u{12}', 'n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 229);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						['n', 'g', 'i', 'r', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 2217);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 2218);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['e', 'c', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 588);
							s.is_empty()
						}
						['m', 'o', 'i', 'p', 'i', 't', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						['o', 'n', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 230);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1304);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 591);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1306);
									s.is_empty()
								}
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 592);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1309);
									s.is_empty()
								}
								['o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1310);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 593);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 594);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1312);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t0::n269(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n120(s, out),
		['e', s @ ..] => super::t0::n121(s, out),
		['f', s @ ..] => super::t0::n122(s, out),
		['g', s @ ..] => super::t0::n123(s, out),
		['h', s @ ..] => super::t0::n124(s, out),
		['i', s @ ..] => super::t0::n125(s, out),
		['j', '\u{12}', 'n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['k', '\u{13}', 'i', 's', 'i', 'r', '\u{1}', s @ ..] => super::t0::n127(s, out),
		['l', s @ ..] => super::t0::n128(s, out),
		['m', s @ ..] => super::t0::n129(s, out),
		['n', s @ ..] => super::t0::n130(s, out),
		['o', s @ ..] => super::t0::n131(s, out),
		['p', s @ ..] => super::t0::n132(s, out),
		['r', s @ ..] => super::t0::n133(s, out),
		['s', s @ ..] => super::t0::n134(s, out),
		['t', s @ ..] => super::t0::n135(s, out),
		['u', s @ ..] => super::t0::n136(s, out),
		['v', s @ ..] => super::t0::n137(s, out),
		['y', '\u{12}', '-', 'o', 'y', '\u{1}', s @ ..] => super::t0::n138(s, out),
		['z', s @ ..] => super::t0::n139(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['o', '\u{12}', 'p', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['u', '\u{12}', s @ ..] => {
			match s {
				[] => true,
				['c', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				['n', 'l', 'u', 'v', '\u{1}', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['\u{12}', 'l', 'r', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['\u{13}', 'n', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', '\u{12}', 'a', 's', 's', 'i', 'o', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', '\u{12}', 'n', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 143);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['o', '\u{11}', 'b', '\u{1}', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['t', '\u{12}', 'i', 'l', 'a', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	s.is_empty()
}

/// Words ending -à.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', 'i', 'b', 'i', 's', 'i', 'v', 'i', 'd', 'n', s @ ..] => {
									super::push(out, 1555);
									s.is_empty()
								}
								['r', 'e', 'v', '-', s @ ..] => {
									super::push(out, 1556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -è.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	s.is_empty()
}

/// Words ending -ì.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	s.is_empty()
}

/// Words ending -ò.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	s.is_empty()
}

/// Words ending -ù.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['b', '\u{13}', 'i', 'r', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ca.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1561);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						['i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 266);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 749);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 268);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['i', 'n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['n', 'o', 'r', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						['p', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 755);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								['b', 'a', 's', 'l', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 269);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['f', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2503);
											s.is_empty()
										}
										['g', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 2504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => super::t0::n1983(s, out),
								['s', s @ ..] => super::t0::n1984(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t0::n905(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n331(s, out),
				['n', s @ ..] => super::t0::n332(s, out),
				['o', 'i', s @ ..] => super::t0::n333(s, out),
				['r', s @ ..] => super::t0::n334(s, out),
				['s', s @ ..] => super::t0::n335(s, out),
				['u', 'd', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ada (3 syllables).
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		['e', 'l', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		['m', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 770);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['m', 'a', 'l', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						['e', 'c', 's', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 't', 's', 'i', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ida (3 syllables).
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['c', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'a', 'm', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['m', 'r', s @ ..] => {
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 3101);
									s.is_empty()
								}
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						['t', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'u', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['m', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		['r', 'b', 'a', 'n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 775);
			s.is_empty()
		}
		['t', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['g', 'a', 'l', 'o', 'u', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word testacalda (3 syllables).
pub(crate) fn n937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 779);
	s.is_empty()
}

/// Words ending -nda (3 syllables).
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				['c', 'r', 'a', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['w', 'r', 'a', 'y', 'n', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'm', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 780);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				['r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', 'i', 'r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oda (3 syllables).
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 782);
			s.is_empty()
		}
		['t', 'a', 'm', 'e', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['v', 'i', s @ ..] => {
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rda (3 syllables).
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['a', 'i', 'l', 'g', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 784);
			s.is_empty()
		}
		['e', 'm', 'i', 'g', 'n', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 785);
			s.is_empty()
		}
		['o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 786);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word barracuda (3 syllables).
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	s.is_empty()
}

/// Words ending -ea.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 'r', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', 'l', s @ ..] => {
					super::push(out, 291);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'o', 'c', 'a', 'm', 'r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				['r', 'a', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['f', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['f', 'o', 't', 's', 'i', 'm', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 296);
					s.is_empty()
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['l', 'a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['n', 'i', 'l', 'o', 'd', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 787);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1608);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'y', '\u{1}', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 790);
							s.is_empty()
						}
						['p', 'l', s @ ..] => {
							super::push(out, 791);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2540);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 2541);
											s.is_empty()
										}
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
					super::push(out, 303);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', 't', 'e', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1611);
									s.is_empty()
								}
								['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1612);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 792);
							s.is_empty()
						}
						['s', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 793);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 1613);
									s.is_empty()
								}
								['t', 'o', 'b', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['e', 'i', 'l', 'g', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 794);
							s.is_empty()
						}
						['i', 's', 'a', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 795);
							s.is_empty()
						}
						['o', 'l', 'a', 'i', 'c', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 796);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 797);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									match s {
										[] => true,
										['b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2542);
											s.is_empty()
										}
										['p', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 't', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 798);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 799);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['i', 'c', 'a', 'l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2544);
											s.is_empty()
										}
										['r', 'd', 'i', '\u{1}', s @ ..] => {
											super::push(out, 2545);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 800);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 307);
					match s {
						[] => true,
						['f', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 802);
							s.is_empty()
						}
						['i', 'c', 's', s @ ..] => {
							super::push(out, 803);
							s.is_empty()
						}
						['s', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ha.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['\u{12}', 's', 'i', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['\u{13}', 'c', 'a', 'h', 'c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 312);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1616);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1618);
									s.is_empty()
								}
								['u', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1620);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 805);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1622);
									s.is_empty()
								}
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 1623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1624);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1626);
									s.is_empty()
								}
								['u', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 315);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1628);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2546);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2547);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 806);
							s.is_empty()
						}
						['g', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1629);
									s.is_empty()
								}
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1631);
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 317);
					match s {
						[] => true,
						['g', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 811);
							s.is_empty()
						}
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['o', 'b', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 1633);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1634);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'n', 'a', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t0::n178(s, out),
		_ => false,
	}
}

/// Words ending -ja.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	s.is_empty()
}

/// Words ending -ka.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', 'z', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['i', 'g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['o', 'd', 'n', 'o', 'w', 'k', 'e', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				['s', 'o', 'i', 'r', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['u', 'o', 'b', 'a', 'r', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['g', 'n', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['r', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['b', 'a', 'b', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						['c', 's', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1690);
									s.is_empty()
								}
								['t', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						['p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 858);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1692);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['n', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2609);
											s.is_empty()
										}
										['p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2610);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 't', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['d', 'n', 'a', 'c', 'a', 'z', 'r', 'o', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						['t', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						['z', 'u', 'v', 'u', 'v', '\u{1}', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['f', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						['m', 'o', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						['u', 'q', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['a', s @ ..] => super::t0::n1062(s, out),
						['e', s @ ..] => super::t0::n1063(s, out),
						['i', s @ ..] => super::t0::n1064(s, out),
						['o', s @ ..] => super::t0::n1065(s, out),
						['u', 'n', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1708);
									s.is_empty()
								}
								['o', 'n', '\u{1}', s @ ..] => super::t0::n2174(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n422(s, out),
				['u', s @ ..] => super::t1::n423(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1720);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 359);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1722);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 2631);
											s.is_empty()
										}
										['g', '\u{1}', s @ ..] => {
											super::push(out, 2632);
											s.is_empty()
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
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1723);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 1724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1725);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1727);
									s.is_empty()
								}
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1728);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1730);
									s.is_empty()
								}
								['h', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1731);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 1732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1733);
									s.is_empty()
								}
								['o', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1734);
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 361);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1736);
									s.is_empty()
								}
								['i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 892);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 893);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t1::n185(s, out),
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 923);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['w', 'b', '\u{1}', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['i', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 376);
					match s {
						[] => true,
						['b', 'b', s @ ..] => {
							super::push(out, 926);
							match s {
								[] => true,
								['a', 'g', 'a', 't', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 927);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1780);
									s.is_empty()
								}
								['r', 'o', 'm', 'a', 't', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1781);
									s.is_empty()
								}
								['u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 928);
							match s {
								[] => true,
								['t', 's', 'i', 'r', 'c', 'a', 'n', 'o', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1784);
									match s {
										[] => true,
										['n', 'o', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2664);
											s.is_empty()
										}
										['z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2665);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 't', 't', 'e', 's', s @ ..] => {
							match s {
								[] => true,
								['-', 'e', 'n', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								['e', 'n', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', 'u', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						['s', 'a', 'c', 'c', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						['t', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								['r', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', 'c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2666);
											s.is_empty()
										}
										['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2667);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'h', s @ ..] => {
							super::push(out, 932);
							s.is_empty()
						}
						['m', 's', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['t', 'a', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1792);
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
						['a', s @ ..] => {
							match s {
								[] => true,
								['p', 's', 'i', 'd', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								['t', 'n', 'o', 'm', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1795);
									s.is_empty()
								}
								['s', 'n', 'i', s @ ..] => {
									match s {
										[] => true,
										['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2668);
											s.is_empty()
										}
										['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2669);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'g', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => super::t1::n2283(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t1::n1166(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n451(s, out),
				['n', s @ ..] => super::t1::n452(s, out),
				['o', s @ ..] => super::t1::n453(s, out),
				['r', 'e', 'c', 'n', 'i', 'p', '\u{1}', s @ ..] => super::t1::n454(s, out),
				['u', s @ ..] => super::t1::n455(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oa.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						['r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						['k', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 387);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['\u{11}', 's', '\u{1}', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 950);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									super::push(out, 1825);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 1826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 952);
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
				['a', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['l', 'a', 't', 'o', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['o', 'c', 's', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						['t', 's', 'i', 'm', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['ù', 'z', 'z', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['f', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 955);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				['k', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				['p', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 957);
							s.is_empty()
						}
						['u', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'o', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 959);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1827);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 960);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1829);
									s.is_empty()
								}
								['r', 'a', 'b', 'a', 't', 'n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 1830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 's', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 962);
							s.is_empty()
						}
						['f', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 963);
							s.is_empty()
						}
						['g', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 964);
							s.is_empty()
						}
						['m', 'y', 'a', '\u{1}', s @ ..] => {
							super::push(out, 965);
							s.is_empty()
						}
						['n', 'i', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 966);
							s.is_empty()
						}
						['t', 'a', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'a', 'c', 'a', 'r', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['b', 'a', 'l', 'a', 'd', 'i', 'c', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['a', 'u', 'q', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 971);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 972);
							match s {
								[] => true,
								['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1831);
									s.is_empty()
								}
								['i', 'd', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1832);
									s.is_empty()
								}
								['l', 'u', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						['h', 'c', 's', 'a', s @ ..] => {
							super::push(out, 974);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 1834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 975);
							match s {
								[] => true,
								['c', 'l', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1835);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 1836);
									match s {
										[] => true,
										['n', 'a', 'b', 'a', s @ ..] => {
											super::push(out, 2684);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 't', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1837);
									s.is_empty()
								}
								['m', 'l', 'a', 'c', 's', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1838);
									s.is_empty()
								}
								['t', s @ ..] => super::t1::n2332(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t1::n1221(s, out),
						['n', s @ ..] => super::t1::n1222(s, out),
						['p', s @ ..] => super::t1::n1223(s, out),
						['s', 's', 'e', 't', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => super::t1::n1224(s, out),
						['t', 't', 'e', s @ ..] => super::t1::n1225(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n484(s, out),
				['o', s @ ..] => super::t1::n485(s, out),
				['p', s @ ..] => super::t1::n486(s, out),
				['r', s @ ..] => super::t1::n487(s, out),
				['t', s @ ..] => super::t1::n488(s, out),
				['u', s @ ..] => super::t1::n489(s, out),
				['v', s @ ..] => super::t1::n490(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 993);
							s.is_empty()
						}
						['r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'p', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 411);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', 'i', 'n', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						['l', 'u', 'l', 'l', 'e', 'c', 'i', 'l', 'a', 'c', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 996);
							s.is_empty()
						}
						['r', 'e', 't', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1852);
									s.is_empty()
								}
								['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'e', 'n', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 999);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1000);
							match s {
								[] => true,
								['m', 'm', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1001);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1855);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 2709);
											s.is_empty()
										}
										['e', 'l', 'l', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2710);
											s.is_empty()
										}
										['i', 'l', 'l', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2711);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'r', 'i', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1003);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1004);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1856);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1006);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 1012);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['e', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 1858);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1859);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													super::push(out, 3147);
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 3148);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'b', 'b', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1015);
							s.is_empty()
						}
						['ì', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 419);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t1::n200(s, out),
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				['p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['t', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', 'a', 'm', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				['g', 'n', 'i', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1973);
									s.is_empty()
								}
								['s', 's', 'a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'g', 'o', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				['t', 'a', 'c', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['\u{12}', 'i', '\u{1}', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'o', 'c', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', 'i', 'm', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1061);
							s.is_empty()
						}
						['v', 'v', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 437);
					match s {
						[] => true,
						['i', 't', 't', 'a', 'b', 's', '\u{1}', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['n', 'a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 1064);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 2787);
											s.is_empty()
										}
										['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2788);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 't', 't', 'a', 'b', 's', '\u{1}', s @ ..] => {
									super::push(out, 1975);
									s.is_empty()
								}
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

/// Words ending -wa.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	s.is_empty()
}

/// The word abaya (3 syllables).
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// Words ending -za.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['\u{12}', 'n', 'a', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 439);
					match s {
						[] => true,
						['a', 'r', 'e', 'g', 'i', 'l', 'l', 'e', 'b', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['o', 'f', 'o', 'c', 'o', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 442);
					match s {
						[] => true,
						['a', 'm', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'o', 't', s @ ..] => {
									super::push(out, 1976);
									match s {
										[] => true,
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2789);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'o', s @ ..] => {
									super::push(out, 1977);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2790);
											s.is_empty()
										}
										['p', 's', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2791);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'p', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						['o', 'c', 'c', 'i', 'p', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word àa (1 syllables).
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	s.is_empty()
}

/// The word dèa (1 syllables).
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	s.is_empty()
}

/// Words ending -ìa.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['\u{12}', 'c', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 188);
			s.is_empty()
		}
		['\u{13}', 'm', 'e', 't', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tavoce (3 syllables).
pub(crate) fn n3475(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3189);
			s.is_empty()
		}
		['s', 'e', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sce (3 syllables).
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['a', 'f', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uce (3 syllables).
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['d', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		['l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -de.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['\u{12}', 'a', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 459);
					match s {
						[] => true,
						['d', 'u', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						['h', 'c', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						['i', 'b', s @ ..] => {
							super::push(out, 1087);
							match s {
								[] => true,
								['a', 'i', 'c', 'c', 'a', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 1993);
									s.is_empty()
								}
								['i', 'g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'm', 'a', 'l', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1088);
							s.is_empty()
						}
						['t', 'u', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1090);
							match s {
								[] => true,
								['a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'a', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1091);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1092);
							s.is_empty()
						}
						['s', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1094);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1095);
							s.is_empty()
						}
						['h', 'c', 'a', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1097);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 1996);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1997);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1098);
							match s {
								[] => true,
								['l', 'c', s @ ..] => {
									super::push(out, 1999);
									s.is_empty()
								}
								['r', 't', 's', s @ ..] => {
									super::push(out, 2000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1099);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'c', s @ ..] => {
							super::push(out, 1100);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 1101);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1102);
							match s {
								[] => true,
								['t', 'o', 'l', 'g', s @ ..] => {
									super::push(out, 2002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ee.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// Words ending -fe.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'i', 'h', 'g', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ge.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				['d', 'i', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 466);
					s.is_empty()
				}
				['n', 'i', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 468);
					match s {
						[] => true,
						['b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1104);
							s.is_empty()
						}
						['l', 'l', 'a', s @ ..] => {
							super::push(out, 1105);
							s.is_empty()
						}
						['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1106);
							s.is_empty()
						}
						['r', 't', 's', 'o', 'c', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1108);
							s.is_empty()
						}
						['l', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -he.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1110);
							s.is_empty()
						}
						['i', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1111);
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
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'l', 'e', 'm', 'a', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1112);
							s.is_empty()
						}
						['i', 'v', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1113);
							s.is_empty()
						}
						['o', 'd', 'd', 'e', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1114);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'r', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ie.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['f', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				['l', 'g', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				['p', 'p', 'u', 'y', '\u{1}', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1116);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1117);
							match s {
								[] => true,
								['n', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'i', 'f', 'f', 'e', '\u{1}', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', 'a', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2004);
									s.is_empty()
								}
								['i', 't', 't', 'o', 'b', 'a', s @ ..] => {
									super::push(out, 2005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', 'o', 'e', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2006);
									s.is_empty()
								}
								['t', 'a', 'l', 'o', 'c', 'c', 'o', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['\u{13}', 'a', 'c', 'm', 'u', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['b', 'b', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['d', 'o', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				['g', 'n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1121);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1123);
							s.is_empty()
						}
						['o', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 't', 'u', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['u', 'o', 'j', '\u{1}', s @ ..] => {
					super::push(out, 485);
					s.is_empty()
				}
				['z', 'z', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 487);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 1125);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['t', 'n', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2795);
											s.is_empty()
										}
										['v', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 2796);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'b', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1127);
							match s {
								[] => true,
								['i', 't', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2009);
									s.is_empty()
								}
								['i', 'm', 'i', s @ ..] => {
									super::push(out, 2010);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'd', s @ ..] => {
									super::push(out, 2014);
									s.is_empty()
								}
								['r', 'o', 'm', s @ ..] => {
									super::push(out, 2015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1129);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1130);
							match s {
								[] => true,
								['n', 'a', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', s @ ..] => {
							super::push(out, 1131);
							match s {
								[] => true,
								['i', 'c', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								['t', 'o', 'u', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 't', 't', 'a', 'r', 'b', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1133);
							match s {
								[] => true,
								['a', 's', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								['o', s @ ..] => super::t1::n2550(s, out),
								_ => false,
							}
						}
						['d', 'i', 'r', 'p', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['f', 'a', 'n', 'g', 'e', 's', '\u{1}', s @ ..] => super::t1::n1420(s, out),
						['n', s @ ..] => super::t1::n1421(s, out),
						['o', s @ ..] => super::t1::n1422(s, out),
						['p', 'a', 'i', 'c', 'a', 'b', '\u{1}', s @ ..] => super::t1::n1423(s, out),
						['r', s @ ..] => super::t1::n1424(s, out),
						['s', 'e', 'l', 'i', 't', 'e', '\u{1}', s @ ..] => super::t1::n1425(s, out),
						['t', s @ ..] => super::t1::n1426(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n583(s, out),
				['o', s @ ..] => super::t1::n584(s, out),
				['r', 'e', 'p', 'a', 'l', 'i', 'f', 'n', 'i', '\u{1}', s @ ..] => super::t1::n585(s, out),
				['u', s @ ..] => super::t1::n586(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1155);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['a', 'z', 'z', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'o', 'e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['l', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2818);
											s.is_empty()
										}
										['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2819);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'r', 'e', 's', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1159);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2043);
									s.is_empty()
								}
								['f', 'n', 'i', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2044);
									s.is_empty()
								}
								['i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', 'l', 'i', 'd', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						['u', 'q', 'i', 'l', 'i', 'd', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'a', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				['i', 'c', 'n', 'o', 'c', 'i', 'd', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', 'f', s @ ..] => {
									match s {
										[] => true,
										['a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 2820);
											s.is_empty()
										}
										['i', 'g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'i', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1163);
							s.is_empty()
						}
						['n', 'a', 't', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'f', 'a', 'r', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				['s', 'i', 'h', 'c', 'u', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 504);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2047);
									s.is_empty()
								}
								['o', 'v', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2048);
									s.is_empty()
								}
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

/// Words ending -ne.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 506);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2049);
									s.is_empty()
								}
								['y', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 2050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1166);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1169);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'c', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 2822);
											s.is_empty()
										}
										['z', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2823);
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
								['a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2052);
									s.is_empty()
								}
								['g', 'a', 't', 'n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 510);
					match s {
						[] => true,
						['b', 'b', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						['c', 'o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2054);
									s.is_empty()
								}
								['t', 's', 'i', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'o', 'e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						['l', 'i', 't', 'e', s @ ..] => {
							match s {
								[] => true,
								['i', 's', 's', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2056);
									s.is_empty()
								}
								['m', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1173);
							match s {
								[] => true,
								['m', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 2058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['p', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2059);
									s.is_empty()
								}
								['t', 'n', 'o', 'm', 'a', 'c', 'c', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n607(s, out),
				['n', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2074);
									s.is_empty()
								}
								['p', s @ ..] => super::t1::n2620(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n609(s, out),
				['r', s @ ..] => super::t1::n610(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oe.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['\u{12}', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		['\u{13}', 'h', 'c', 'o', 'n', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pe.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{12}', 'u', 'o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						['t', 'o', 'e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 's', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 1205);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'i', 'c', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								['i', 'g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2104);
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
						['p', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						['u', 'q', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'z', 'a', 'c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['y', 't', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['b', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2105);
									s.is_empty()
								}
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
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
			super::push(out, 209);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['c', 'n', 'i', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								['o', 't', 't', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'i', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2849);
											s.is_empty()
										}
										['g', 'e', 'h', 'c', 'e', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2850);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1210);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2851);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2852);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1211);
							match s {
								[] => true,
								['a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 2853);
											s.is_empty()
										}
										['r', 't', 'l', 'o', '\u{1}', s @ ..] => {
											super::push(out, 2854);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1212);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 2112);
									s.is_empty()
								}
								['r', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2113);
									s.is_empty()
								}
								['s', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['n', 'o', 'd', 'n', 'a', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'b', 'e', 'f', 'a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['e', s @ ..] => super::t1::n625(s, out),
				['f', 'i', 'c', 'o', 't', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['d', 'r', 'a', '\u{1}', s @ ..] => super::t1::n1534(s, out),
						['s', s @ ..] => super::t1::n1535(s, out),
						['t', s @ ..] => super::t1::n1536(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n628(s, out),
				['p', 'a', 'c', 'a', 'i', 'c', 'c', 'u', 's', '\u{1}', s @ ..] => super::t1::n629(s, out),
				['r', s @ ..] => super::t1::n630(s, out),
				['t', s @ ..] => super::t1::n631(s, out),
				['u', s @ ..] => super::t1::n632(s, out),
				['è', s @ ..] => super::t1::n633(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						['o', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['b', 'a', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						['c', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						['e', 't', '-', 'p', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						['f', 'a', 'c', 'r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1240);
							s.is_empty()
						}
						['k', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['i', 't', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1242);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'g', 'r', 'a', 'l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2153);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								['s', 'i', 'l', 'a', 'n', 'r', 'o', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'i', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['e', 'p', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['d', 'r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						['p', 'a', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'r', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
								['i', 'a', 'v', 'a', 't', 'e', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2157);
									s.is_empty()
								}
								['j', 'a', 'v', 'a', 't', 'e', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2159);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
													super::push(out, 3216);
													s.is_empty()
												}
												['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3217);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2160);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', 'a', 'f', 'f', 'u', 'r', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2879);
											s.is_empty()
										}
										['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 2880);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1247);
							match s {
								[] => true,
								['c', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2161);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2881);
											s.is_empty()
										}
										['p', 'u', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 2882);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'r', 'e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 2162);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 't', 'n', 'i', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3218);
													s.is_empty()
												}
												['s', 'i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 3219);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'i', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'l', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								['t', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'i', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				['t', 'e', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['-', 'a', 'd', '-', 'i', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						['g', 'x', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['l', 'p', 'r', 'e', 'v', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['a', 'r', 'g', 'a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['s', 'o', 'p', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', 'o', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2166);
									s.is_empty()
								}
								['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1262);
							match s {
								[] => true,
								['a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2169);
									s.is_empty()
								}
								['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						['c', 'i', 'd', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						['l', s @ ..] => super::t1::n1586(s, out),
						['m', 'i', 'l', s @ ..] => super::t1::n1587(s, out),
						['n', s @ ..] => super::t1::n1588(s, out),
						['p', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						['r', s @ ..] => super::t1::n1590(s, out),
						['s', 'u', 'l', 'a', 'd', 'n', 'a', '\u{1}', s @ ..] => super::t1::n1591(s, out),
						['t', s @ ..] => super::t1::n1592(s, out),
						['v', 'a', s @ ..] => super::t1::n1593(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n655(s, out),
				['o', s @ ..] => super::t1::n656(s, out),
				['r', s @ ..] => super::t1::n657(s, out),
				['s', s @ ..] => super::t1::n658(s, out),
				['t', s @ ..] => super::t1::n659(s, out),
				['u', s @ ..] => super::t1::n660(s, out),
				['y', s @ ..] => super::t1::n661(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['\u{11}', 'b', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['\u{12}', 'g', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', 't', 's', 'e', 'r', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				['g', 'n', s @ ..] => {
					match s {
						[] => true,
						['a', 's', 'a', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['i', 'l', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', 'a', 'l', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['n', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['n', 'a', 'u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						['r', 'b', 'i', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2214);
									s.is_empty()
								}
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2215);
									s.is_empty()
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
						['g', 'n', 'e', 'g', 'a', 'i', 'g', 'g', 'a', 's', 's', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						['t', 'c', 'e', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', 't', 'n', 'a', 's', 's', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1294);
							s.is_empty()
						}
						['r', 'p', 'a', 'r', 'i', 't', '\u{1}', s @ ..] => {
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

/// Words ending -xe.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	s.is_empty()
}

/// Words ending -ze.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['n', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'n', 'e', 'c', 'r', 'e', 't', 's', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ée.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	s.is_empty()
}

/// Words ending -co (3 syllables).
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2915);
									s.is_empty()
								}
								['o', 'm', 'o', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'd', 'n', 'o', 'f', 's', '\u{1}', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2917);
									s.is_empty()
								}
								['b', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1316);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2919);
									s.is_empty()
								}
								['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2921);
									s.is_empty()
								}
								['e', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2922);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['f', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3229);
											s.is_empty()
										}
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 3230);
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
								['a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3231);
											s.is_empty()
										}
										['o', 'l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3232);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', 'm', 'o', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 's', 'a', 'r', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				['t', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'm', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						['s', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['a', 'v', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						['o', 'c', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['o', 'l', 'b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['t', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['a', 't', 'a', 'g', 'a', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1326);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2924);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2926);
									s.is_empty()
								}
								['p', 'p', 'a', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3233);
											s.is_empty()
										}
										['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'a', 'r', 't', 'i', 't', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						['i', 'r', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2230);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', 'c', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2927);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2929);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3235);
											s.is_empty()
										}
										['s', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'e', 'b', 'm', 'o', '\u{1}', s @ ..] => super::t1::n2810(s, out),
						['o', 'd', '\u{1}', s @ ..] => super::t1::n2811(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n1664(s, out),
				['r', s @ ..] => super::t1::n1665(s, out),
				['t', s @ ..] => super::t1::n1666(s, out),
				['v', s @ ..] => super::t1::n1667(s, out),
				['z', s @ ..] => super::t1::n1668(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n707(s, out),
		['n', s @ ..] => super::t1::n708(s, out),
		['o', s @ ..] => super::t1::n709(s, out),
		['r', s @ ..] => super::t1::n710(s, out),
		['s', s @ ..] => super::t1::n711(s, out),
		['u', s @ ..] => super::t1::n712(s, out),
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['i', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1346);
							match s {
								[] => true,
								['o', 'i', 'c', 'i', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['e', 'r', 'f', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1348);
							s.is_empty()
						}
						['p', 'r', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2256);
									match s {
										[] => true,
										['e', 'g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2934);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'r', 'a', 'l', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2935);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2936);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 's', 'i', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2257);
									s.is_empty()
								}
								['t', 'n', 'e', 'l', 'l', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2258);
									s.is_empty()
								}
								['z', 'r', 'o', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 2259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', 's', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2260);
									s.is_empty()
								}
								['u', 'n', 'i', 'm', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['f', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2937);
											s.is_empty()
										}
										['o', 's', 's', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 2262);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['g', 'a', 'v', '\u{1}', s @ ..] => {
													super::push(out, 3241);
													s.is_empty()
												}
												['r', 'i', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3242);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'o', 't', 't', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 2263);
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
						['m', 'i', 's', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						['r', 'b', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'e', 't', 's', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2264);
									s.is_empty()
								}
								['l', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'b', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2266);
									s.is_empty()
								}
								['o', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						['t', 'i', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
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

/// Words ending -eo.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'd', 'i', 'v', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 's', '\u{1}', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1356);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', 'i', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2268);
									s.is_empty()
								}
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', 'o', 't', 'a', 'm', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['z', 'a', 't', 't', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2270);
									s.is_empty()
								}
								['o', 'l', 'c', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1359);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['c', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1365);
							match s {
								[] => true,
								['e', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2272);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', 'r', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 2939);
											s.is_empty()
										}
										['n', 'o', 'r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 2940);
											s.is_empty()
										}
										['s', 'e', 'i', 'h', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'r', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2942);
											s.is_empty()
										}
										['i', 'c', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2943);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2944);
											s.is_empty()
										}
										['g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 2945);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						['p', 'o', 'e', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 2946);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2947);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									match s {
										[] => true,
										['i', 'd', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2948);
											s.is_empty()
										}
										['u', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 2949);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2950);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 2951);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', 'r', 'u', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'r', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						['z', 'z', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['-', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						['s', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2274);
									s.is_empty()
								}
								['n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'b', 'b', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2276);
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2278);
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						['t', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['a', 'd', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2280);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2282);
									s.is_empty()
								}
								['i', 'd', 'r', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['l', s @ ..] => super::t1::n1728(s, out),
						['u', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n742(s, out),
				['u', s @ ..] => super::t1::n743(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['\u{12}', 'c', s @ ..] => {
			match s {
				[] => true,
				['n', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['c', 'o', 'i', 'h', 'c', 'c', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -io.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['r', 'd', 's', '\u{1}', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['v', 's', '\u{1}', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['-', 'n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['i', 'l', 'g', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						['l', 'l', 'i', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2286);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2288);
									s.is_empty()
								}
								['i', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						['s', 'u', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2290);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 2291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2292);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2294);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 2295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2296);
									s.is_empty()
								}
								['i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['g', 'g', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2298);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 2299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1389);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2958);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 2959);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 2300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['i', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2301);
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2960);
											s.is_empty()
										}
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 2961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2962);
											s.is_empty()
										}
										['f', '\u{1}', s @ ..] => {
											super::push(out, 2963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', s @ ..] => super::t1::n1754(s, out),
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t1::n765(s, out),
				['r', s @ ..] => super::t1::n766(s, out),
				['s', s @ ..] => super::t1::n767(s, out),
				['t', s @ ..] => super::t1::n768(s, out),
				['u', 'q', 'c', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => super::t1::n769(s, out),
				['v', s @ ..] => super::t1::n770(s, out),
				['z', s @ ..] => super::t1::n771(s, out),
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t1::n282(s, out),
		_ => false,
	}
}

/// The word risiko (3 syllables).
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	s.is_empty()
}

/// Words ending -lo.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 664);
					s.is_empty()
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				['é', 'm', '\u{1}', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['c', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2368);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', 'o', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['t', 'i', 'd', 'n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						['i', 'd', 'n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						['o', 't', 't', 'i', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1450);
							match s {
								[] => true,
								['t', 's', 'i', 'r', 'c', 'a', 'v', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 2370);
									s.is_empty()
								}
								['v', 'a', 'c', 'a', 'p', 'p', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 2371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1451);
							match s {
								[] => true,
								['m', 'm', 'a', 'c', 'o', 'z', 'z', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 2372);
									s.is_empty()
								}
								['n', 'a', 'm', 'r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2373);
									s.is_empty()
								}
								['t', 'n', 'a', 'm', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1452);
							match s {
								[] => true,
								['b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'c', 'c', 'a', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3018);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['c', 'r', 'o', 't', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4108(s, out),
														['s', '\u{1}', s @ ..] => super::t1::n4109(s, out),
														_ => false,
													}
												}
												['d', 'n', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 3256);
													s.is_empty()
												}
												['p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
													super::push(out, 3257);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 'a', 'i', 'c', 'n', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['m', 'e', 'z', s @ ..] => {
							super::push(out, 1453);
							match s {
								[] => true,
								['z', 'e', 'r', 'p', 'a', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1454);
							match s {
								[] => true,
								['i', 'n', 'a', 'm', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2378);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['d', 'i', 'u', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3258);
													s.is_empty()
												}
												['f', 'f', 'u', 'r', 'r', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3259);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3019);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 's', 'a', s @ ..] => {
							super::push(out, 1455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'm', 'o', 'g', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2380);
									s.is_empty()
								}
								['o', 'r', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3020);
											s.is_empty()
										}
										['s', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3021);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									match s {
										[] => true,
										['c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3022);
											s.is_empty()
										}
										['t', 't', 'o', 'r', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1457);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2382);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['n', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['r', 'g', 'a', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['k', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['s', 'è', 't', 'n', 'a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['c', 'a', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['t', 'a', '-', 'o', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1467);
							match s {
								[] => true,
								['-', 'o', 'n', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2384);
									s.is_empty()
								}
								['e', 'r', 't', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'h', 'c', 's', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['b', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', 'h', 'i', 'j', '\u{1}', s @ ..] => {
											super::push(out, 3024);
											s.is_empty()
										}
										['e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 3025);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'a', 'm', s @ ..] => {
									super::push(out, 2387);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3026);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'd', 'n', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['m', 'e', 'k', '\u{1}', s @ ..] => {
													super::push(out, 3260);
													s.is_empty()
												}
												['n', s @ ..] => {
													match s {
														[] => true,
														['i', 'g', 'i', 'r', 'o', '\u{1}', s @ ..] => super::t1::n4110(s, out),
														['o', 'i', 'z', 'a', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t1::n4111(s, out),
														_ => false,
													}
												}
												['r', 'e', 'b', 'i', 'l', 'l', 'i', '\u{1}', s @ ..] => {
													super::push(out, 3261);
													s.is_empty()
												}
												['u', 't', 't', 'e', 'l', 'l', 'e', 't', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3262);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 't', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3028);
											s.is_empty()
										}
										['u', 's', 'n', 'o', 'c', 'r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
											super::push(out, 3029);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2389);
									match s {
										[] => true,
										['a', 'v', s @ ..] => super::t1::n3741(s, out),
										['i', s @ ..] => super::t1::n3742(s, out),
										['o', 't', 'n', 'i', 'l', 'c', '\u{1}', s @ ..] => super::t1::n3743(s, out),
										['u', 'm', 'o', 'c', 'o', s @ ..] => super::t1::n3744(s, out),
										_ => false,
									}
								}
								['r', s @ ..] => super::t1::n3010(s, out),
								['s', 'e', 'p', 'o', 'i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => super::t1::n3011(s, out),
								['t', s @ ..] => super::t1::n3012(s, out),
								['v', 'a', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => super::t1::n3013(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n806(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['h', 'c', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1472);
							match s {
								[] => true,
								['c', 'l', 'u', 'p', 'e', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2395);
									s.is_empty()
								}
								['l', 'i', 'z', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2397);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['s', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3272);
													s.is_empty()
												}
												['t', 't', 'o', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 3273);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'r', 'e', 'p', 'o', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2398);
									s.is_empty()
								}
								['z', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3038);
											s.is_empty()
										}
										['z', 'a', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3039);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1473);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
											super::push(out, 3040);
											s.is_empty()
										}
										['i', 'g', 'g', 'o', 'p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3274);
													s.is_empty()
												}
												['p', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3275);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3041);
											s.is_empty()
										}
										['n', 'o', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3042);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['n', 'a', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 3276);
													s.is_empty()
												}
												['s', 'a', 'f', '\u{1}', s @ ..] => {
													super::push(out, 3277);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						['t', 'e', 'm', 'o', 'r', 'o', 'u', 'l', 'f', 'o', 'r', 'o', 'l', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2399);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 3044);
											s.is_empty()
										}
										['i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3045);
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
								['r', 'e', 't', 'n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2400);
									s.is_empty()
								}
								['t', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'd', 'n', 'e', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2402);
									s.is_empty()
								}
								['c', 'l', 'a', 'c', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2403);
									s.is_empty()
								}
								['p', 'm', 'o', 'c', 'a', 'b', 'b', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1479);
							match s {
								[] => true,
								['l', 'a', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2405);
									s.is_empty()
								}
								['n', 'o', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1480);
							match s {
								[] => true,
								['c', 'c', 'a', 'f', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 'z', 'z', 'a', 'p', 's', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => super::t1::n1868(s, out),
						['r', s @ ..] => super::t1::n1869(s, out),
						['s', 'a', 'l', 'a', 'c', 'i', 'r', 'a', 'c', 's', '\u{1}', s @ ..] => super::t1::n1870(s, out),
						['t', 's', 'i', s @ ..] => super::t1::n1871(s, out),
						['v', 'a', s @ ..] => super::t1::n1872(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n815(s, out),
				['o', s @ ..] => super::t1::n816(s, out),
				['r', s @ ..] => super::t1::n817(s, out),
				['u', 't', 'n', 'a', 'v', 'o', 'n', '\u{1}', s @ ..] => super::t1::n818(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['\u{11}', 'z', '\u{1}', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['d', 'o', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				['p', 'm', 'a', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['l', 'r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['p', 'm', 'a', 'h', 's', '-', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1487);
							match s {
								[] => true,
								['i', 'g', 'g', 'o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3046);
											s.is_empty()
										}
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3047);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['a', 'c', 'o', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						['e', 't', s @ ..] => {
							match s {
								[] => true,
								['-', 'o', 'i', 'z', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2423);
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3048);
											s.is_empty()
										}
										['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 3049);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2424);
									s.is_empty()
								}
								['o', 'i', 'z', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'u', 'r', 'g', 'o', 'p', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2426);
									s.is_empty()
								}
								['e', 'c', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'o', 'c', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['u', 'l', 'a', 'z', 'z', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['b', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2428);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['t', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['-', 'e', 'v', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2430);
									s.is_empty()
								}
								['i', 'g', 'r', 'e', 't', 'a', 'v', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 2431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', 'a', 't', 'a', 'v', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'u', s @ ..] => {
							super::push(out, 1499);
							match s {
								[] => true,
								['n', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['p', '-', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['v', 'o', 'b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1503);
							match s {
								[] => true,
								['a', 'r', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 2433);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['m', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 3050);
											s.is_empty()
										}
										['t', 't', s @ ..] => {
											match s {
												[] => true,
												['a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 3278);
													s.is_empty()
												}
												['e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 3279);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						['v', 'a', 'l', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2434);
									s.is_empty()
								}
								['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['e', 'f', 'a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2436);
									s.is_empty()
								}
								['s', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', s @ ..] => {
									match s {
										[] => true,
										['i', 'c', 'c', 'i', 'r', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3051);
											s.is_empty()
										}
										['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3052);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'z', 'a', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['a', 'e', 't', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2439);
									s.is_empty()
								}
								['m', 'a', 'g', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2440);
									s.is_empty()
								}
								['v', 'o', s @ ..] => {
									super::push(out, 2441);
									match s {
										[] => true,
										['i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 3053);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', 'p', s @ ..] => {
									super::push(out, 2442);
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3054);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1506);
							match s {
								[] => true,
								['a', 'n', 'a', s @ ..] => {
									super::push(out, 2444);
									s.is_empty()
								}
								['e', 'n', 'a', 'c', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['a', 't', 'n', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', 'h', 'c', 'c', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2446);
									s.is_empty()
								}
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['\u{12}', 'i', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', 'o', 'd', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						['n', 'i', 'g', 'n', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						['p', 'a', 'r', 'v', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'a', 'd', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'i', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 1514);
							match s {
								[] => true,
								['i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'i', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', 'c', 'o', 't', 'l', 'a', 'p', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2449);
									s.is_empty()
								}
								['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2451);
									s.is_empty()
								}
								['r', 'e', 'b', 'y', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', 'i', 'm', 'a', 't', 'l', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 3055);
											s.is_empty()
										}
										['n', 'i', 'm', 'a', 't', 'l', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 3056);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'g', s @ ..] => {
									super::push(out, 2453);
									match s {
										[] => true,
										['o', 'i', 'h', 'c', 'c', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'l', 'f', 'a', 'r', 'u', 's', 'i', 'm', '\u{1}', s @ ..] => {
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

/// Words ending -to.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1520);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1522);
							match s {
								[] => true,
								['i', 'r', 'b', 'b', 'a', 'f', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								['r', 'e', 'm', 'a', 'r', 't', 's', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', 'a', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2456);
									s.is_empty()
								}
								['f', 'i', 'd', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2458);
									s.is_empty()
								}
								['n', 'i', 'c', 'n', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', 'i', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1524);
							match s {
								[] => true,
								['s', '-', 'à', 't', 't', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'p', 'p', 'a', 't', 'i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1526);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3058);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['f', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 3280);
													s.is_empty()
												}
												['i', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3281);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'l', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 3059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						['j', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1528);
							s.is_empty()
						}
						['r', 'c', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						['t', 'r', 'a', 'p', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				['n', s @ ..] => super::t1::n860(s, out),
				['o', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 2467);
									s.is_empty()
								}
								['o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t1::n1943(s, out),
						['u', 'n', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n862(s, out),
				['s', s @ ..] => super::t1::n863(s, out),
				['t', s @ ..] => super::t1::n864(s, out),
				['u', s @ ..] => super::t1::n865(s, out),
				['é', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['\u{11}', 'd', '\u{1}', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['\u{13}', 'g', 'e', 'i', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 253);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['\u{12}', 'o', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1547);
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
				['e', 'o', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2497);
									s.is_empty()
								}
								['d', 'e', 'c', 's', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2498);
									s.is_empty()
								}
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

/// The word yo-yo (2 syllables).
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	s.is_empty()
}

/// Words ending -zo.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['m', 'o', 'r', 'o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						['r', 'p', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 's', 'a', 'n', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1551);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['g', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3291);
													s.is_empty()
												}
												['s', 's', 'a', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 3292);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3080);
											s.is_empty()
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
								['m', 'l', 'a', 'm', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2499);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2500);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['n', 'o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 3293);
													s.is_empty()
												}
												['p', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3294);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'm', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lecca (3 syllables).
pub(crate) fn n1983(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', 'a', 'c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2505);
			s.is_empty()
		}
		['a', 'c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2506);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -secca (3 syllables).
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				['t', 'n', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'n', 'r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -occa (3 syllables).
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		['c', 's', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ica (3 syllables).
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['i', 's', 's', 'a', 'l', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['a', 'n', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3083);
									s.is_empty()
								}
								['d', 'i', 'u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3084);
									s.is_empty()
								}
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3085);
									s.is_empty()
								}
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3086);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 3295);
											s.is_empty()
										}
										['o', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3296);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['d', 'i', '\u{1}', s @ ..] => {
											super::push(out, 3297);
											s.is_empty()
										}
										['t', 's', 'a', '\u{1}', s @ ..] => {
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
				['i', 'h', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'o', 'r', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						['r', 't', 't', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						['t', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', 'e', 'g', 'a', 'm', 'r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 'c', 'e', 'm', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['t', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 't', 'o', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', 'n', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						['p', 'o', 'r', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', 'n', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						['r', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3087);
									s.is_empty()
								}
								['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['a', 'c', 'r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['b', 'b', 'a', 'f', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'm', 'o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['t', 'o', 'i', 'd', 'a', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['i', 'f', 'a', 'c', 'i', 'm', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'o', 'c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3089);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3090);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3366);
													s.is_empty()
												}
												['o', s @ ..] => {
													match s {
														[] => true,
														['i', 'b', '\u{1}', s @ ..] => super::t1::n4141(s, out),
														['m', 'r', 'e', 't', '\u{1}', s @ ..] => super::t1::n4142(s, out),
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
						['o', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						['t', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'o', 'p', 'o', 'c', 'i', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 3091);
									s.is_empty()
								}
								['h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3093);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 's', 'i', '\u{1}', s @ ..] => {
											super::push(out, 3299);
											s.is_empty()
										}
										['o', 't', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3300);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', 't', 'r', 'o', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3094);
									s.is_empty()
								}
								['u', 'g', 'n', 'i', 'l', 'o', 'c', 'i', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', 'g', 'a', 'i', 'd', 'o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3096);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3301);
											s.is_empty()
										}
										['r', 't', 's', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3302);
											s.is_empty()
										}
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

/// Words ending -nca (3 syllables).
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['p', 'a', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ioca (3 syllables).
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rca (3 syllables).
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['-', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3097);
									s.is_empty()
								}
								['t', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['t', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['o', 'r', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 't', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						['o', 'i', 'c', s @ ..] => {
							match s {
								[] => true,
								['c', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3099);
									s.is_empty()
								}
								['u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['f', 'a', 'p', 'm', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['n', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				['z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sca (3 syllables).
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['i', 'g', 'g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['r', 'r', 'u', 'b', 'm', 'a', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'p', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				['e', 'c', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ia (3 syllables).
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['c', 'a', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['a', 's', 'i', 'd', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3103);
											s.is_empty()
										}
										['t', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 't', 'i', 't', 'e', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2548);
									s.is_empty()
								}
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
			super::push(out, 326);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2549);
									s.is_empty()
								}
								['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', 'a', 'f', 'o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['d', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3303);
													s.is_empty()
												}
												['i', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3304);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'a', 't', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2551);
									s.is_empty()
								}
								['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2552);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2553);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['o', 't', s @ ..] => {
													match s {
														[] => true,
														['t', 'o', 's', '\u{1}', s @ ..] => super::t1::n4126(s, out),
														['u', 'a', '\u{1}', s @ ..] => super::t1::n4127(s, out),
														_ => false,
													}
												}
												['s', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3305);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'a', 'r', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2555);
									s.is_empty()
								}
								['m', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 2557);
									s.is_empty()
								}
								['t', 'r', 'a', 'c', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2558);
									s.is_empty()
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
						['p', 'o', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['t', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['r', 't', 'e', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						['i', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2559);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 't', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'x', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2561);
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2562);
									s.is_empty()
								}
								['u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', 'l', 'u', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2564);
									s.is_empty()
								}
								['f', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 2565);
									s.is_empty()
								}
								['t', 's', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', 'a', 'c', '\u{1}', s @ ..] => super::t1::n2098(s, out),
						['o', 'c', 'o', 'r', 't', 'e', 'i', 'd', '\u{1}', s @ ..] => super::t1::n2099(s, out),
						['u', 'i', 't', 'a', s @ ..] => super::t1::n2100(s, out),
						_ => false,
					}
				}
				['u', 'd', 'i', 'f', s @ ..] => super::t1::n1003(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n391(s, out),
		['f', s @ ..] => super::t1::n392(s, out),
		['g', s @ ..] => super::t1::n393(s, out),
		['h', s @ ..] => super::t1::n394(s, out),
		['k', s @ ..] => super::t1::n395(s, out),
		['l', s @ ..] => super::t1::n396(s, out),
		['m', s @ ..] => super::t1::n397(s, out),
		['n', s @ ..] => super::t1::n398(s, out),
		['p', s @ ..] => super::t1::n399(s, out),
		['r', s @ ..] => super::t1::n400(s, out),
		['s', s @ ..] => super::t1::n401(s, out),
		['t', s @ ..] => super::t1::n402(s, out),
		['u', 'l', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => super::t1::n403(s, out),
		['v', s @ ..] => super::t1::n404(s, out),
		['z', s @ ..] => super::t1::n405(s, out),
		_ => false,
	}
}

/// Words ending -alla (3 syllables).
pub(crate) fn n1062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 866);
	match s {
		[] => true,
		['h', 'g', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1694);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2611);
					s.is_empty()
				}
				['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1695);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ella (3 syllables).
pub(crate) fn n1063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 867);
	match s {
		[] => true,
		['c', 'i', 'l', 'o', 'p', 'l', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		['d', 'u', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'u', 'q', 'c', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3121);
							s.is_empty()
						}
						['i', 'c', 'r', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 3122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 2613);
					match s {
						[] => true,
						['t', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 'c', 'l', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'p', 'a', 'c', 'o', 'i', 'c', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1699);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['a', 't', 't', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2617);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3124);
							s.is_empty()
						}
						['r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2618);
					s.is_empty()
				}
				['s', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3126);
							s.is_empty()
						}
						['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -illa (3 syllables).
pub(crate) fn n1064(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'm', 'a', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1702);
			s.is_empty()
		}
		['l', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1703);
			s.is_empty()
		}
		['r', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1704);
			s.is_empty()
		}
		['z', 'p', 'm', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olla (3 syllables).
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 868);
	match s {
		[] => true,
		['c', 'n', 'i', 'p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1706);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						['r', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 3129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'r', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 2620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'f', 'a', 't', 's', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1707);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word nonnulla (3 syllables).
pub(crate) fn n2174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1709);
	s.is_empty()
}

