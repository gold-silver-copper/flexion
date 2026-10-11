use flexion::Path;

/// Words ending -tiv.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4306);
									s.is_empty()
								}
								['o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 's', 'e', 'v', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						['z', 'o', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4308);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['p', 'm', 'e', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['k', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 4310);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4311);
									s.is_empty()
								}
								['p', 'i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3346);
							match s {
								[] => true,
								['o', 'f', 'r', s @ ..] => {
									super::push(out, 4313);
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
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', s @ ..] => {
					match s {
						[] => true,
						['s', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3349);
							s.is_empty()
						}
						['z', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4314);
									s.is_empty()
								}
								['n', 'e', 'g', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4315);
									s.is_empty()
								}
								['p', 'o', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						['i', 'p', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3353);
							s.is_empty()
						}
						['k', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3354);
							s.is_empty()
						}
						['o', 'k', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3355);
							s.is_empty()
						}
						['t', 's', 'n', 'o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 3357);
							s.is_empty()
						}
						['i', 'l', 'a', 'u', 'q', 'h', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3359);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', 'u', 'g', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4964);
											s.is_empty()
										}
										['s', 'ä', 'r', 'p', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4965);
											s.is_empty()
										}
										['t', 's', 'o', '\u{1}', s @ ..] => {
											super::push(out, 4966);
											s.is_empty()
										}
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
					super::push(out, 2238);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['o', 'n', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						['r', 'e', 's', 'n', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4317);
									s.is_empty()
								}
								['l', 'a', 'n', 'o', 'i', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4318);
									s.is_empty()
								}
								['t', 'r', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4319);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['m', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						['i', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4320);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4322);
									s.is_empty()
								}
								['o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'e', 'p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2244);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t1::n1558(s, out),
		['m', 'u', 's', 'ä', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1269);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1270);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['e', 'z', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				['i', 'r', 'k', 's', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['u', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1271);
			s.is_empty()
		}
		['s', 'e', 'g', 'g', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1272);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -end.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3376);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2260);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3378);
									s.is_empty()
								}
								['s', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3379);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 3380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3381);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3382);
									s.is_empty()
								}
								['e', 'm', s @ ..] => {
									super::push(out, 3383);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2262);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2263);
							match s {
								[] => true,
								['t', 's', 'f', s @ ..] => {
									super::push(out, 3386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 3389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 's', 'b', s @ ..] => {
							super::push(out, 2265);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3390);
									s.is_empty()
								}
								['r', 's', s @ ..] => {
									super::push(out, 3391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3392);
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3393);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1302);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 4336);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4337);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 3395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['i', 'b', 's', 't', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'w', 'r', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3396);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 3397);
									s.is_empty()
								}
								['w', 'h', 'c', 's', 'r', s @ ..] => {
									super::push(out, 3398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1304);
					match s {
						[] => true,
						['o', 'b', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['i', 'n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2271);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2272);
							match s {
								[] => true,
								['r', 't', 'e', s @ ..] => {
									super::push(out, 3399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['e', s @ ..] => super::t1::n2883(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t1::n1608(s, out),
				['r', s @ ..] => super::t1::n1609(s, out),
				['u', s @ ..] => super::t1::n1610(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n677(s, out),
		['h', s @ ..] => super::t1::n678(s, out),
		['i', s @ ..] => super::t1::n679(s, out),
		['k', s @ ..] => super::t1::n680(s, out),
		['l', s @ ..] => super::t1::n681(s, out),
		['m', s @ ..] => super::t1::n682(s, out),
		['n', s @ ..] => super::t1::n683(s, out),
		['p', s @ ..] => super::t1::n684(s, out),
		['r', s @ ..] => super::t1::n685(s, out),
		['s', s @ ..] => super::t1::n686(s, out),
		['t', s @ ..] => super::t1::n687(s, out),
		['u', s @ ..] => super::t1::n688(s, out),
		['z', s @ ..] => super::t1::n689(s, out),
		['ß', s @ ..] => super::t1::n690(s, out),
		_ => false,
	}
}

/// Words ending -und.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['b', 'i', 'r', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 577);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 578);
			s.is_empty()
		}
		['r', 's', 'i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 579);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['l', 'r', 'e', 'p', 'm', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2394);
							s.is_empty()
						}
						['n', 'r', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'l', 'r', 'e', 'p', 'm', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'd', 'i', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ständig.
pub(crate) fn n3067(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 3546);
			s.is_empty()
		}
		['d', 'n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 3547);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4422);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4995);
							s.is_empty()
						}
						['s', 'g', s @ ..] => {
							super::push(out, 4996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4424);
					s.is_empty()
				}
				['l', 'o', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4997);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3548);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'v', 'h', 'c', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 3549);
			s.is_empty()
		}
		['t', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 3550);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word vordergründig.
pub(crate) fn n4447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3553);
	s.is_empty()
}

/// Words ending -sündig.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2428);
	s.is_empty()
}

/// Words ending -tündig.
pub(crate) fn n3074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2429);
	s.is_empty()
}

