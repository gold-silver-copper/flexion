use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['r', 'a', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 37);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 38);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 236);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 39);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 40);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n4(s, out),
		['r', s @ ..] => super::t0::n5(s, out),
		['t', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 227);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 229);
							match s {
								[] => true,
								['e', 'd', 'e', s @ ..] => {
									super::push(out, 906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 230);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 907);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 908);
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 2891);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 2892);
													s.is_empty()
												}
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
					super::push(out, 35);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 231);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1942);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1943);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 232);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1944);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1945);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 233);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 234);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 36);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 42);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 43);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 238);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 239);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 240);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 241);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1946);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 2893);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 1947);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 2894);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 1948);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 2895);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 2896);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 2897);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 1949);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1950);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1951);
											match s {
												[] => true,
												['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 2898);
													s.is_empty()
												}
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
					super::push(out, 45);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 242);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1952);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 1953);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 243);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 911);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 1954);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1955);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 2899);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 2900);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 1956);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 912);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1957);
											s.is_empty()
										}
										['p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 913);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 244);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 245);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 915);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 916);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1959);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1960);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 917);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 246);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 247);
							match s {
								[] => true,
								['g', 'e', s @ ..] => {
									super::push(out, 919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 248);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 920);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 921);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 922);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 923);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 249);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n14(s, out),
		['d', s @ ..] => super::t0::n15(s, out),
		['e', s @ ..] => super::t0::n16(s, out),
		['f', s @ ..] => super::t0::n17(s, out),
		['g', s @ ..] => super::t0::n18(s, out),
		['h', s @ ..] => super::t0::n19(s, out),
		['i', s @ ..] => super::t0::n20(s, out),
		['l', s @ ..] => super::t0::n21(s, out),
		['m', s @ ..] => super::t0::n22(s, out),
		['n', s @ ..] => super::t0::n23(s, out),
		['o', s @ ..] => {
			super::push(out, 23);
			s.is_empty()
		}
		['p', s @ ..] => super::t0::n25(s, out),
		['r', s @ ..] => super::t0::n26(s, out),
		['s', s @ ..] => super::t0::n27(s, out),
		['t', s @ ..] => super::t0::n28(s, out),
		['u', s @ ..] => super::t0::n29(s, out),
		['v', s @ ..] => super::t0::n30(s, out),
		['x', s @ ..] => super::t0::n31(s, out),
		['z', s @ ..] => super::t0::n32(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['o', s @ ..] => super::t0::n33(s, out),
		_ => false,
	}
}

