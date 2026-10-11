use flexion::Path;

/// Words ending -ot.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1110);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['s', 't', 'u', s @ ..] => {
					super::push(out, 2267);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 3045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1113);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1114);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				['l', 'a', 'b', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2272);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1116);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2275);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2276);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3047);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3048);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3309);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3387);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3388);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 3051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2278);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1121);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1122);
			s.is_empty()
		}
		['u', 'q', 'i', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1123);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1124);
			s.is_empty()
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', '\u{1}', s @ ..] => {
			super::push(out, 1125);
			s.is_empty()
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pt.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['a', 'w', 's', 'r', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3052);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3310);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 3421);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 3422);
													s.is_empty()
												}
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
							super::push(out, 3053);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3311);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 3389);
											s.is_empty()
										}
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
					super::push(out, 2289);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				['n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['p', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						['g', 'r', 'e', 'b', 'b', 'o', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 2298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1132);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1134);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['u', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3312);
									s.is_empty()
								}
								['i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3313);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3064);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3066);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3390);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													super::push(out, 3423);
													s.is_empty()
												}
												['r', '\u{1}', s @ ..] => {
													super::push(out, 3424);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', s @ ..] => {
											super::push(out, 3391);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3392);
											s.is_empty()
										}
										['p', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3393);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 3425);
													s.is_empty()
												}
												['v', 'o', '\u{1}', s @ ..] => {
													super::push(out, 3426);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3394);
											s.is_empty()
										}
										['t', 'u', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tt.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['o', 't', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['r', 'o', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1144);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1145);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1146);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['n', 'i', 's', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1149);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vt.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wt.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	s.is_empty()
}

/// Words ending -xt.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1152);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrow.
pub(crate) fn n2438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2354);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3086);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3396);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sow.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1192);
	match s {
		[] => true,
		['a', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vow.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	s.is_empty()
}

/// Words ending -terfly.
pub(crate) fn n2476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2391);
	s.is_empty()
}

/// Words ending -lly.
pub(crate) fn n1255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1228);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			super::push(out, 2392);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3099);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oly.
pub(crate) fn n1256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1229);
	match s {
		[] => true,
		['h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2393);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2394);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ply.
pub(crate) fn n1257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1230);
	s.is_empty()
}

/// Words ending -rly.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 2395);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -my.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	s.is_empty()
}

/// Words ending -ny.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	s.is_empty()
}

/// Words ending -oy.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -py.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	s.is_empty()
}

/// Words ending -ry.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sy.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	s.is_empty()
}

/// Words ending -ty.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['n', 'u', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uy.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1235);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['p', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 1238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vy.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	s.is_empty()
}

/// Words ending -xy.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	s.is_empty()
}

/// Words ending -zy.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	s.is_empty()
}

/// Words ending -nsound.
pub(crate) fn n1385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	s.is_empty()
}

/// The word thunderstrike.
pub(crate) fn n3560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3427);
	s.is_empty()
}

/// Words ending -orcarve.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2663);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 3205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -set.
pub(crate) fn n1099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1074);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2176);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2966);
					s.is_empty()
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2967);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2969);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2178);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2179);
			s.is_empty()
		}
		['k', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2971);
					s.is_empty()
				}
				['i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2180);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2973);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
					super::push(out, 2975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2182);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2183);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2978);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2980);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', s @ ..] => {
					super::push(out, 2981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2184);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2983);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2984);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pacquet.
pub(crate) fn n3089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2985);
	s.is_empty()
}

/// The word briquet.
pub(crate) fn n3090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2986);
	s.is_empty()
}

/// Words ending -oquet.
pub(crate) fn n3091(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3286);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vet.
pub(crate) fn n1102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1077);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2186);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2187);
			s.is_empty()
		}
		['i', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2188);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2189);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2991);
					s.is_empty()
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wet.
pub(crate) fn n1103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1078);
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 2190);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yet.
pub(crate) fn n1104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1079);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gsit.
pub(crate) fn n2318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2242);
	s.is_empty()
}

/// The word withsit.
pub(crate) fn n2319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2243);
	s.is_empty()
}

/// Words ending -isit.
pub(crate) fn n2320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2244);
	s.is_empty()
}

/// Words ending -nsit.
pub(crate) fn n2321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2245);
	match s {
		[] => true,
		['e', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 3035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osit.
pub(crate) fn n2322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2246);
	s.is_empty()
}

/// Words ending -rsit.
pub(crate) fn n2323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2247);
	s.is_empty()
}

/// Words ending -tsit.
pub(crate) fn n2324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2248);
	s.is_empty()
}

/// Words ending -ysit.
pub(crate) fn n2325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2249);
	s.is_empty()
}

/// The word tit.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1099);
	s.is_empty()
}

/// Words ending -uit.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1100);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2250);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3036);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['e', 'g', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3038);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3039);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2252);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2253);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vit.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1101);
	s.is_empty()
}

/// Words ending -wit.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1102);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2254);
			s.is_empty()
		}
		['e', 'r', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2255);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xit.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2257);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2258);
			s.is_empty()
		}
		_ => false,
	}
}