/// Words ending -rdig.
pub(crate) fn n747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['e', 'n', 'e', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		['ü', 'w', s @ ..] => {
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				['s', 'g', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['m', 'm', s @ ..] => {
							super::push(out, 3554);
							s.is_empty()
						}
						['t', 'l', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udig.
pub(crate) fn n748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['e', 'r', 'f', 'n', s @ ..] => {
			super::push(out, 1409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ädig.
pub(crate) fn n749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	s.is_empty()
}

/// Words ending -eig.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fig.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 1410);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3556);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3557);
									s.is_empty()
								}
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
			super::push(out, 631);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['o', 'k', 'ß', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['ö', 'k', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						['k', 'c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						['r', 'r', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						['z', 't', 'i', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['ä', 'l', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['s', 'g', 'n', 'a', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						['ß', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gig.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['e', 'w', 'b', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3558);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'w', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3560);
									s.is_empty()
								}
								['t', 'i', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
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
						['g', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3562);
									s.is_empty()
								}
								['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3563);
									s.is_empty()
								}
								['k', 'c', 'ü', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3564);
									s.is_empty()
								}
								['n', 'e', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3565);
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 4425);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4426);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 5002);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['z', 't', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['h', 'c', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hig.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['u', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3568);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'b', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 3570);
									s.is_empty()
								}
								['r', 'e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3571);
									s.is_empty()
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
						['l', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3572);
									s.is_empty()
								}
								['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3573);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3574);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 3575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				['ü', 'r', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['s', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['ö', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['ü', 'm', s @ ..] => {
					match s {
						[] => true,
						['h', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2452);
							s.is_empty()
						}
						['t', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', 'e', 'f', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						['f', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3576);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'm', 'i', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2457);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 2458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3578);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 4427);
											s.is_empty()
										}
										['g', 'n', 'u', 'l', s @ ..] => {
											super::push(out, 4428);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'i', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 2459);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3579);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['n', 'u', s @ ..] => {
											match s {
												[] => true,
												['d', 'i', 'e', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5003);
													s.is_empty()
												}
												['g', 'i', 'm', 'h', 'e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5004);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 5005);
													s.is_empty()
												}
												['m', 'h', 'e', 'n', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 5006);
													s.is_empty()
												}
												['r', 'e', s @ ..] => {
													match s {
														[] => true,
														['n', 'i', 'e', 'm', 'e', 'g', 'l', 'l', 'a', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t1::n6525(s, out),
														['s', 's', 'e', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t1::n6526(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 5007);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'z', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4429);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'i', s @ ..] => {
									match s {
										[] => true,
										['s', 'o', 'l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 4430);
											s.is_empty()
										}
										['t', s @ ..] => super::t1::n5516(s, out),
										_ => false,
									}
								}
								['r', 'h', 'e', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3580);
									s.is_empty()
								}
								['s', 'u', 'l', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3581);
									s.is_empty()
								}
								['t', s @ ..] => super::t1::n4480(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t1::n3116(s, out),
						['ß', 'u', 'l', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => super::t1::n3117(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word breiig.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	s.is_empty()
}

/// Words ending -kig.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 645);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2461);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2462);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2468);
							match s {
								[] => true,
								['e', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						['r', 'd', s @ ..] => {
							super::push(out, 2471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 646);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		['u', 'ä', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lig.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						['i', 'e', 'w', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						['n', 'h', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						['r', 'h', s @ ..] => {
							super::push(out, 2476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'ä', s @ ..] => {
			super::push(out, 651);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['d', 'a', 'f', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						['r', 'ü', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'ü', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'n', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2480);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'ü', s @ ..] => {
			super::push(out, 653);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['m', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3586);
									s.is_empty()
								}
								['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 3587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', 's', '\u{1}', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2486);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 3588);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3589);
									s.is_empty()
								}
								['z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2489);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['s', 'n', 'a', s @ ..] => {
									super::push(out, 3591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'h', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['t', 't', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 3592);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2493);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['ä', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2495);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2497);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2498);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 2499);
							s.is_empty()
						}
						['s', 's', s @ ..] => {
							super::push(out, 2500);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2501);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3595);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'r', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mig.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 't', 's', 'n', 'i', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['h', 'c', 's', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 664);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['i', 't', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 1458);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['ö', 'f', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'i', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', 'p', s @ ..] => {
									super::push(out, 3597);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', 'u', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'e', s @ ..] => {
											super::push(out, 4433);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 4434);
											match s {
												[] => true,
												['n', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 5010);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', 'r', s @ ..] => {
											super::push(out, 4435);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 4436);
											s.is_empty()
										}
										['p', 'p', s @ ..] => {
											super::push(out, 4437);
											match s {
												[] => true,
												['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 5011);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', s @ ..] => {
									super::push(out, 3600);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3601);
									s.is_empty()
								}
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
			super::push(out, 668);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 670);
			s.is_empty()
		}
		['è', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nig.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['a', 'w', 't', 'e', '\u{1}', s @ ..] => {
			super::push(out, 672);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2507);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1461);
					s.is_empty()
				}
				['i', 's', 'n', 'e', s @ ..] => {
					super::push(out, 1462);
					match s {
						[] => true,
						['g', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pig.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['p', 'a', 'l', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rig.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['a', 'a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1464);
					match s {
						[] => true,
						['ß', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['l', 'ä', s @ ..] => {
					super::push(out, 1466);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', 'g', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3603);
									s.is_empty()
								}
								['n', 'i', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3604);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'z', 't', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3606);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4438);
											s.is_empty()
										}
										['n', 'u', 's', 's', 'a', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4439);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', 'c', 'i', 'l', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						['z', 't', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				['u', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 1471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3608);
									s.is_empty()
								}
								['i', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'd', 'o', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				['f', 'p', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['l', 'u', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 3610);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['i', 'e', 'r', 'e', 'i', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['m', 'n', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						['j', 'g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3612);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		['l', 'l', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 686);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['n', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 688);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		['s', 's', 'ä', 'w', '\u{1}', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', 'e', 's', 'e', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				['i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						['u', 'h', 'c', 's', 't', 'i', 'e', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['w', 'e', s @ ..] => {
									super::push(out, 3614);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						['ä', 'l', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3615);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3616);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['r', 't', 'f', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['ß', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 691);
			s.is_empty()
		}
		['ß', 'ä', 'w', '\u{1}', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 693);
			s.is_empty()
		}
		['ö', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				['e', 'g', s @ ..] => super::t1::n1859(s, out),
				['l', 'l', 'e', 'h', '\u{1}', s @ ..] => super::t1::n1860(s, out),
				['n', 'i', 'e', 'f', '\u{1}', s @ ..] => super::t1::n1861(s, out),
				['r', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => super::t1::n1862(s, out),
				_ => false,
			}
		}
		['ü', s @ ..] => super::t1::n830(s, out),
		_ => false,
	}
}

/// Words ending -sig.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 695);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'e', 'l', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['k', 'e', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		['l', 'l', s @ ..] => {
			super::push(out, 699);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 700);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'k', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3617);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', 'w', 'z', '\u{1}', s @ ..] => {
											super::push(out, 4440);
											s.is_empty()
										}
										['t', 'i', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 4441);
											s.is_empty()
										}
										['z', 't', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 4442);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'h', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						['r', 'f', 's', 'a', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3618);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 3619);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4443);
											s.is_empty()
										}
										['h', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4444);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['l', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3621);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 'p', 'r', 'e', 's', 's', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4445);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'e', '\u{1}', s @ ..] => {
													super::push(out, 5012);
													s.is_empty()
												}
												['h', 'c', 'ä', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 5013);
													s.is_empty()
												}
												['i', 'r', 'e', 's', '\u{1}', s @ ..] => {
													super::push(out, 5014);
													s.is_empty()
												}
												['m', 'm', 'i', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 5015);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3622);
									match s {
										[] => true,
										['k', 'l', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4446);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 't', 'e', '\u{1}', s @ ..] => {
											super::push(out, 4447);
											s.is_empty()
										}
										['k', 'n', 'u', 'p', 'r', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 4448);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'b', 'b', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3623);
									s.is_empty()
								}
								['z', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2540);
							match s {
								[] => true,
								['n', 'e', 's', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 4449);
											s.is_empty()
										}
										['z', 'l', 'e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 4450);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 's', 'l', 'e', 'k', 'r', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3626);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				['m', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['ü', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tig.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'b', 'r', 'e', 'r', 'h', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['t', 's', 'n', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				['n', 'ü', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2545);
							s.is_empty()
						}
						['n', 'r', 'e', 'v', 'd', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'ü', 'd', 'e', 'b', 's', 'g', s @ ..] => {
					super::push(out, 1516);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3627);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'l', s @ ..] => {
							super::push(out, 2547);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['k', 's', s @ ..] => {
									super::push(out, 3629);
									s.is_empty()
								}
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
			super::push(out, 706);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2549);
							match s {
								[] => true,
								['d', 'l', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2550);
							match s {
								[] => true,
								['h', 'c', 's', 'r', 'h', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3631);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3632);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4451);
											s.is_empty()
										}
										['f', s @ ..] => super::t1::n5539(s, out),
										_ => false,
									}
								}
								['r', s @ ..] => super::t1::n4539(s, out),
								['s', s @ ..] => super::t1::n4540(s, out),
								['w', s @ ..] => super::t1::n4541(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t1::n3225(s, out),
						['s', 'ü', 'h', 'c', '\u{1}', s @ ..] => super::t1::n3226(s, out),
						['u', s @ ..] => super::t1::n3227(s, out),
						['ä', s @ ..] => super::t1::n3228(s, out),
						['ü', s @ ..] => super::t1::n3229(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n847(s, out),
		['l', s @ ..] => super::t1::n848(s, out),
		['m', 'a', 's', '\u{1}', s @ ..] => super::t1::n849(s, out),
		['n', s @ ..] => super::t1::n850(s, out),
		['o', 'z', '\u{1}', s @ ..] => super::t1::n851(s, out),
		['r', s @ ..] => super::t1::n852(s, out),
		['s', s @ ..] => super::t1::n853(s, out),
		['t', s @ ..] => super::t1::n854(s, out),
		['u', s @ ..] => super::t1::n855(s, out),
		['ä', s @ ..] => super::t1::n856(s, out),
		['ö', s @ ..] => super::t1::n857(s, out),
		['ü', s @ ..] => super::t1::n858(s, out),
		_ => false,
	}
}

/// The word reuig.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	s.is_empty()
}

/// Words ending -rvig.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	s.is_empty()
}

/// Words ending -wig.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	s.is_empty()
}

/// The word juxig.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	s.is_empty()
}

/// Words ending -zig.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['n', 'i', 'e', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				['e', 'h', 'l', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						['s', 's', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'm', 'f', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßig.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 715);
			s.is_empty()
		}
		['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		['u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 718);
			s.is_empty()
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						['e', 'n', 'i', 't', 'u', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3694);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'c', 'e', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3696);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3698);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', 'i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4508);
											s.is_empty()
										}
										['z', 't', 'i', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 4509);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2609);
							match s {
								[] => true,
								['i', 'n', 't', 'l', 'ä', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3699);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 3700);
									s.is_empty()
								}
								['t', 'f', s @ ..] => {
									super::push(out, 3701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['t', 'h', 'c', 'i', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word böig.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	s.is_empty()
}

/// Words ending -ich.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3703);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3704);
									match s {
										[] => true,
										['e', 't', 'o', s @ ..] => {
											super::push(out, 4510);
											s.is_empty()
										}
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
					super::push(out, 1558);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['f', 'o', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 4511);
											s.is_empty()
										}
										['r', 'e', 'u', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4512);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'n', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 2620);
							s.is_empty()
						}
						['m', 'u', 'i', s @ ..] => {
							super::push(out, 2621);
							match s {
								[] => true,
								['c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3705);
									s.is_empty()
								}
								['t', 'n', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3707);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4513);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4514);
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
								['p', s @ ..] => {
									super::push(out, 3708);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n888(s, out),
		['t', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sch.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 740);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 741);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				['ü', 'h', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 742);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n894(s, out),
		['l', s @ ..] => {
			super::push(out, 744);
			s.is_empty()
		}
		['n', 'i', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['u', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4017);
									s.is_empty()
								}
								['n', 'u', s @ ..] => {
									super::push(out, 4018);
									s.is_empty()
								}
								['o', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 748);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xuell.