/// Words ending -co.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['i', 's', 'e', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 261);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 931);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 932);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1962);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				['d', s @ ..] => super::t0::n268(s, out),
				['e', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 942);
							match s {
								[] => true,
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t0::n270(s, out),
				['l', s @ ..] => super::t0::n271(s, out),
				['m', s @ ..] => {
					super::push(out, 268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						['e', s @ ..] => super::t0::n972(s, out),
						['i', s @ ..] => super::t0::n973(s, out),
						['r', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n273(s, out),
				['p', s @ ..] => super::t0::n274(s, out),
				['r', s @ ..] => super::t0::n275(s, out),
				['s', s @ ..] => super::t0::n276(s, out),
				['t', s @ ..] => super::t0::n277(s, out),
				['v', s @ ..] => super::t0::n278(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n55(s, out),
		['n', s @ ..] => super::t0::n56(s, out),
		['o', s @ ..] => super::t0::n57(s, out),
		['r', s @ ..] => super::t0::n58(s, out),
		['s', s @ ..] => super::t0::n59(s, out),
		['u', s @ ..] => super::t0::n60(s, out),
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				['h', 'c', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1023);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1024);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 61);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1028);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1029);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2095);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1030);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1033);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1037);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2097);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 308);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1040);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1041);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2099);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2100);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2101);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3020);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1042);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1043);
							match s {
								[] => true,
								['i', 'o', s @ ..] => {
									super::push(out, 2102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1044);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1046);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2104);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1047);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 310);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['i', 'r', 'f', 'n', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1056);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 313);
					match s {
						[] => true,
						['i', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['a', 'l', 'i', s @ ..] => {
							super::push(out, 1059);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => super::t0::n2178(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n319(s, out),
				['s', s @ ..] => super::t0::n320(s, out),
				['t', s @ ..] => super::t0::n321(s, out),
				['v', s @ ..] => super::t0::n322(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n66(s, out),
		['n', s @ ..] => super::t0::n67(s, out),
		['o', s @ ..] => super::t0::n68(s, out),
		['r', s @ ..] => super::t0::n69(s, out),
		['u', s @ ..] => super::t0::n70(s, out),
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 337);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1097);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1099);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['h', 'e', s @ ..] => {
							super::push(out, 1101);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['o', 's', 'e', s @ ..] => {
							super::push(out, 1102);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2155);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1103);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2157);
									s.is_empty()
								}
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
			super::push(out, 71);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1104);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1105);
							match s {
								[] => true,
								['p', 'e', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 344);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1106);
							match s {
								[] => true,
								['c', 'e', s @ ..] => {
									super::push(out, 2159);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 3045);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'm', 'e', s @ ..] => {
							super::push(out, 1107);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2160);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2162);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3046);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 3047);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 346);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1108);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1109);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1110);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 347);
					match s {
						[] => true,
						['i', 'm', 'e', s @ ..] => {
							super::push(out, 1111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2166);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2168);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 3048);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 2169);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3049);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 2170);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2171);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2172);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2173);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2174);
									s.is_empty()
								}
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
			super::push(out, 72);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1112);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1113);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 2175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 1114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n355(s, out),
				['n', s @ ..] => super::t0::n356(s, out),
				['o', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						['o', s @ ..] => super::t0::n1151(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n74(s, out),
		['g', s @ ..] => super::t0::n75(s, out),
		['i', s @ ..] => super::t0::n76(s, out),
		['l', s @ ..] => super::t0::n77(s, out),
		['m', s @ ..] => super::t0::n78(s, out),
		['n', s @ ..] => super::t0::n79(s, out),
		['o', s @ ..] => super::t0::n80(s, out),
		['p', s @ ..] => super::t0::n81(s, out),
		['r', s @ ..] => super::t0::n82(s, out),
		['s', s @ ..] => super::t0::n83(s, out),
		['t', s @ ..] => super::t0::n84(s, out),
		['u', s @ ..] => super::t0::n85(s, out),
		['v', s @ ..] => super::t0::n86(s, out),
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['p', 'r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 399);
					s.is_empty()
				}
				['r', 'f', 'f', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 404);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2234);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2235);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2236);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['g', 'e', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1179);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1180);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 409);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1182);
							match s {
								[] => true,
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 2240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1183);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 410);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1185);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2241);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1186);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1187);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2242);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2243);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1189);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1190);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1191);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2244);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2245);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1192);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2247);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2248);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3080);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1193);
							s.is_empty()
						}
						['l', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1196);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1197);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 414);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1199);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1200);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1202);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 416);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1205);
							match s {
								[] => true,
								['f', 'e', s @ ..] => {
									super::push(out, 2250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1208);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2251);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2252);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2253);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => super::t0::n422(s, out),
				['x', s @ ..] => {
					super::push(out, 418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n91(s, out),
		['n', s @ ..] => super::t0::n92(s, out),
		['o', s @ ..] => super::t0::n93(s, out),
		['r', s @ ..] => super::t0::n94(s, out),
		['u', s @ ..] => super::t0::n95(s, out),
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1244);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2287);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 435);
					match s {
						[] => true,
						['a', 'b', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2289);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						['s', s @ ..] => {
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

/// Words ending -io.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1255);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2291);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 3101);
											match s {
												[] => true,
												['a', 't', s @ ..] => super::t0::n3602(s, out),
												_ => false,
											}
										}
										['c', s @ ..] => super::t0::n3213(s, out),
										['d', s @ ..] => super::t0::n3214(s, out),
										['g', s @ ..] => super::t0::n3215(s, out),
										['l', s @ ..] => super::t0::n3216(s, out),
										['m', s @ ..] => super::t0::n3217(s, out),
										['n', s @ ..] => super::t0::n3218(s, out),
										['p', s @ ..] => super::t0::n3219(s, out),
										['r', s @ ..] => super::t0::n3220(s, out),
										['u', s @ ..] => super::t0::n3221(s, out),
										['v', s @ ..] => super::t0::n3222(s, out),
										_ => false,
									}
								}
								['i', s @ ..] => super::t0::n2363(s, out),
								['l', s @ ..] => super::t0::n2364(s, out),
								['r', s @ ..] => super::t0::n2365(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n1288(s, out),
						['l', s @ ..] => super::t0::n1289(s, out),
						['m', s @ ..] => super::t0::n1290(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n445(s, out),
				['i', s @ ..] => super::t0::n446(s, out),
				['l', s @ ..] => super::t0::n447(s, out),
				['n', s @ ..] => super::t0::n448(s, out),
				['o', s @ ..] => super::t0::n449(s, out),
				['r', s @ ..] => super::t0::n450(s, out),
				['s', s @ ..] => super::t0::n451(s, out),
				['u', s @ ..] => super::t0::n452(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n102(s, out),
		['e', s @ ..] => super::t0::n103(s, out),
		['f', s @ ..] => super::t0::n104(s, out),
		['g', s @ ..] => super::t0::n105(s, out),
		['h', s @ ..] => super::t0::n106(s, out),
		['l', s @ ..] => super::t0::n107(s, out),
		['m', s @ ..] => super::t0::n108(s, out),
		['n', s @ ..] => super::t0::n109(s, out),
		['p', s @ ..] => super::t0::n110(s, out),
		['r', s @ ..] => super::t0::n111(s, out),
		['s', s @ ..] => super::t0::n112(s, out),
		['t', s @ ..] => super::t0::n113(s, out),
		['u', s @ ..] => super::t0::n114(s, out),
		['v', s @ ..] => super::t0::n115(s, out),
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['u', 's', 'n', s @ ..] => {
							super::push(out, 1374);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 2393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 2394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['i', 'v', 'e', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['i', 's', 'n', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1386);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2395);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['u', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2397);
									s.is_empty()
								}
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
			super::push(out, 119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1391);
							match s {
								[] => true,
								['s', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1392);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1393);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2400);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 3181);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => super::t0::n2481(s, out),
								_ => false,
							}
						}
						['c', s @ ..] => super::t0::n1429(s, out),
						['f', s @ ..] => super::t0::n1430(s, out),
						['m', s @ ..] => {
							super::push(out, 1396);
							match s {
								[] => true,
								['o', s @ ..] => super::t0::n2485(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => super::t0::n1432(s, out),
						['t', s @ ..] => super::t0::n1433(s, out),
						['v', s @ ..] => super::t0::n1434(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n542(s, out),
				['o', s @ ..] => super::t0::n543(s, out),
				['u', s @ ..] => super::t0::n544(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n122(s, out),
		['p', s @ ..] => super::t0::n123(s, out),
		['u', s @ ..] => super::t0::n124(s, out),
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'l', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 560);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2447);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2448);
									s.is_empty()
								}
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
			super::push(out, 124);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['p', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1449);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2449);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2450);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3209);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3210);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1450);
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
				['l', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1453);
							s.is_empty()
						}
						['p', 'e', s @ ..] => {
							super::push(out, 1454);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2451);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1457);
							match s {
								[] => true,
								['g', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 2453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1458);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['l', 'f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2456);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['p', 'e', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2458);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2459);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2460);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3211);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3212);
											s.is_empty()
										}
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
					super::push(out, 580);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1462);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2461);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3213);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 2462);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2463);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3215);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1463);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2465);
									s.is_empty()
								}
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
			super::push(out, 130);
			s.is_empty()
		}
		['u', s @ ..] => super::t0::n133(s, out),
		['y', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 592);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1470);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 134);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['f', 'n', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1478);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2473);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2474);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3217);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3218);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 2475);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2476);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 3219);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3220);
											s.is_empty()
										}
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
					super::push(out, 605);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2477);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3221);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3222);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2478);
									s.is_empty()
								}
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
			super::push(out, 137);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1481);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => super::t0::n3335(s, out),
										['u', s @ ..] => super::t0::n3336(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n1523(s, out),
						['o', s @ ..] => super::t0::n1524(s, out),
						['r', s @ ..] => super::t0::n1525(s, out),
						['u', s @ ..] => super::t0::n1526(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n616(s, out),
				['g', s @ ..] => super::t0::n617(s, out),
				['l', s @ ..] => super::t0::n618(s, out),
				['m', s @ ..] => super::t0::n619(s, out),
				['p', s @ ..] => super::t0::n620(s, out),
				['r', s @ ..] => super::t0::n621(s, out),
				['s', s @ ..] => super::t0::n622(s, out),
				['t', s @ ..] => super::t0::n623(s, out),
				['u', s @ ..] => super::t0::n624(s, out),
				['v', s @ ..] => super::t0::n625(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n140(s, out),
		['n', s @ ..] => super::t0::n141(s, out),
		['o', s @ ..] => super::t0::n142(s, out),
		['r', s @ ..] => super::t0::n143(s, out),
		['s', s @ ..] => super::t0::n144(s, out),
		['u', s @ ..] => super::t0::n145(s, out),
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1538);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1545);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 2526);
									s.is_empty()
								}
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
			super::push(out, 146);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['n', 'a', 'm', 'e', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['r', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['u', 'c', 's', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2527);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2529);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3246);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 3247);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2530);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2531);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2532);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2533);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1557);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2535);
									s.is_empty()
								}
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
			super::push(out, 151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 639);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1560);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2537);
									s.is_empty()
								}
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
			super::push(out, 153);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 3248);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3249);
											s.is_empty()
										}
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
					super::push(out, 642);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1562);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['c', 'u', 'l', 'e', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2540);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2542);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2543);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 3250);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2544);
									s.is_empty()
								}
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
			super::push(out, 155);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1566);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 2545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => super::t0::n3363(s, out),
										['e', s @ ..] => super::t0::n3364(s, out),
										['i', s @ ..] => super::t0::n3365(s, out),
										['n', s @ ..] => super::t0::n3366(s, out),
										['s', s @ ..] => super::t0::n3367(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => super::t0::n1613(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n660(s, out),
				['d', s @ ..] => super::t0::n661(s, out),
				['f', s @ ..] => super::t0::n662(s, out),
				['g', s @ ..] => super::t0::n663(s, out),
				['i', s @ ..] => super::t0::n664(s, out),
				['l', s @ ..] => super::t0::n665(s, out),
				['m', s @ ..] => super::t0::n666(s, out),
				['n', s @ ..] => super::t0::n667(s, out),
				['p', s @ ..] => super::t0::n668(s, out),
				['s', s @ ..] => super::t0::n669(s, out),
				['t', s @ ..] => super::t0::n670(s, out),
				['v', s @ ..] => super::t0::n671(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n159(s, out),
		['h', s @ ..] => super::t0::n160(s, out),
		['i', s @ ..] => super::t0::n161(s, out),
		['o', s @ ..] => super::t0::n162(s, out),
		['p', s @ ..] => super::t0::n163(s, out),
		['r', s @ ..] => super::t0::n164(s, out),
		['t', s @ ..] => super::t0::n165(s, out),
		['u', s @ ..] => super::t0::n166(s, out),
		['y', s @ ..] => super::t0::n167(s, out),
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['i', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['u', 'p', 'o', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1657);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2614);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2615);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['p', 's', 'e', s @ ..] => {
							super::push(out, 1659);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2617);
									s.is_empty()
								}
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
			super::push(out, 170);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1660);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['c', 'c', s @ ..] => {
							super::push(out, 1662);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2619);
									s.is_empty()
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1666);
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 2620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1670);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 2621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['t', 'e', 'd', s @ ..] => {
									super::push(out, 2622);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3278);
											s.is_empty()
										}
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
					super::push(out, 729);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1674);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2623);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2624);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2625);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2627);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2628);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2629);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3279);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 3522);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3523);
													s.is_empty()
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 3280);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 3281);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3282);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 3283);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2630);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2631);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 2633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1677);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1678);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2635);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 3284);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3285);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 3524);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3525);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 3286);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3287);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1679);
							match s {
								[] => true,
								['t', 'e', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1681);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1682);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 3288);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						['d', s @ ..] => super::t0::n1735(s, out),
						['r', s @ ..] => super::t0::n1736(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n178(s, out),
		['i', s @ ..] => super::t1::n179(s, out),
		['l', s @ ..] => super::t1::n180(s, out),
		['n', s @ ..] => super::t1::n181(s, out),
		['o', s @ ..] => super::t1::n182(s, out),
		['p', s @ ..] => super::t1::n183(s, out),
		['r', s @ ..] => super::t1::n184(s, out),
		['s', s @ ..] => super::t1::n185(s, out),
		['t', s @ ..] => super::t1::n186(s, out),
		['u', s @ ..] => super::t1::n187(s, out),
		['æ', s @ ..] => super::t1::n188(s, out),
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1790);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2734);
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['n', 'i', 't', 's', 'n', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1793);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1795);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2737);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1799);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2739);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2740);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1800);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1802);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2743);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2745);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2746);
									s.is_empty()
								}
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
			super::push(out, 194);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2747);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2748);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 3322);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2750);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 3526);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3527);
													match s {
														[] => true,
														['e', s @ ..] => super::t1::n3692(s, out),
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
									super::push(out, 2751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 1805);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2753);
									s.is_empty()
								}
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
			super::push(out, 195);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2754);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2755);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3323);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2756);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3324);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3325);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n198(s, out),
		['t', s @ ..] => super::t1::n199(s, out),
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1817);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
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
			super::push(out, 200);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2767);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3330);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3331);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2768);
									s.is_empty()
								}
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
			super::push(out, 202);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1824);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2769);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2770);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 3332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2771);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3333);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3334);
											s.is_empty()
										}
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
					super::push(out, 825);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2772);
									s.is_empty()
								}
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
			super::push(out, 204);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 826);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1829);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['d', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 830);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 1832);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['c', 'e', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 832);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1834);
							match s {
								[] => true,
								['g', 'n', 'a', 'v', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 833);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1835);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2776);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 834);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1839);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2778);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 2779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1840);
							match s {
								[] => true,
								['g', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2780);
									s.is_empty()
								}
								['t', 'c', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1841);
							match s {
								[] => true,
								['a', 's', 'e', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 835);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1842);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2783);
									match s {
										[] => true,
										['b', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 3335);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 2784);
									match s {
										[] => true,
										['o', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3336);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1843);
							match s {
								[] => true,
								['h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1844);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2786);
									s.is_empty()
								}
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

/// Words ending -or.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 836);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2787);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3337);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3338);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 2788);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2789);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2790);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['r', 'p', 'e', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1847);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 3339);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 3340);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 3528);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 3529);
													s.is_empty()
												}
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
							super::push(out, 1848);
							match s {
								[] => true,
								['r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2792);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2793);
									match s {
										[] => true,
										['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3341);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1849);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3342);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 3530);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 3344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1850);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2794);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2795);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2796);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2798);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['e', 'm', 'e', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1853);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2800);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3345);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 3531);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 2801);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1855);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2803);
									match s {
										[] => true,
										['a', 'p', 'e', s @ ..] => {
											super::push(out, 3346);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 3532);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 3347);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2804);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3348);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 3349);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 2805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3350);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3351);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 3352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								['v', s @ ..] => super::t1::n2903(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n214(s, out),
		['e', s @ ..] => super::t1::n215(s, out),
		['f', s @ ..] => super::t1::n216(s, out),
		['g', s @ ..] => super::t1::n217(s, out),
		['h', s @ ..] => super::t1::n218(s, out),
		['i', s @ ..] => super::t1::n219(s, out),
		['l', s @ ..] => super::t1::n220(s, out),
		['m', s @ ..] => super::t1::n221(s, out),
		['n', s @ ..] => super::t1::n222(s, out),
		['p', s @ ..] => super::t1::n223(s, out),
		['r', s @ ..] => super::t1::n224(s, out),
		['s', s @ ..] => super::t1::n225(s, out),
		['t', s @ ..] => super::t1::n226(s, out),
		['u', s @ ..] => super::t1::n227(s, out),
		['v', s @ ..] => super::t1::n228(s, out),
		['x', s @ ..] => super::t1::n229(s, out),
		_ => false,
	}
}

/// Words ending -dico.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 936);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1973);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1974);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2905);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 939);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1977);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1978);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1980);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1981);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2906);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3413);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fico.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1984);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2908);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1986);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2914);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1987);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2916);
							match s {
								[] => true,
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 3415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1990);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1991);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2924);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2927);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2928);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3416);
									match s {
										[] => true,
										['e', 'l', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3556);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1993);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2931);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3418);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3419);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3420);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3557);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3558);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3421);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1995);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lico.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 945);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 946);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1996);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1997);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1998);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1999);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2943);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3559);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3560);
											s.is_empty()
										}
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

/// Words ending -emico.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2000);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -imico.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2002);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nico.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2004);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pico.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	s.is_empty()
}

/// Words ending -rico.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['v', 'i', s @ ..] => {
					super::push(out, 2006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2008);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2009);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2945);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2011);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sico.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tico.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vico.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lco.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['u', 's', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nco.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2014);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2015);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2018);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2019);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2020);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2949);
									s.is_empty()
								}
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

/// Words ending -oco.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rco.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['u', 'p', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sco.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 981);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2024);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2025);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2950);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2026);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2028);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2952);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2953);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2954);
									match s {
										[] => true,
										['e', s @ ..] => {
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
				['c', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2031);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2955);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2033);
							match s {
								[] => true,
								['a', 'm', 'e', s @ ..] => {
									super::push(out, 2957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => super::t1::n2100(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t1::n1010(s, out),
				['f', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['g', s @ ..] => super::t1::n1012(s, out),
				['i', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['l', s @ ..] => super::t1::n1014(s, out),
				['m', s @ ..] => super::t1::n1015(s, out),
				['n', s @ ..] => super::t1::n1016(s, out),
				['p', s @ ..] => super::t1::n1017(s, out),
				['r', s @ ..] => super::t1::n1018(s, out),
				['t', s @ ..] => super::t1::n1019(s, out),
				['u', s @ ..] => super::t1::n1020(s, out),
				['v', s @ ..] => super::t1::n1021(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n289(s, out),
		['o', s @ ..] => super::t1::n290(s, out),
		['u', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['f', 'n', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uco.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['s', 'e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word dilapido.
pub(crate) fn n2178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2111);
	s.is_empty()
}

/// Words ending -rido.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sido.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tido.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	s.is_empty()
}

/// Words ending -vido.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1065);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mdo.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['u', 'n', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndo.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2117);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3021);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2120);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3024);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3466);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 3467);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3025);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3468);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3469);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3470);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2121);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['e', 'r', 'p', 'e', s @ ..] => {
							super::push(out, 2123);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3027);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2126);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2127);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2128);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3032);
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
						['e', s @ ..] => {
							super::push(out, 2129);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2131);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2132);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2135);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2137);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2138);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2139);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3040);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2140);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odo.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 326);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdo.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['p', 'e', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udo.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['f', 'e', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2146);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2147);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2150);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3043);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2151);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2152);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ideo.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1115);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2176);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1117);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndeo.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3051);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3471);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3473);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3565);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3566);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3054);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3475);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3476);
									s.is_empty()
								}
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