pub(crate) fn n2117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1695);
	match s {
		[] => true,
		['e', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4047);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -still.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'n', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['n', 'e', 'h', 'c', 's', 'u', 'ä', 'm', 's', 'k', 'c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oll.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['v', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2851);
							s.is_empty()
						}
						['n', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', s @ ..] => {
					super::push(out, 1698);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 1699);
					match s {
						[] => true,
						['s', 'h', 'c', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', 'n', 'u', 'n', 'h', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word geblieben.
pub(crate) fn n6482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5239);
	s.is_empty()
}

/// The word dageblieben.
pub(crate) fn n6483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5240);
	s.is_empty()
}

/// The word zurückgeblieben.
pub(crate) fn n6484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5241);
	s.is_empty()
}

/// The word grossgeschrieben.
pub(crate) fn n6485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5242);
	s.is_empty()
}

/// Words ending -ieden.
pub(crate) fn n2153(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4054);
					s.is_empty()
				}
				['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 4055);
					s.is_empty()
				}
				['t', 'n', 'e', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 4056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2870);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eiden.
pub(crate) fn n2154(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2872);
			s.is_empty()
		}
		['r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2873);
			s.is_empty()
		}
		['s', 't', s @ ..] => {
			super::push(out, 2874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lden.
pub(crate) fn n1059(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nden.
pub(crate) fn n1060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 882);
	match s {
		[] => true,
		['a', 't', 's', 'e', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4058);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'e', s @ ..] => {
					super::push(out, 2879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fen.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 883);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['a', 'h', 'c', 's', 't', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2880);
							match s {
								[] => true,
								['h', 'c', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4835);
											s.is_empty()
										}
										['n', 'i', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4836);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2882);
							match s {
								[] => true,
								['g', 'e', 'g', 'b', s @ ..] => {
									super::push(out, 4060);
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
						['\u{1}', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4061);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2885);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4063);
									s.is_empty()
								}
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
			super::push(out, 885);
			match s {
				[] => true,
				['i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['o', 'h', 'e', 'b', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['r', 'o', 'w', 'r', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['a', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['r', 'r', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gen.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['l', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4064);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4837);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 4838);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4839);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4840);
											match s {
												[] => true,
												['e', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
													super::push(out, 5243);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['t', 'e', 'g', 'n', 'i', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['i', 't', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4065);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4066);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2890);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4841);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 4842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2891);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4843);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											super::push(out, 4844);
											s.is_empty()
										}
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
					super::push(out, 1738);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						['r', 'd', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2893);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2894);
							match s {
								[] => true,
								['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4068);
									s.is_empty()
								}
								['z', 'e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4069);
									s.is_empty()
								}
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
			super::push(out, 893);
			match s {
				[] => true,
				['h', 't', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				['l', 'r', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2897);
							s.is_empty()
						}
						['z', 'r', s @ ..] => {
							super::push(out, 2898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['w', 'e', 'g', 's', 'u', 'a', 'n', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2900);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4845);
											s.is_empty()
										}
										['s', 't', 'r', 'a', 'w', 'n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4846);
											s.is_empty()
										}
										['t', 'i', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 4847);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 4848);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5244);
													s.is_empty()
												}
												['u', '\u{1}', s @ ..] => {
													super::push(out, 5245);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						['r', 'e', 'l', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hen.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2905);
							match s {
								[] => true,
								['g', 'e', 'g', s @ ..] => {
									super::push(out, 4070);
									match s {
										[] => true,
										['s', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4071);
									match s {
										[] => true,
										['e', 'g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5246);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 5247);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 4850);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 4072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2908);
							match s {
								[] => true,
								['w', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4851);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4852);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['s', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4073);
									s.is_empty()
								}
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

/// Words ending -ken.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2916);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4853);
											s.is_empty()
										}
										['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'r', 'e', 'n', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4074);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4075);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				['u', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4076);
									s.is_empty()
								}
								['d', 'u', 'e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4077);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 4078);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4855);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'n', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4079);
									s.is_empty()
								}
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
			super::push(out, 899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -len.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['t', 's', 'e', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1754);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 4856);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4857);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['t', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'w', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2924);
							match s {
								[] => true,
								['c', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4858);
											s.is_empty()
										}
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						['r', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -men.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['m', 'o', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1756);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4860);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4081);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4862);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4863);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4864);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nen.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['a', 'l', 'l', 'e', 'z', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 902);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'k', 'c', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1759);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2931);
							s.is_empty()
						}
						['r', 'r', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 4082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'i', 'w', 'z', '\u{1}', s @ ..] => {
			super::push(out, 905);
			s.is_empty()
		}
		['u', 't', 't', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ren.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 907);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['a', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4866);
											s.is_empty()
										}
										['z', 'r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 4867);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 4083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4084);
									s.is_empty()
								}
								['s', 'u', 'a', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['c', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4086);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				['l', 'r', 'e', 'v', 't', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['r', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'u', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sen.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['p', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['r', 'e', 'v', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['c', 'a', 'w', 'h', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 915);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'r', 'u', 't', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 4868);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 4869);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5248);
													s.is_empty()
												}
												['n', 'e', s @ ..] => {
													super::push(out, 5249);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 5250);
													match s {
														[] => true,
														['u', 'a', '\u{1}', s @ ..] => super::t1::n6669(s, out),
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
									super::push(out, 4088);
									match s {
										[] => true,
										['e', 'z', s @ ..] => {
											super::push(out, 4870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4089);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 4871);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', 't', 'h', 'c', 'i', 'l', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2941);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4091);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'f', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4092);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4872);
											s.is_empty()
										}
										['x', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 4873);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 4094);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4874);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2945);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4875);
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4876);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						['h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4095);
									s.is_empty()
								}
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 5251);
													s.is_empty()
												}
												['g', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6670(s, out),
														['b', s @ ..] => super::t1::n6671(s, out),
														['n', s @ ..] => super::t1::n6672(s, out),
														['s', s @ ..] => super::t1::n6673(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 4877);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4878);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													super::push(out, 5252);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 4879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
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

/// Words ending -ten.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['r', 't', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4096);
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4097);
									s.is_empty()
								}
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
			super::push(out, 919);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['l', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4098);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4880);
											s.is_empty()
										}
										['g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5253);
													s.is_empty()
												}
												['n', 'u', s @ ..] => {
													super::push(out, 5254);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4099);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 5255);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 5256);
													s.is_empty()
												}
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
							super::push(out, 2955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 923);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['a', 'r', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['e', 'm', 'a', 'g', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 925);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['y', 'h', 't', 'e', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2956);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2957);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2958);
							match s {
								[] => true,
								['h', 'c', 's', 'e', 'g', s @ ..] => {
									super::push(out, 4100);
									match s {
										[] => true,
										['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4881);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4882);
											s.is_empty()
										}
										['m', 'u', '\u{1}', s @ ..] => {
											super::push(out, 4883);
											s.is_empty()
										}
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
					super::push(out, 1782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zen.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßen.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['i', 'e', 'h', 'e', s @ ..] => {
			super::push(out, 931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nehmbar.
pub(crate) fn n3785(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4124);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4125);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ähmbar.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2992);
	s.is_empty()
}

/// Words ending -mmbar.
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2993);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2994);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -formbar.
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2996);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nbar.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', 'f', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				['h', 'c', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4126);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1815);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1816);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4887);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4128);
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
				['e', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4129);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4889);
									s.is_empty()
								}
								['t', 'r', 'e', 'z', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pbar.
pub(crate) fn n1142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	s.is_empty()
}

/// Words ending -rbar.
pub(crate) fn n1143(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3000);
					match s {
						[] => true,
						['n', 'ä', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4891);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'ö', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						['ü', 'r', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4132);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4133);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4893);
									s.is_empty()
								}
								['m', 'a', 'r', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'f', s @ ..] => {
							super::push(out, 4134);
							s.is_empty()
						}
						['p', 'm', s @ ..] => {
							super::push(out, 4135);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 4895);
									s.is_empty()
								}
								['s', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4897);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4899);
									s.is_empty()
								}
								['u', 'd', 'n', s @ ..] => {
									super::push(out, 4900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'e', 't', 's', 'n', s @ ..] => {
					super::push(out, 3001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4136);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'f', 's', 'u', 'a', 'n', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1818);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1819);
			match s {
				[] => true,
				['t', 's', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sbar.
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				['n', 'u', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1821);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 3007);
					s.is_empty()
				}
				['u', 'l', 'f', 'n', 'i', 'e', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tbar.
pub(crate) fn n1145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	match s {
		[] => true,
		['e', 'r', 't', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3009);
					s.is_empty()
				}
				['r', 'e', 'v', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1822);
			s.is_empty()
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 'o', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3011);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4138);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3014);
					s.is_empty()
				}
				['r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4140);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1823);
			s.is_empty()
		}
		['l', 'a', 'h', 'c', 's', 'm', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1824);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'v', 'r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4142);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', s @ ..] => {
			super::push(out, 1825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -schaubar.
pub(crate) fn n1146(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1826);
			s.is_empty()
		}
		['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
			super::push(out, 1827);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zbar.
pub(crate) fn n1147(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'e', 'r', s @ ..] => {
			super::push(out, 1828);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1829);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1830);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4144);
							s.is_empty()
						}
						['s', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 3020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßbar.
pub(crate) fn n1148(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'i', 'n', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['i', 'e', 'r', s @ ..] => {
			super::push(out, 1832);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word evolutionär.
pub(crate) fn n6501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5257);
	s.is_empty()
}

/// The word revolutionär.
pub(crate) fn n6502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5258);
	s.is_empty()
}

/// The word inhaltslos.
pub(crate) fn n3881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3071);
	s.is_empty()
}

/// The word hauslos.
pub(crate) fn n2395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1899);
	s.is_empty()
}

/// Words ending -tlos.
pub(crate) fn n1217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1002);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			super::push(out, 1900);
			s.is_empty()
		}
		['f', 'i', s @ ..] => {
			super::push(out, 1901);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3073);
					match s {
						[] => true,
						['u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', s @ ..] => {
			match s {
				[] => true,
				['h', 'e', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3075);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 3080);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', s @ ..] => {
			super::push(out, 1902);
			s.is_empty()
		}
		['ß', 'u', 'w', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulos.
pub(crate) fn n1218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1003);
	s.is_empty()
}

/// The word alternativlos.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1004);
	s.is_empty()
}

/// The word affixlos.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	s.is_empty()
}

/// Words ending -zlos.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1904);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3082);
					s.is_empty()
				}
				['e', 'r', 'r', 'u', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 's', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3084);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word grußlos.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	s.is_empty()
}

/// Words ending -rtet.
pub(crate) fn n1281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1049);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				['w', 'r', 'e', 'n', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', 'h', 'e', 'g', 'b', s @ ..] => {
			super::push(out, 1946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stet.
pub(crate) fn n1282(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1947);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1948);
			s.is_empty()
		}
		['o', 'r', 'r', s @ ..] => {
			super::push(out, 1949);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1950);
			s.is_empty()
		}
		['ü', 'r', 't', s @ ..] => {
			super::push(out, 1951);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttet.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1952);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1953);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1954);
			match s {
				[] => true,
				['m', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['r', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ötet.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1051);
	match s {
		[] => true,
		['r', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1956);
			s.is_empty()
		}
		['t', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gsberechtigt.
pub(crate) fn n6503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5259);
	s.is_empty()
}

/// Words ending -hsberechtigt.
pub(crate) fn n6504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5260);
	s.is_empty()
}

/// Words ending -icht.
pub(crate) fn n1327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1089);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['e', 'k', 'c', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4191);
							s.is_empty()
						}
						['e', 'g', 'e', 'l', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4192);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['h', 'c', 's', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2001);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['k', 'n', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scht.
pub(crate) fn n1329(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
			super::push(out, 2005);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['m', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'ä', 'f', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2007);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['l', 'f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'w', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4195);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['i', 'n', 'k', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				['l', 'e', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				['f', 'p', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucht.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1090);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['r', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4197);
							s.is_empty()
						}
						['r', 'e', 'v', 'n', s @ ..] => {
							super::push(out, 4198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4199);
							s.is_empty()
						}
						['t', 'f', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4200);
							s.is_empty()
						}
						['u', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', 'e', 'v', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4202);
							s.is_empty()
						}
						['s', 'n', 'o', 'i', 't', 'i', 'n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'r', s @ ..] => {
			super::push(out, 2011);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4204);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							super::push(out, 4205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ächt.
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1091);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2014);
			s.is_empty()
		}
		['w', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oht.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	s.is_empty()
}

/// Words ending -sht.
pub(crate) fn n535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	s.is_empty()
}

/// Words ending -uht.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äht.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'b', 'e', 'g', 'f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1093);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1094);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word erhöht.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	s.is_empty()
}

/// The word abgebrüht.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	s.is_empty()
}

/// Words ending -eilt.
pub(crate) fn n1373(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 'e', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2064);
			s.is_empty()
		}
		['p', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2065);
			s.is_empty()
		}
		['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
			super::push(out, 2066);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['r', 'e', 't', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['g', 'n', 'a', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -llt.
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1126);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3206);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'e', 'b', 'l', 'h', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['t', 's', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3208);
							s.is_empty()
						}
						['g', 'r', 'e', 'h', 'c', s @ ..] => {
							super::push(out, 3209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['h', 'c', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				['w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['r', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3210);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3212);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3213);
							match s {
								[] => true,
								['e', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4220);
									s.is_empty()
								}
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

/// Words ending -ält.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['u', 'q', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -resistent.
pub(crate) fn n5281(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'k', 'i', 't', 'o', 'i', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4924);
			s.is_empty()
		}
		['e', 'i', 'p', 'a', 'r', 'e', 'h', 't', '\u{1}', s @ ..] => {
			super::push(out, 4925);
			s.is_empty()
		}
		['n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4926);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4927);
			s.is_empty()
		}
		['t', 'f', 'a', 's', 'n', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vent.
pub(crate) fn n1415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1154);
	s.is_empty()
}

/// Words ending -zent.
pub(crate) fn n1416(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'j', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2104);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2105);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2106);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hnt.
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1155);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2108);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'w', 'e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 1156);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1157);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 1158);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -int.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['e', 'm', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnt.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['k', 'e', s @ ..] => {
					super::push(out, 2112);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4226);
									s.is_empty()
								}
								['s', 't', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4227);
									s.is_empty()
								}
								['t', 'l', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2114);
					match s {
						[] => true,
						['s', 'e', 'g', 'n', 'u', s @ ..] => {
							super::push(out, 3235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2115);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'z', s @ ..] => {
									super::push(out, 4229);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4230);
									match s {
										[] => true,
										['e', 'v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5261);
													s.is_empty()
												}
												['n', s @ ..] => {
													match s {
														[] => true,
														['e', 'n', 'n', 'o', 's', '\u{1}', s @ ..] => super::t1::n6674(s, out),
														['r', 'i', 'h', '\u{1}', s @ ..] => super::t1::n6675(s, out),
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
		['i', 's', 'e', 'g', 'l', s @ ..] => {
			super::push(out, 1160);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1161);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 1162);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word unbetont.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	s.is_empty()
}

/// Words ending -rnt.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['k', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2116);
					s.is_empty()
				}
				['l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 't', 's', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		['ö', 'h', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unt.
pub(crate) fn n575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1165);
			match s {
				[] => true,
				['l', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				['h', 'c', 's', 't', 'e', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 1167);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word schneegekrönt.
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	s.is_empty()
}

/// Words ending -ert.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1178);
			match s {
				[] => true,
				['u', 'a', 'z', 'e', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['i', 'w', 'e', s @ ..] => {
					super::push(out, 2122);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4231);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4232);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 4233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2124);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3240);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													super::push(out, 5262);
													s.is_empty()
												}
												['g', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 5263);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3241);
							match s {
								[] => true,
								['w', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 3242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 2127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['f', 'e', s @ ..] => {
					super::push(out, 2128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1181);
			match s {
				[] => true,
				['a', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['ä', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 's', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1182);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4236);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5264);
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 5265);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 4930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3244);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ö', 'n', s @ ..] => {
							super::push(out, 3245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n1449(s, out),
		['k', s @ ..] => {
			super::push(out, 1184);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3286);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', 'e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4289);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['ä', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1190);
			match s {
				[] => true,
				['a', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				['h', 'c', 'ü', 'h', 'c', 's', 'e', 'g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2150);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4291);
									s.is_empty()
								}
								['r', 'e', 'b', s @ ..] => {
									super::push(out, 4292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						['i', 'e', 'g', 'e', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						['l', 'o', 'p', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2151);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'g', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1191);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2153);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3299);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3300);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4293);
									s.is_empty()
								}
								['s', 'e', 'g', 's', s @ ..] => {
									super::push(out, 4294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => super::t1::n1458(s, out),
		['z', s @ ..] => {
			super::push(out, 1193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ust.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['b', 'e', s @ ..] => {
			super::push(out, 1219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gelöst.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		['f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1221);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wüst.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	s.is_empty()
}

/// Words ending -ktiv.
pub(crate) fn n1558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1268);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				['g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 3365);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3367);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4324);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', 'n', 'u', 'r', 'e', 'm', 'm', 'ä', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3368);
					s.is_empty()
				}
				['t', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4326);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3370);
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 4328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'n', 'o', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4329);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3371);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3372);
					s.is_empty()
				}
				['s', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3373);
					s.is_empty()
				}
				['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2246);
			s.is_empty()
		}
		['n', 'i', 't', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2247);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4331);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4332);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4334);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eifend.