/// Words ending -ordeo.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['m', 'e', 'd', s @ ..] => {
			super::push(out, 2185);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eeo.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	s.is_empty()
}

/// Words ending -geo.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2188);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3056);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3057);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2189);
							match s {
								[] => true,
								['e', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['e', 't', 'e', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ieo.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	s.is_empty()
}

/// Words ending -leo.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['u', 'n', 'e', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 362);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2193);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2195);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -meo.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -neo.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3060);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3061);
									s.is_empty()
								}
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
			super::push(out, 371);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2204);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3062);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2205);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3064);
									s.is_empty()
								}
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
			super::push(out, 373);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oeo.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	s.is_empty()
}

/// Words ending -peo.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['t', 's', 'n', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reo.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3066);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3067);
									s.is_empty()
								}
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
			super::push(out, 376);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['l', 'f', 'e', 'd', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3068);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1158);
					match s {
						[] => true,
						['t', 'e', s @ ..] => {
							super::push(out, 2217);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3070);
									s.is_empty()
								}
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

/// Words ending -seo.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teo.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ueo.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['r', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'e', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['o', 't', 'e', s @ ..] => {
							super::push(out, 2219);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3071);
									s.is_empty()
								}
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

/// Words ending -veo.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2221);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2223);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3072);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3073);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3074);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2230);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3076);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2231);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2232);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3078);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigo.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1210);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3477);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgo.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['v', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2255);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2256);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngo.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['l', 'p', 'e', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'e', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3084);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3086);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3479);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3480);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 3087);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3088);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3481);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3482);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2266);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3483);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 3567);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 3484);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 3568);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 3569);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 3485);
											s.is_empty()
										}
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
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2271);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2273);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3090);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3091);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2274);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2275);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2276);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2277);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3093);
									s.is_empty()
								}
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