pub(crate) fn n2883(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4338);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4967);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4968);
							match s {
								[] => true,
								['ü', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', 't', 's', 'y', 's', '\u{1}', s @ ..] => {
											super::push(out, 5417);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 5418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 3400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pfend.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['i', 'h', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfend.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'w', 'm', s @ ..] => {
			super::push(out, 2276);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 2277);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3402);
					s.is_empty()
				}
				['h', 'c', 's', 'f', 'e', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 3403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ufend.
pub(crate) fn n1610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'n', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gend.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['h', 'c', 's', 'e', s @ ..] => {
							super::push(out, 3406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2279);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3407);
							s.is_empty()
						}
						['s', 'u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3408);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['d', 'i', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4339);
									s.is_empty()
								}
								['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4340);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4341);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 't', 'u', 'e', 'd', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2281);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3410);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4343);
									match s {
										[] => true,
										['h', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 4970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3411);
							s.is_empty()
						}
						['w', 'r', 'e', 'w', s @ ..] => {
							super::push(out, 3412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3413);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3414);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'r', 'f', 'e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4344);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4345);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3416);
							match s {
								[] => true,
								['e', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'h', 'c', 's', 's', 't', 'i', 'e', 'h', 'd', 'n', 'u', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2284);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4347);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4348);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4349);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2285);
					match s {
						[] => true,
						['u', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'ö', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3421);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 2288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1312);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', 'n', 'e', s @ ..] => {
									super::push(out, 4351);
									s.is_empty()
								}
								['d', 'h', s @ ..] => {
									super::push(out, 4352);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3424);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4354);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 'z', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1314);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['n', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hend.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1316);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4356);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3429);
							match s {
								[] => true,
								['r', 'l', 'h', s @ ..] => {
									super::push(out, 4358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 4971);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4972);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['d', 'n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => super::t1::n6678(s, out),
														['z', '\u{1}', s @ ..] => super::t1::n6679(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4973);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4974);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4975);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4976);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3431);
							match s {
								[] => true,
								['g', 's', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4359);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3432);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4362);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['f', 'p', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4364);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 4365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2295);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3434);
							s.is_empty()
						}
						['e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						['s', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3439);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3440);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4366);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3441);
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4368);
									s.is_empty()
								}
								['t', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2298);
					match s {
						[] => true,
						['z', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3443);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2300);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 3445);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 4370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 1319);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1320);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 1321);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iend.
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3446);
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4371);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kend.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3448);
							s.is_empty()
						}
						['m', 'h', 'c', 's', 'l', s @ ..] => {
							super::push(out, 3449);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3450);
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4978);
											s.is_empty()
										}
										['r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 4979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['l', 'b', 'k', 'c', 'ü', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3452);
							s.is_empty()
						}
						['t', 's', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3453);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3455);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3456);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['a', 'w', 'h', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3458);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3460);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lend.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3462);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 3464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 2320);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3465);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					super::push(out, 2321);
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
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4376);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4377);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4378);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4379);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4380);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3468);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 4980);
											s.is_empty()
										}
										['r', 'e', 'h', 'c', s @ ..] => {
											super::push(out, 4981);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 3469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3470);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2322);
					match s {
						[] => true,
						['w', 'l', s @ ..] => {
							super::push(out, 3472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'f', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3473);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1326);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 1327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mend.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1328);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3475);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3476);
							s.is_empty()
						}
						['l', 'i', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 3477);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3478);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 1330);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4382);
									s.is_empty()
								}
								['r', 'o', 'v', 'u', s @ ..] => {
									super::push(out, 4383);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 4982);
											s.is_empty()
										}
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
			super::push(out, 1331);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1332);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 1333);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nend.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1335);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				['e', 'd', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3480);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3481);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1336);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3482);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3483);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'ö', 't', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 1339);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['t', 'l', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pend.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 568);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1341);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1342);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rend.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	match s {
		[] => true,
		['a', 'p', 's', 't', s @ ..] => {
			super::push(out, 1344);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2337);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3484);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3485);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4386);
									s.is_empty()
								}
								['e', 'l', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4388);
									s.is_empty()
								}
								['o', 's', 'i', 'e', s @ ..] => {
									super::push(out, 4389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3487);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3488);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4390);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5274);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 5275);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 4983);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 4984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4391);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4392);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3489);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3490);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4985);
											s.is_empty()
										}
										['r', 'a', 't', s @ ..] => {
											super::push(out, 4986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3491);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 4394);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 3492);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4987);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4988);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4989);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3493);
							match s {
								[] => true,
								['u', 'd', 'e', s @ ..] => {
									super::push(out, 4397);
									s.is_empty()
								}
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
			super::push(out, 1346);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3494);
							s.is_empty()
						}
						['k', 'r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3496);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3497);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3499);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4399);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4400);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['b', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2343);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2344);
					s.is_empty()
				}
				['l', 'k', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', 't', 'e', s @ ..] => {
			super::push(out, 1348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -send.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['c', 'a', 'w', 'h', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3502);
							s.is_empty()
						}
						['w', 'e', s @ ..] => {
							super::push(out, 3503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2351);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3504);
							match s {
								[] => true,
								['m', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3505);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4403);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3506);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['i', 'l', 'f', s @ ..] => {
							super::push(out, 3507);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 2353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tend.
pub(crate) fn n687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 570);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'u', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						['u', 'e', 'l', 't', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'a', 'b', s @ ..] => {
							super::push(out, 3513);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4409);
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'e', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['d', 'e', 'b', 'h', 'c', 'i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				['v', 'r', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['w', 'k', 'n', 'i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uend.
pub(crate) fn n688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zend.
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1364);
			s.is_empty()
		}
		['n', 'ä', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1365);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3518);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßend.
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['a', 'm', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1366);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3520);
							match s {
								[] => true,
								['c', 's', 'r', s @ ..] => {
									super::push(out, 4411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4412);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4413);
									match s {
										[] => true,
										['c', 's', 's', s @ ..] => {
											super::push(out, 4990);
											s.is_empty()
										}
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
			super::push(out, 1368);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4414);
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4416);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 4991);
											s.is_empty()
										}
										['z', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5276);
													s.is_empty()
												}
												['z', 'r', 'e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 5277);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 't', 's', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1369);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1370);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word verallgemeinerungsfähig.
pub(crate) fn n6525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5278);
	s.is_empty()
}