/// Words ending -ogo.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgo.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['p', 's', 'n', s @ ..] => {
					super::push(out, 1230);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2280);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 2282);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3094);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2283);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3096);
									s.is_empty()
								}
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
			super::push(out, 430);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1236);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2285);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3097);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2286);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3099);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugo.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tabefacio.
pub(crate) fn n3602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3486);
	s.is_empty()
}

/// Words ending -cefacio.
pub(crate) fn n3213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3102);
	s.is_empty()
}

/// Words ending -defacio.
pub(crate) fn n3214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3103);
	s.is_empty()
}

/// Words ending -gefacio.
pub(crate) fn n3215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3104);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3487);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3488);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lefacio.
pub(crate) fn n3216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3105);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3489);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3570);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mefacio.
pub(crate) fn n3217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3106);
	s.is_empty()
}

/// Words ending -nefacio.
pub(crate) fn n3218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3107);
	s.is_empty()
}

/// Words ending -pefacio.
pub(crate) fn n3219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3108);
	s.is_empty()
}

/// Words ending -refacio.
pub(crate) fn n3220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3571);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3491);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3492);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3493);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uefacio.
pub(crate) fn n3221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3110);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3495);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3496);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 3497);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3498);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3582);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vefacio.
pub(crate) fn n3222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3499);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ifacio.
pub(crate) fn n2363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2292);
	s.is_empty()
}