/// The word verbesserungsfähig.
pub(crate) fn n6526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5279);
	s.is_empty()
}

/// Words ending -tionsfähig.
pub(crate) fn n5516(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', 'u', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5008);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tsfähig.
pub(crate) fn n4480(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 4431);
			s.is_empty()
		}
		['t', 'i', 'r', 't', 'n', 'i', 'e', 'r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tfähig.
pub(crate) fn n3116(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 3582);
			s.is_empty()
		}
		['o', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3583);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3584);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word beschlußfähig.
pub(crate) fn n3117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2460);
	s.is_empty()
}

/// Words ending -gehörig.
pub(crate) fn n1859(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2529);
			s.is_empty()
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 2530);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2531);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hellhörig.
pub(crate) fn n1860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1502);
	s.is_empty()
}

/// The word feinhörig.
pub(crate) fn n1861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1503);
	s.is_empty()
}

/// The word schwerhörig.
pub(crate) fn n1862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1504);
	s.is_empty()
}

/// Words ending -ürig.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	s.is_empty()
}

/// Words ending -flichtig.
pub(crate) fn n5539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4452);
	s.is_empty()
}

/// Words ending -richtig.
pub(crate) fn n4539(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 4453);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 4454);
			s.is_empty()
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 4455);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sichtig.
pub(crate) fn n4540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3633);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4456);
			s.is_empty()
		}
		['h', 'c', 'r', 'u', 'd', 'n', s @ ..] => {
			super::push(out, 4457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wichtig.
pub(crate) fn n4541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3634);
	match s {
		[] => true,
		['e', 'g', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5016);
					s.is_empty()
				}
				['r', 'h', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'n', 'e', 'b', 'e', 'l', 'r', s @ ..] => {
			super::push(out, 4458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rchtig.
pub(crate) fn n3225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	s.is_empty()
}

/// The word chüschtig.
pub(crate) fn n3226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	s.is_empty()
}

/// Words ending -uchtig.
pub(crate) fn n3227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3635);
			match s {
				[] => true,
				['l', 'h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ächtig.
pub(crate) fn n3228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2554);
	match s {
		[] => true,
		['d', 'r', s @ ..] => {
			super::push(out, 3636);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['d', 'r', 'o', 'k', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5018);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 5019);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5020);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3637);
			match s {
				[] => true,
				['h', 'c', 's', 'b', s @ ..] => {
					super::push(out, 4460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4462);
					s.is_empty()
				}
				['h', 'c', 'i', s @ ..] => {
					super::push(out, 4463);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4464);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							super::push(out, 5022);
							s.is_empty()
						}
						['d', 'n', 'u', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3638);
			match s {
				[] => true,
				['p', 'l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4465);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5024);
							s.is_empty()
						}
						['s', 't', 'h', s @ ..] => {
							super::push(out, 5025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üchtig.
pub(crate) fn n3229(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'f', s @ ..] => {
			match s {
				[] => true,
				['k', 'i', 'l', 'b', 'u', 'p', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4466);
					s.is_empty()
				}
				['l', 'l', 'a', 'f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 4467);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['b', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4469);
					s.is_empty()
				}
				['d', 'n', 'i', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 4470);
					s.is_empty()
				}
				['e', 'i', 'f', 'a', 'r', 'g', 'o', 'n', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4471);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5026);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4472);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 5028);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', s @ ..] => {
					super::push(out, 4473);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5280);
									s.is_empty()
								}
								['t', 't', 'e', 'l', 'b', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 5281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5282);
									s.is_empty()
								}
								['t', 'o', 'k', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4474);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4475);
					match s {
						[] => true,
						['l', 'l', 'a', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5031);
							s.is_empty()
						}
						['f', 'i', 'g', 'h', 'c', 's', 'u', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5032);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5033);
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
				['g', 'u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4476);
					s.is_empty()
				}
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['g', 'u', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						['r', 'h', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'h', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4477);
					s.is_empty()
				}
				['s', 'r', 'h', 'e', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 4478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 3639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itig.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 707);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 3640);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3641);
							match s {
								[] => true,
								['a', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4479);
									s.is_empty()
								}
								['e', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'j', s @ ..] => {
							super::push(out, 3642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3643);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltig.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2555);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2556);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3645);
							match s {
								[] => true,
								['i', 'd', s @ ..] => {
									super::push(out, 4481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'u', 'ä', 's', '\u{1}', s @ ..] => {
							super::push(out, 3646);
							s.is_empty()
						}
						['f', 'f', 'o', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 4482);
									s.is_empty()
								}
								['k', 'c', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 4483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3647);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3648);
							s.is_empty()
						}
						['k', 'i', 'n', 'e', 's', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3649);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['l', 'y', 'r', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4484);
									s.is_empty()
								}
								['o', 'h', 'o', 'k', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3650);
							match s {
								[] => true,
								['u', 'i', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5036);
											s.is_empty()
										}
										['l', 'l', 'y', s @ ..] => {
											super::push(out, 5037);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 5038);
											s.is_empty()
										}
										['n', 'o', s @ ..] => {
											super::push(out, 5039);
											s.is_empty()
										}
										['r', 't', 't', s @ ..] => {
											super::push(out, 5040);
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
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'o', 'l', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 5041);
											s.is_empty()
										}
										['n', 'i', 'h', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 5042);
											s.is_empty()
										}
										['s', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 5043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 't', 'o', 'k', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4486);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['m', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5044);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 5045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', 'd', 'b', 'y', 'l', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 5046);
											s.is_empty()
										}
										['k', 'c', 'u', 'z', '\u{1}', s @ ..] => {
											super::push(out, 5047);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', 'b', 'y', 'l', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5048);
											s.is_empty()
										}
										['n', 'e', 's', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5049);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4489);
									s.is_empty()
								}
								['r', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4490);
									s.is_empty()
								}
								['t', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'e', 'g', 'l', 'e', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1518);
			s.is_empty()
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['g', 'r', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3652);
							s.is_empty()
						}
						['l', 'e', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3653);
							s.is_empty()
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3655);
							s.is_empty()
						}
						['n', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 3656);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3657);
							s.is_empty()
						}
						['t', 't', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'e', 'i', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'g', s @ ..] => {
			match s {
				[] => true,
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				['r', 'e', 't', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word samtig.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 708);
	s.is_empty()
}

/// Words ending -ntig.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 709);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['k', 't', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1520);
			s.is_empty()
		}
		['u', 't', '\u{1}', s @ ..] => {
			super::push(out, 1521);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zotig.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 710);
	s.is_empty()
}