/// Words ending -lfacio.
pub(crate) fn n2364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2293);
	s.is_empty()
}

/// Words ending -rfacio.
pub(crate) fn n2365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2294);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 3112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iacio.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	s.is_empty()
}

/// Words ending -lacio.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1257);
	s.is_empty()
}

/// Words ending -macio.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1258);
	s.is_empty()
}

/// Words ending -ccio.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	s.is_empty()
}

/// Words ending -icio.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1259);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3113);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2301);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3116);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3119);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1266);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1267);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3122);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3123);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3124);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3125);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3126);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3503);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3504);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lcio.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['a', 'c', 'e', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ncio.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['v', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3130);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3132);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocio.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1274);
			s.is_empty()
		}
		['r', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1275);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcio.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['f', 'n', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3135);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3136);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3137);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3506);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3507);
									s.is_empty()
								}
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

/// Words ending -scio.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucio.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['a', 's', 'n', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dio.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1282);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['p', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2317);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3138);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2318);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3140);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1290);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eio.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	s.is_empty()
}

/// Words ending -fio.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['c', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gio.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hio.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	s.is_empty()
}

/// Words ending -lio.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 456);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1302);
					match s {
						[] => true,
						['e', 's', 'n', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['a', 't', 's', 'n', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3143);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2330);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3145);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2331);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3147);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1306);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1307);
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['b', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2337);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1310);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1311);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mio.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['o', 'd', 'e', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2341);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nio.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1313);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2344);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2349);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3149);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2355);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3151);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3153);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pio.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 2359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1326);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2362);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3155);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3156);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2363);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2364);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2365);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2368);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3160);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['r', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 477);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['u', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['a', 's', 'n', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3162);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3163);
									s.is_empty()
								}
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
			super::push(out, 485);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1346);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1347);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3164);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 3508);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3165);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2381);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3167);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3509);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sio.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tio.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2386);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1355);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3170);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3171);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3173);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3510);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 3511);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 3512);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3513);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3174);
									s.is_empty()
								}
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
			super::push(out, 492);
			match s {
				[] => true,
				['a', 'p', 'e', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['u', 'l', 'g', 'e', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1360);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1361);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2389);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3175);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1362);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1364);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3177);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uio.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vio.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1368);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2391);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3179);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ebello.