/// Words ending -rtig.
pub(crate) fn n852(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2563);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2564);
					s.is_empty()
				}
				['d', 'm', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				['e', 'd', 'n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				['f', 'n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2568);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3659);
							s.is_empty()
						}
						['u', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['h', 'o', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3662);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4492);
									s.is_empty()
								}
								['d', 'e', 'i', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4493);
									s.is_empty()
								}
								['g', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4494);
									s.is_empty()
								}
								['l', 'u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4495);
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'f', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 'a', 'c', 'c', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 2571);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3667);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4497);
									s.is_empty()
								}
								['l', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'd', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3668);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4499);
									s.is_empty()
								}
								['u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3669);
							s.is_empty()
						}
						['ö', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3671);
							match s {
								[] => true,
								['l', 'f', 't', 'n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3672);
							s.is_empty()
						}
						['t', 'i', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3674);
							s.is_empty()
						}
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3675);
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
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3676);
							s.is_empty()
						}
						['d', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3677);
							s.is_empty()
						}
						['g', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3678);
							s.is_empty()
						}
						['r', 'e', 'g', 'n', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3679);
							s.is_empty()
						}
						['s', 'h', 'c', 'u', 'a', 'r', 'b', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3680);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4502);
									s.is_empty()
								}
								['s', 'n', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3681);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3682);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3683);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3684);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4504);
									s.is_empty()
								}
								['h', 'ö', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3685);
							s.is_empty()
						}
						['u', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3686);
							s.is_empty()
						}
						['z', 'n', 'e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1522);
			s.is_empty()
		}
		['u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1523);
			s.is_empty()
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3688);
							s.is_empty()
						}
						['n', 'e', 'g', 'e', 'g', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3689);
							s.is_empty()
						}
						['r', 'e', 'd', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stig.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 711);
	match s {
		[] => true,
		['a', 'l', 'k', s @ ..] => {
			super::push(out, 1525);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4506);
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 4507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
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
				['ü', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['u', 'l', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3691);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'o', s @ ..] => {
					super::push(out, 2581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2582);
					s.is_empty()
				}
				['r', 'b', 'l', s @ ..] => {
					super::push(out, 2583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttig.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1527);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2584);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2585);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1528);
			s.is_empty()
		}
		['o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1529);
			s.is_empty()
		}
		['u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utig.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1531);
			s.is_empty()
		}
		['e', 'd', s @ ..] => {
			match s {
				[] => true,
				['i', 'e', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2587);
					s.is_empty()
				}
				['n', 'i', 'e', 'n', s @ ..] => {
					super::push(out, 2588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1532);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['h', 'n', s @ ..] => {
					super::push(out, 2589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ätig.
pub(crate) fn n856(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2590);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['d', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2592);
					s.is_empty()
				}
				['l', 'h', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2593);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2595);
					s.is_empty()
				}
				['t', 'l', 'a', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ötig.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1534);
			s.is_empty()
		}
		['r', 'h', 'c', 's', 'r', 'e', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ütig.
pub(crate) fn n858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2597);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1536);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 2599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2600);
					match s {
						[] => true,
						['i', 'e', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1538);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lich.
pub(crate) fn n888(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2625);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4515);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 4516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'r', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4517);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4519);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', 'r', 'h', 'c', 's', 't', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2626);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2627);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 3710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2628);
					s.is_empty()
				}
				['ä', 'n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'h', 'c', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3711);
							s.is_empty()
						}
						['s', 'l', 'e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				['g', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2631);
					s.is_empty()
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3713);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 3714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2632);
					match s {
						[] => true,
						['i', 'b', 'r', s @ ..] => {
							super::push(out, 3715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3716);
							match s {
								[] => true,
								['g', 'u', s @ ..] => {
									super::push(out, 4521);
									match s {
										[] => true,
										['j', 's', 'f', 'u', 'r', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5050);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3717);
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 5051);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5284);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 5285);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'g', 'n', 'i', s @ ..] => {
											super::push(out, 5052);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 4522);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5053);
											s.is_empty()
										}
										['p', 'm', 'e', s @ ..] => {
											match s {
												[] => true,
												['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 5286);
													s.is_empty()
												}
												['n', 'u', 's', 'h', 'c', 'u', 'r', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5287);
													s.is_empty()
												}
												['r', 'u', 't', 'a', 'r', 'e', 'p', 'm', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 5288);
													s.is_empty()
												}
												['s', 'h', 'c', 'u', 'r', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5289);
													s.is_empty()
												}
												['t', 'h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 5290);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 5054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4523);
									match s {
										[] => true,
										['r', 'f', s @ ..] => {
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 5291);
													s.is_empty()
												}
												['s', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 5292);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 4524);
									match s {
										[] => true,
										['s', 'k', s @ ..] => {
											super::push(out, 5055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 3718);
							match s {
								[] => true,
								['t', 's', 'n', 'e', 'g', 'e', 'g', 's', 'g', 'n', 'u', 'l', 't', 't', 'i', 'm', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 3719);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'ö', 'h', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2633);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 2634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1561);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2635);
					s.is_empty()
				}
				['f', 'e', 'r', 't', 'r', 'e', s @ ..] => {
					super::push(out, 2636);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3720);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2637);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2638);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3722);
							s.is_empty()
						}
						['ä', s @ ..] => super::t1::n4652(s, out),
						_ => false,
					}
				}
				['ä', 'r', 'g', s @ ..] => {
					super::push(out, 2639);
					s.is_empty()
				}
				['ö', 'h', 'c', s @ ..] => {
					super::push(out, 2640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n1949(s, out),
		['h', s @ ..] => super::t1::n1950(s, out),
		['i', 'e', s @ ..] => super::t1::n1951(s, out),
		['k', s @ ..] => super::t1::n1952(s, out),
		['m', s @ ..] => super::t1::n1953(s, out),
		['n', s @ ..] => super::t1::n1954(s, out),
		['r', s @ ..] => super::t1::n1955(s, out),
		['s', s @ ..] => super::t1::n1956(s, out),
		['t', s @ ..] => super::t2::n1957(s, out),
		['u', s @ ..] => super::t2::n1958(s, out),
		['v', 'r', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1571);
			s.is_empty()
		}
		['z', s @ ..] => super::t2::n1960(s, out),
		['ß', s @ ..] => super::t2::n1961(s, out),
		_ => false,
	}
}

/// Words ending -isch.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				['s', 'o', 'r', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1585);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3794);
							s.is_empty()
						}
						['r', 'k', s @ ..] => {
							super::push(out, 3795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2692);
					match s {
						[] => true,
						['h', 'p', 'o', 'r', s @ ..] => {
							super::push(out, 3796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'ä', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1586);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'm', 'y', 's', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3797);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2695);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2696);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3800);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3801);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3802);
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 3803);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'k', s @ ..] => {
							super::push(out, 3805);
							s.is_empty()
						}
						['ä', 'l', s @ ..] => {
							match s {
								[] => true,
								['d', 'm', s @ ..] => {
									super::push(out, 4591);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4592);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4593);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
													super::push(out, 5314);
													s.is_empty()
												}
												['t', 'a', s @ ..] => {
													super::push(out, 5315);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 5107);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 's', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 3806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2698);
					match s {
						[] => true,
						['g', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3807);
							s.is_empty()
						}
						['h', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3808);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3809);
							s.is_empty()
						}
						['l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3810);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3811);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 3813);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4595);
									s.is_empty()
								}
								['r', 'e', 'b', 'ü', s @ ..] => {
									super::push(out, 4596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 2700);
					s.is_empty()
				}
				['ü', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 't', s @ ..] => {
					super::push(out, 2702);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 2703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2704);
					match s {
						[] => true,
						['r', 'g', 'o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4597);
									s.is_empty()
								}
								['n', 'r', s @ ..] => {
									super::push(out, 4598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2705);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2706);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4599);
									s.is_empty()
								}
								['e', 'p', 's', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'a', 'r', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 5108);
											s.is_empty()
										}
										['n', 'e', 'n', 'ä', 'm', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 5109);
											s.is_empty()
										}
										['r', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 5110);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 5316);
													s.is_empty()
												}
												['t', 'h', 'c', 'e', 'l', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5317);
													s.is_empty()
												}
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
					super::push(out, 2707);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 2708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n1980(s, out),
		['h', s @ ..] => super::t2::n1981(s, out),
		['i', s @ ..] => {
			super::push(out, 1588);
			match s {
				[] => true,
				['a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2717);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 'u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3834);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t2::n1983(s, out),
		['l', s @ ..] => super::t2::n1984(s, out),
		['m', s @ ..] => super::t2::n1985(s, out),
		['n', s @ ..] => super::t2::n1986(s, out),
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 2751);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n1988(s, out),
		['r', s @ ..] => super::t2::n1989(s, out),
		['s', s @ ..] => super::t2::n1990(s, out),
		['t', s @ ..] => super::t2::n1991(s, out),
		['u', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n3537(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => super::t2::n1993(s, out),
		['w', s @ ..] => super::t2::n1994(s, out),
		['x', 'o', 't', s @ ..] => super::t2::n1995(s, out),
		['z', s @ ..] => super::t2::n1996(s, out),
		['ß', s @ ..] => super::t2::n1997(s, out),
		['ä', 'p', 'o', 'r', s @ ..] => super::t2::n1998(s, out),
		['ö', 'r', s @ ..] => super::t2::n1999(s, out),
		_ => false,
	}
}

/// The word ausgelassen.
pub(crate) fn n6669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5410);
	s.is_empty()
}

/// The word geschlossen.
pub(crate) fn n6670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5411);
	s.is_empty()
}

/// Words ending -bgeschlossen.
pub(crate) fn n6671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5412);
	s.is_empty()
}

/// Words ending -ngeschlossen.
pub(crate) fn n6672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5413);
	s.is_empty()
}

/// Words ending -sgeschlossen.
pub(crate) fn n6673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5414);
	s.is_empty()
}