pub(crate) fn n2481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2401);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3182);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cello.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fello.
pub(crate) fn n1430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2403);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omello.
pub(crate) fn n2485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2405);
	s.is_empty()
}

/// Words ending -pello.
pub(crate) fn n1432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2406);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tello.
pub(crate) fn n1433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	s.is_empty()
}

/// Words ending -vello.
pub(crate) fn n1434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2408);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2410);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2411);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2412);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -illo.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['r', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3514);
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3515);
									s.is_empty()
								}
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
			super::push(out, 1401);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2417);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1403);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2421);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3191);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3192);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3193);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 3194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ollo.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ullo.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olo.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1410);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2423);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2425);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2426);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2427);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plo.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	s.is_empty()
}

/// Words ending -ulo.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3196);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2430);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2431);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2432);
							match s {
								[] => true,
								['t', 'a', 'm', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 548);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2439);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3202);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2440);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2444);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3205);
									s.is_empty()
								}
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
			super::push(out, 552);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2446);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
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

/// Words ending -umo.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		['c', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 582);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 583);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 584);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 585);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['s', 'e', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 587);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1465);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2468);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['s', 'i', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ibicino.
pub(crate) fn n3335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3223);
	s.is_empty()
}

/// Words ending -ubicino.
pub(crate) fn n3336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3224);
	s.is_empty()
}