/// The word sonnenverbrannt.
pub(crate) fn n6674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5415);
	s.is_empty()
}

/// The word hirnverbrannt.
pub(crate) fn n6675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5416);
	s.is_empty()
}

/// Words ending -iert.
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3246);
					s.is_empty()
				}
				['i', 'z', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3247);
					s.is_empty()
				}
				['n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3248);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3249);
					s.is_empty()
				}
				['r', 'a', 't', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4238);
							s.is_empty()
						}
						['l', 'i', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['m', 'k', 'c', 'a', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2136);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3252);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 4242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4243);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4931);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4932);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', 'r', 't', 's', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2137);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3254);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 3256);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3257);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['m', 'i', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', 'a', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						['o', 'r', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3259);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['k', 'i', 'd', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4249);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['c', 's', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', 't', 'u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4934);
									s.is_empty()
								}
								['g', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4935);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 'p', 'e', s @ ..] => {
					super::push(out, 3261);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3262);
					match s {
						[] => true,
						['a', 'r', 'g', 'o', 'r', 'p', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4251);
							s.is_empty()
						}
						['o', 'n', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4252);
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
				['g', s @ ..] => {
					super::push(out, 3263);
					match s {
						[] => true,
						['i', 's', 'e', 'd', s @ ..] => {
							super::push(out, 4253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4254);
							s.is_empty()
						}
						['d', 'r', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						['f', 'f', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						['l', 'p', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4258);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3264);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4937);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5266);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 5267);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', s @ ..] => {
									super::push(out, 4938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4260);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4939);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4940);
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
						['a', 'h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4261);
							s.is_empty()
						}
						['e', 't', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4262);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2139);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2140);
			match s {
				[] => true,
				['a', 'k', 'n', s @ ..] => {
					super::push(out, 3265);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				['o', 'b', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['t', 'n', 'e', 'z', 'n', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3269);
					match s {
						[] => true,
						['s', 'n', 'e', 'z', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4267);
							s.is_empty()
						}
						['z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n4113(s, out),
				['i', s @ ..] => super::t2::n4114(s, out),
				['r', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['s', s @ ..] => super::t2::n4116(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t2::n2706(s, out),
		['u', s @ ..] => super::t2::n2707(s, out),
		['v', s @ ..] => super::t2::n2708(s, out),
		['w', s @ ..] => {
			super::push(out, 2143);
			s.is_empty()
		}
		['z', s @ ..] => super::t2::n2710(s, out),
		_ => false,
	}
}

/// Words ending -wert.
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1192);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2155);
			s.is_empty()
		}
		['h', 'c', 's', 'r', s @ ..] => {
			super::push(out, 2156);
			s.is_empty()
		}
		['n', 'u', s @ ..] => {
			super::push(out, 2157);
			s.is_empty()
		}
		['s', 'n', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					super::push(out, 3301);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['h', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word auseinanderbrechend.
pub(crate) fn n6678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5419);
	s.is_empty()
}

/// The word zerbrechend.
pub(crate) fn n6679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5420);
	s.is_empty()
}

/// Words ending -äuflich.
pub(crate) fn n4652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3723);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5056);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -glich.
pub(crate) fn n1949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	match s {
		[] => true,
		['a', 'r', 't', s @ ..] => {
			super::push(out, 2641);
			s.is_empty()
		}
		['e', 'i', 'r', 'k', 'n', 'e', 'h', 'c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2642);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3724);
					s.is_empty()
				}
				['w', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2643);
			match s {
				[] => true,
				['i', 'r', 'b', s @ ..] => {
					super::push(out, 3726);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['m', 'u', 'l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5060);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', 'r', 'p', 's', 'r', 'u', 'h', s @ ..] => {
					super::push(out, 3727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'z', 't', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2645);
			s.is_empty()
		}
		['u', 'a', 't', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3728);
					s.is_empty()
				}
				['n', 'u', 't', s @ ..] => {
					super::push(out, 3729);
					s.is_empty()
				}
				['s', 'r', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4528);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 5063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3732);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['m', 'n', s @ ..] => {
					super::push(out, 3733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'z', 'e', s @ ..] => {
			super::push(out, 2647);
			match s {
				[] => true,
				['b', 't', s @ ..] => {
					super::push(out, 3734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hlich.
pub(crate) fn n1950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1564);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3735);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4531);
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3736);
					s.is_empty()
				}
				['i', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3737);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3738);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3739);
					match s {
						[] => true,
						['n', 'e', 'm', 't', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'ä', 'r', 'b', 'e', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						['m', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4535);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'b', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5065);
									s.is_empty()
								}
								['r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 5066);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 3741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2648);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2649);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2650);
			s.is_empty()
		}
		['t', 'ö', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		['ä', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2652);
			s.is_empty()
		}
		['ö', 'r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3742);
					s.is_empty()
				}
				['t', 'h', 'c', 'u', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eilich.
pub(crate) fn n1951(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2653);
			s.is_empty()
		}
		['z', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2654);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -klich.
pub(crate) fn n1952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1565);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3744);
					s.is_empty()
				}
				['i', 'l', s @ ..] => {
					super::push(out, 3745);
					s.is_empty()
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4537);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5293);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 5294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4538);
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
				['a', s @ ..] => {
					super::push(out, 3746);
					s.is_empty()
				}
				['e', 'd', 'r', 'o', s @ ..] => {
					super::push(out, 3747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'w', s @ ..] => {
			super::push(out, 2655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mlich.
pub(crate) fn n1953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2656);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3748);
					s.is_empty()
				}
				['e', 'n', 'r', 'e', 'v', 'n', s @ ..] => {
					super::push(out, 3749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2657);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3750);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 2659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nlich.
pub(crate) fn n1954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2660);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 3752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4539);
							s.is_empty()
						}
						['d', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						['l', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 's', 'n', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5295);
											s.is_empty()
										}
										['l', 'u', 'ä', 's', '\u{1}', s @ ..] => {
											super::push(out, 5296);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 5068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5069);
									s.is_empty()
								}
								['e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 5070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', 'w', 'e', 'g', 'r', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', 'n', s @ ..] => {
					super::push(out, 3754);
					s.is_empty()
				}
				['r', 'n', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', 's', 'r', 'e', 'ß', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				['ä', 'm', 'n', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'ä', 'r', 'b', 't', s @ ..] => {
			super::push(out, 2661);
			s.is_empty()
		}
		['ü', 'r', 'g', 'b', s @ ..] => {
			super::push(out, 2662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlich.
pub(crate) fn n1955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3758);
					match s {
						[] => true,
						['n', 'ä', 'b', s @ ..] => {
							super::push(out, 4544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4545);
							s.is_empty()
						}
						['r', 'ü', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5071);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'n', 'i', 't', 'n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 4547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3759);
					s.is_empty()
				}
				['n', 'n', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				['p', 'r', s @ ..] => {
					super::push(out, 3761);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4548);
							s.is_empty()
						}
						['s', 'u', 'ä', '\u{1}', s @ ..] => {
							super::push(out, 4549);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5073);
									s.is_empty()
								}
								['e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5074);
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
						['h', s @ ..] => {
							super::push(out, 4550);
							match s {
								[] => true,
								['c', 'r', s @ ..] => {
									super::push(out, 5075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4551);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4552);
							match s {
								[] => true,
								['a', 'l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5076);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', s @ ..] => {
									super::push(out, 5077);
									s.is_empty()
								}
								['ö', 'l', s @ ..] => {
									super::push(out, 5078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4553);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 4554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4555);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 4556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2663);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4557);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4558);
							s.is_empty()
						}
						['g', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4559);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 3763);
					match s {
						[] => true,
						['f', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5079);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5080);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 4561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2664);
			s.is_empty()
		}
		['u', 'o', 't', 'r', 'e', 'ß', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2665);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 2666);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 2667);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 2668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -slich.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				['w', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'ö', 'b', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 2669);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2670);
			match s {
				[] => true,
				['a', 'm', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3767);
					s.is_empty()
				}
				['e', 'i', 'l', 'h', 'c', 's', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3768);
					s.is_empty()
				}
				['i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5082);
									s.is_empty()
								}
								['t', 't', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2671);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4563);
							s.is_empty()
						}
						['r', 'e', 's', s @ ..] => {
							super::push(out, 4564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