/// Words ending -ncino.
pub(crate) fn n1523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocino.
pub(crate) fn n1524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1483);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 2480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcino.
pub(crate) fn n1525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1484);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucino.
pub(crate) fn n1526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1485);
	s.is_empty()
}

/// Words ending -dino.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1486);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gino.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2482);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 3225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1491);
			match s {
				[] => true,
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lino.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1492);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1493);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2488);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3226);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1494);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2490);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1495);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1496);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1497);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mino.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1499);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2494);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1501);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1502);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2498);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3230);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3231);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3233);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pino.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1507);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1509);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rino.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1511);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sino.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tino.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1514);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1515);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1516);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3235);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3237);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3516);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uino.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1519);
			s.is_empty()
		}
		['q', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vino.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	s.is_empty()
}

/// Words ending -mno.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	s.is_empty()
}

/// Words ending -nno.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	s.is_empty()
}

/// Words ending -ono.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['t', 'o', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rno.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3240);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3241);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3242);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3243);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3518);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3519);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3244);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2525);
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
				['t', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sno.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	s.is_empty()
}

/// Words ending -uno.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dverbero.
pub(crate) fn n3363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3251);
	s.is_empty()
}

/// Words ending -everbero.
pub(crate) fn n3364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3252);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iverbero.
pub(crate) fn n3365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3253);
	s.is_empty()
}

/// Words ending -nverbero.
pub(crate) fn n3366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3254);
	s.is_empty()
}

/// Words ending -sverbero.
pub(crate) fn n3367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3255);
	s.is_empty()
}

/// Words ending -ubero.
pub(crate) fn n1613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2546);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2547);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cero.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1570);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2548);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2549);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1571);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dero.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['o', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1576);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fero.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1580);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2554);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1583);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gero.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2556);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1585);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1586);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2559);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2561);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3259);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3521);
									s.is_empty()
								}
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
			super::push(out, 1587);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iero.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2563);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lero.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['e', 'c', 's', s @ ..] => {
			super::push(out, 1590);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mero.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1591);
			s.is_empty()
		}
		['u', 'n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nero.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1592);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3261);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1593);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1594);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1595);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pero.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1597);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['r', 'p', 'e', s @ ..] => {
					super::push(out, 2570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2571);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2572);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sero.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1601);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1602);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['i', 'd', 'e', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tero.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['l', 'b', 'e', s @ ..] => {
					super::push(out, 2581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2582);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1607);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vero.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	s.is_empty()
}

/// Words ending -gro.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2586);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hro.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	s.is_empty()
}

/// Words ending -iro.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2590);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oro.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3264);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['e', 'd', 'e', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1616);
					match s {
						[] => true,
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 672);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 674);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 675);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['e', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 679);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['v', 'e', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3266);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3267);
									s.is_empty()
								}
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
			super::push(out, 681);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3268);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pro.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['u', 't', 's', 'n', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rro.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2601);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3270);
									s.is_empty()
								}
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
			super::push(out, 688);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2602);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['u', 's', 'n', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tro.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 691);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['n', 'e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1636);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['o', 'm', 'e', 'd', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uro.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1639);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3272);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3273);
									s.is_empty()
								}
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
			super::push(out, 695);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 699);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1642);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3274);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3275);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3276);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 703);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 705);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yro.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	s.is_empty()
}

/// Words ending -ducto.
pub(crate) fn n1735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1685);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ructo.
pub(crate) fn n1736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1686);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eto.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2640);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2641);
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
				['\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 734);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 735);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 736);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 740);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		_ => false,
	}
}

