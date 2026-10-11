use flexion::Path;

/// Words ending -lia.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4442);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['m', 'o', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['n', 'e', 'c', 'i', s @ ..] => {
							super::push(out, 4444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['e', 'n', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						['r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2314);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4447);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4448);
							match s {
								[] => true,
								['n', 'e', 'g', 'o', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['b', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2320);
					match s {
						[] => true,
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2321);
					s.is_empty()
				}
				['p', 'o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['r', 'u', 'a', 't', 'e', 'v', 'o', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				['s', 'n', 'e', 't', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 817);
			s.is_empty()
		}
		['u', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mia.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2327);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 820);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						['n', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['p', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['c', 'o', 'r', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['r', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2335);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2336);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4454);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 6474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2337);
					s.is_empty()
				}
				['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2339);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nia.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['h', 'p', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2342);
					match s {
						[] => true,
						['m', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2343);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2344);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['z', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2347);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['h', 'c', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				['m', 'o', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2353);
					s.is_empty()
				}
				['m', 'o', 'n', 'g', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['o', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['d', 'i', 'l', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['g', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2360);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['e', 'g', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4459);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4460);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6475);
									match s {
										[] => true,
										['c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8043);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8044);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 8968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6476);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6477);
									match s {
										[] => true,
										['c', 'n', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 8045);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8046);
											match s {
												[] => true,
												['i', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 8969);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'm', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['t', 'c', 'e', 't', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		['s', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pia.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['r', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 's', '\u{1}', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2368);
					match s {
						[] => true,
						['u', 'n', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2369);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['t', 'o', 'l', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ria.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4464);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4465);
							match s {
								[] => true,
								['t', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				['f', 'r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2377);
					match s {
						[] => true,
						['u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2378);
					match s {
						[] => true,
						['c', 'i', 't', 's', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4468);
							s.is_empty()
						}
						['n', 'o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4469);
							s.is_empty()
						}
						['t', 'i', 't', 's', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2379);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4471);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4472);
							match s {
								[] => true,
								['o', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4473);
							match s {
								[] => true,
								['c', 'a', 'n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'r', 'i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2380);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2381);
					match s {
						[] => true,
						['a', 'p', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4474);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2383);
					match s {
						[] => true,
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2384);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2385);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4477);
							match s {
								[] => true,
								['l', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6482);
									s.is_empty()
								}
								['r', 'c', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 6483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4478);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2386);
					s.is_empty()
				}
				['v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 840);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2388);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2389);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2390);
					match s {
						[] => true,
						['n', 'i', 'n', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2391);
					match s {
						[] => true,
						['p', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2394);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4483);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4484);
							s.is_empty()
						}
						['n', 'e', 's', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4485);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4486);
							s.is_empty()
						}
						['s', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2397);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2398);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2400);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['a', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2404);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4489);
							match s {
								[] => true,
								['r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4490);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['i', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8047);
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4491);
							s.is_empty()
						}
						['s', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['i', 'h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4493);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4494);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'i', '\u{1}', s @ ..] => {
											super::push(out, 8049);
											s.is_empty()
										}
										['l', 'o', 'd', 'i', '\u{1}', s @ ..] => {
											super::push(out, 8050);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 4495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4496);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6485);
									match s {
										[] => true,
										['n', 'o', 'g', 'i', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8051);
											s.is_empty()
										}
										['s', 'i', '\u{1}', s @ ..] => super::t1::n8186(s, out),
										['t', 'a', 'm', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => super::t1::n8187(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n2435(s, out),
				['s', s @ ..] => super::t1::n2436(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n849(s, out),
		_ => false,
	}
}

/// Words ending -sia.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4500);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2418);
					match s {
						[] => true,
						['t', 's', 'e', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2422);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6486);
									s.is_empty()
								}
								['o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6487);
									s.is_empty()
								}
								['t', 'c', 'y', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6488);
									s.is_empty()
								}
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
			super::push(out, 851);
			match s {
				[] => true,
				['e', 'l', 'a', 't', 'a', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'v', 'o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 852);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2425);
					s.is_empty()
				}
				['r', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tia.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2428);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['c', 'o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4506);
							s.is_empty()
						}
						['i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4508);
							s.is_empty()
						}
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2434);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2435);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4510);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4513);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['a', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4514);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4515);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 6491);
									s.is_empty()
								}
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4516);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4517);
							s.is_empty()
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2439);
					match s {
						[] => true,
						['i', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4519);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4520);
							s.is_empty()
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4521);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4522);
							match s {
								[] => true,
								['i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 6493);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'q', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2440);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2441);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n861(s, out),
		['o', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['m', 'o', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2445);
					s.is_empty()
				}
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2447);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['e', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2450);
					match s {
						[] => true,
						['n', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2451);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4549);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4550);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4551);
							s.is_empty()
						}
						['n', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2454);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2455);
					match s {
						[] => true,
						['s', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -quia.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['l', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -via.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				['n', 'g', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2459);
					s.is_empty()
				}
				['r', 'g', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				['t', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['c', 's', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word apoplexia.
pub(crate) fn n871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 867);
	s.is_empty()
}

/// Words ending -illa.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 884);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['a', 't', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4574);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4575);
					match s {
						[] => true,
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2486);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 4576);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2487);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2488);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2489);
			match s {
				[] => true,
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2490);
			match s {
				[] => true,
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2491);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4579);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4581);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2492);
			match s {
				[] => true,
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4586);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ola.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['e', 'l', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['b', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						['d', 'u', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4589);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4591);
							match s {
								[] => true,
								['o', 'n', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 6543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4592);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4593);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4594);
							match s {
								[] => true,
								['a', 'r', 'b', 'm', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				['s', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				['l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['m', 'e', 'd', 'n', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2510);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4597);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6546);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8993);
													s.is_empty()
												}
												['e', '\u{1}', s @ ..] => {
													super::push(out, 8994);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 't', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4598);
							s.is_empty()
						}
						['o', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'l', 'c', 'c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2513);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4600);
							s.is_empty()
						}
						['s', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['s', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ula.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2516);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 4602);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2517);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2518);
					match s {
						[] => true,
						['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4604);
							s.is_empty()
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2519);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2520);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4606);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 4607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2521);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4608);
							s.is_empty()
						}
						['n', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4609);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4611);
							match s {
								[] => true,
								['e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2522);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4613);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4614);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6549);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 8099);
											s.is_empty()
										}
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8100);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 6550);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 8101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4615);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4616);
							match s {
								[] => true,
								['t', 'o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6551);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4617);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4618);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4619);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n2553(s, out),
				['n', s @ ..] => super::t1::n2554(s, out),
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2526);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2527);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4632);
							match s {
								[] => true,
								['t', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4633);
							match s {
								[] => true,
								['r', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 6582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2528);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2529);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4634);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4636);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => super::t1::n2559(s, out),
				['e', s @ ..] => super::t1::n2560(s, out),
				['n', s @ ..] => super::t1::n2561(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t1::n902(s, out),
		['g', s @ ..] => super::t1::n903(s, out),
		['h', s @ ..] => super::t1::n904(s, out),
		['l', s @ ..] => super::t1::n905(s, out),
		['m', s @ ..] => super::t1::n906(s, out),
		['n', s @ ..] => super::t1::n907(s, out),
		['p', s @ ..] => super::t2::n908(s, out),
		['r', s @ ..] => super::t2::n909(s, out),
		['s', s @ ..] => super::t2::n910(s, out),
		['t', s @ ..] => super::t2::n911(s, out),
		['v', s @ ..] => super::t2::n912(s, out),
		['x', s @ ..] => super::t2::n913(s, out),
		_ => false,
	}
}

/// Words ending -yma.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['c', 'n', 'e', 'r', 'h', 'p', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 2613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				['h', 'c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'a', '\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ficina.
pub(crate) fn n4746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4688);
	s.is_empty()
}

/// The word vicina.
pub(crate) fn n4747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4689);
	s.is_empty()
}

/// The word runcina.
pub(crate) fn n2671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2640);
	s.is_empty()
}

/// The word sarcina.
pub(crate) fn n2672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2641);
	s.is_empty()
}

/// Words ending -scina.
pub(crate) fn n2673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2642);
	match s {
		[] => true,
		['i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4690);
			s.is_empty()
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucina.
pub(crate) fn n2674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2643);
	s.is_empty()
}

/// Words ending -dina.
pub(crate) fn n991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	match s {
		[] => true,
		['n', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2645);
			match s {
				[] => true,
				['f', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6596);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6597);
							match s {
								[] => true,
								['e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8127);
									s.is_empty()
								}
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

/// Words ending -gina.
pub(crate) fn n992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 985);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4692);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2647);
			s.is_empty()
		}
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2648);
			s.is_empty()
		}
		['r', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hina.
pub(crate) fn n993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['c', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2650);
			s.is_empty()
		}
		['t', 'n', 'i', 'b', 'e', 'r', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lina.
pub(crate) fn n994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 987);
	match s {
		[] => true,
		['e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mina.
pub(crate) fn n995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2653);
			match s {
				[] => true,
				['l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2654);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 4695);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2655);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['e', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nina.
pub(crate) fn n996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 989);
	s.is_empty()
}

/// The word heroina.
pub(crate) fn n997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	s.is_empty()
}

/// Words ending -pina.
pub(crate) fn n998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rina.
pub(crate) fn n999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2660);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2661);
			match s {
				[] => true,
				['o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2662);
			match s {
				[] => true,
				['p', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 4700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2663);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4701);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6598);
							s.is_empty()
						}
						['r', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4702);
					match s {
						[] => true,
						['n', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 6600);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8128);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 2664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sina.
pub(crate) fn n1000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 993);
	match s {
		[] => true,
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2665);
			s.is_empty()
		}
		['y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tina.
pub(crate) fn n1001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 994);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2667);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 4704);
					s.is_empty()
				}
				['u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 4705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2668);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2669);
			match s {
				[] => true,
				['e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4706);
					s.is_empty()
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2670);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2671);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 4708);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4709);
					match s {
						[] => true,
						['e', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uina.
pub(crate) fn n1002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	s.is_empty()
}

/// Words ending -vina.
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	s.is_empty()
}

/// Words ending -lna.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	s.is_empty()
}

/// Words ending -mna.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nna.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['e', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ona.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1000);
			s.is_empty()
		}
		['d', 'a', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['t', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rna.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2674);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4710);
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2675);
					s.is_empty()
				}
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2677);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4712);
							s.is_empty()
						}
						['i', 's', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4713);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4715);
							match s {
								[] => true,
								['u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6603);
									s.is_empty()
								}
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
			super::push(out, 1004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -una.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	s.is_empty()
}

/// Words ending -yna.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stra.
pub(crate) fn n1066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1056);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2716);
			match s {
				[] => true,
				['i', 'l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				['r', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2717);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 4728);
					s.is_empty()
				}
				['h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2718);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2719);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4730);
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2720);
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utra.
pub(crate) fn n1067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1057);
	s.is_empty()
}

/// Words ending -ura.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1058);
			s.is_empty()
		}
		['e', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1059);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2722);
					match s {
						[] => true,
						['c', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4733);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4734);
							s.is_empty()
						}
						['v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2723);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4736);
							match s {
								[] => true,
								['m', 'i', 'r', 'g', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2724);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2725);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2726);
					match s {
						[] => true,
						['a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2727);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4739);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4740);
							match s {
								[] => true,
								['v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['c', 'n', 'e', 'm', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 6606);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6607);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 8130);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 6608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4742);
							match s {
								[] => true,
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6610);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4744);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2728);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4745);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4746);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 6614);
									match s {
										[] => true,
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6615);
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8132);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6616);
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 8133);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4747);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4748);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6617);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4749);
							match s {
								[] => true,
								['d', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 6619);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6620);
									match s {
										[] => true,
										['t', 's', s @ ..] => {
											match s {
												[] => true,
												['a', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9003);
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9004);
													s.is_empty()
												}
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
					super::push(out, 2729);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2730);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4751);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4752);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6621);
									match s {
										[] => true,
										['g', 'o', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4753);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2731);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2732);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2733);
					match s {
						[] => true,
						['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2734);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2735);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4757);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4758);
							match s {
								[] => true,
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2736);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4759);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 1065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yra.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['l', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nta.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['h', 'p', 'o', 'c', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 2795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2796);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2797);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2798);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4783);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 8135);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 8136);
											match s {
												[] => true,
												['c', 's', '\u{1}', s @ ..] => {
													super::push(out, 9005);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ota.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['m', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1128);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 1129);
			s.is_empty()
		}
		['p', 's', 'e', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2800);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 1132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pta.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1134);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1135);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['r', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rta.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sta.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['i', 's', 'u', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				['h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2807);
					s.is_empty()
				}
				['r', 'h', 'c', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1141);
			match s {
				[] => true,
				['g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2809);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2811);
					match s {
						[] => true,
						['i', 'v', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2812);
					match s {
						[] => true,
						['e', 'h', 'c', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4786);
							s.is_empty()
						}
						['o', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 4787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4788);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 6625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['a', 'n', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4790);
							s.is_empty()
						}
						['u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2815);
					match s {
						[] => true,
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tta.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['e', 's', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uta.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1144);
			s.is_empty()
		}
		['e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1145);
			s.is_empty()
		}
		['l', 'o', 's', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1146);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xta.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	s.is_empty()
}

/// Words ending -yta.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['o', 'c', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 1148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eum.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2919);
					match s {
						[] => true,
						['e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 4822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1281);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2921);
					match s {
						[] => true,
						['n', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2922);
					s.is_empty()
				}
				['y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1284);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 2927);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2928);
					match s {
						[] => true,
						['d', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4824);
							s.is_empty()
						}
						['s', 'u', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2930);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4827);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4828);
							match s {
								[] => true,
								['l', 'u', 'm', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 6631);
									s.is_empty()
								}
								['s', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2931);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 4829);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2932);
					match s {
						[] => true,
						['l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 4831);
							s.is_empty()
						}
						['t', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['o', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2934);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2935);
					s.is_empty()
				}
				['y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1291);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['a', 'm', 'e', 'n', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gum.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2938);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hum.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2940);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 6633);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6634);
									match s {
										[] => true,
										['r', 'i', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'z', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2941);
					s.is_empty()
				}
				['y', 'z', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2943);
					match s {
						[] => true,
						['i', 'f', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2944);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4835);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						['p', 'r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2945);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ium.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2947);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2948);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 6635);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 6636);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 8140);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9006);
													s.is_empty()
												}
												['r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 9007);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8141);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2949);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4839);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6637);
									s.is_empty()
								}
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
			super::push(out, 1299);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2950);
					match s {
						[] => true,
						['c', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4840);
							s.is_empty()
						}
						['d', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4841);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4842);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 6638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4843);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4844);
							match s {
								[] => true,
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4845);
							match s {
								[] => true,
								['s', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 4846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2951);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2952);
					match s {
						[] => true,
						['a', 'n', 'y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4847);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4848);
							s.is_empty()
						}
						['m', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2953);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4850);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6641);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4851);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6643);
									match s {
										[] => true,
										['n', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 8142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6644);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8143);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 8144);
											match s {
												[] => true,
												['o', '\u{1}', s @ ..] => {
													super::push(out, 9008);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 8145);
											match s {
												[] => true,
												['c', 'n', 'a', 's', '\u{1}', s @ ..] => {
													super::push(out, 9009);
													s.is_empty()
												}
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
							super::push(out, 4852);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6645);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6646);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4853);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6648);
									s.is_empty()
								}
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6649);
									s.is_empty()
								}
								['o', 'r', 't', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4854);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6651);
									match s {
										[] => true,
										['a', 's', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8146);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4855);
							match s {
								[] => true,
								['t', 'e', 'r', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4856);
							s.is_empty()
						}
						['t', 'a', 'r', 'e', 'n', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4857);
							s.is_empty()
						}
						['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2954);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2955);
					match s {
						[] => true,
						['a', 'm', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n1314(s, out),
		['g', s @ ..] => super::t2::n1315(s, out),
		['h', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2971);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4900);
							match s {
								[] => true,
								['n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6679);
									s.is_empty()
								}
								['r', s @ ..] => super::t2::n6783(s, out),
								_ => false,
							}
						}
						['i', 't', 's', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => super::t2::n4964(s, out),
						['r', 'a', '\u{1}', s @ ..] => super::t2::n4965(s, out),
						['y', s @ ..] => super::t2::n4966(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t2::n3003(s, out),
				['t', s @ ..] => super::t2::n3004(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n1317(s, out),
		['m', s @ ..] => super::t2::n1318(s, out),
		['n', s @ ..] => super::t2::n1319(s, out),
		['p', s @ ..] => super::t2::n1320(s, out),
		['r', s @ ..] => super::t2::n1321(s, out),
		['s', s @ ..] => super::t2::n1322(s, out),
		['t', s @ ..] => super::t2::n1323(s, out),
		['u', s @ ..] => super::t2::n1324(s, out),
		['v', s @ ..] => super::t2::n1325(s, out),
		['x', s @ ..] => super::t2::n1326(s, out),
		_ => false,
	}
}

/// Words ending -lum.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3027);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3028);
					match s {
						[] => true,
						['m', 'y', 'c', 'i', 'v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3030);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1314);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3033);
					match s {
						[] => true,
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3035);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3038);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3039);
					match s {
						[] => true,
						['a', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3041);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3042);
					s.is_empty()
				}
				['g', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3043);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5058);
							s.is_empty()
						}
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'g', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3047);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 5060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3048);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5061);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									super::push(out, 6818);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5062);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6820);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6821);
									match s {
										[] => true,
										['a', 'v', '\u{1}', s @ ..] => {
											super::push(out, 8249);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5063);
							match s {
								[] => true,
								['i', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6822);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6823);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6824);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 8250);
											s.is_empty()
										}
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
					super::push(out, 3049);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5064);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6825);
									match s {
										[] => true,
										['n', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8251);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 8252);
											match s {
												[] => true,
												['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 9050);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6826);
									s.is_empty()
								}
								['s', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5065);
							s.is_empty()
						}
						['l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5066);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5067);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5068);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6828);
									s.is_empty()
								}
								['c', 'n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6829);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6830);
									match s {
										[] => true,
										['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8253);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5069);
							s.is_empty()
						}
						['x', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3050);
					match s {
						[] => true,
						['c', 'o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5072);
							s.is_empty()
						}
						['c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3053);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3054);
					s.is_empty()
				}
				['i', s @ ..] => super::t2::n3087(s, out),
				['r', 'e', 't', 's', 'e', 'l', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1320);
			s.is_empty()
		}
		['u', s @ ..] => super::t2::n1335(s, out),
		['y', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mum.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3067);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5106);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3069);
					s.is_empty()
				}
				['c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3070);
					s.is_empty()
				}
				['r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['a', 'r', 'g', 'o', 'l', 'e', 'l', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['n', 'o', 'z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', 'o', 'r', 'o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1328);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -num.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				['c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3080);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3081);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5108);
							s.is_empty()
						}
						['i', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['t', 'e', s @ ..] => {
							super::push(out, 5110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6876);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'n', 'e', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5112);
							match s {
								[] => true,
								['o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5113);
							match s {
								[] => true,
								['y', 't', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				['m', 'o', 'n', 'e', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3092);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3093);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				['x', 'o', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3097);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', 'f', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				['g', 's', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5120);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8297);
											s.is_empty()
										}
										['u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 8298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'a', 't', 's', 'y', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5121);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						['y', 'x', '\u{1}', s @ ..] => {
							super::push(out, 5123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3103);
					match s {
						[] => true,
						['i', 'c', 'r', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5124);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5125);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5127);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5128);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5129);
							match s {
								[] => true,
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5130);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 6882);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 8299);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3107);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5131);
							s.is_empty()
						}
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5132);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5133);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5134);
							match s {
								[] => true,
								['t', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'o', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1334);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1335);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5135);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3113);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5137);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['b', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['s', 'o', 'm', 'e', 'n', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pum.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1340);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['s', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rum.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1343);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3117);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6887);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6888);
									s.is_empty()
								}
								['e', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5141);
							match s {
								[] => true,
								['u', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['i', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3121);
					match s {
						[] => true,
						['l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5143);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['l', 'o', 'v', s @ ..] => {
							super::push(out, 5145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['g', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['h', 't', 'o', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				['l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3131);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5146);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1348);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3134);
					match s {
						[] => true,
						['a', 'l', 'u', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5149);
							match s {
								[] => true,
								['b', 'a', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6891);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', 'i', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['h', 'p', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3139);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 6893);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['p', 's', 'o', 't', 't', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1354);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3142);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5151);
							s.is_empty()
						}
						['n', 'i', 'd', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5152);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5153);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6895);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 8300);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3143);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5154);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6897);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 8301);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 6898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3144);
					match s {
						[] => true,
						['m', 'o', 'i', 'l', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3145);
					match s {
						[] => true,
						['l', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 5156);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3149);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5158);
							match s {
								[] => true,
								['l', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6899);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 6900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5159);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5160);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 6901);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8302);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5161);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6903);
									s.is_empty()
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5163);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6905);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6906);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => super::t2::n8442(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n3183(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n1369(s, out),
		['y', s @ ..] => super::t2::n1370(s, out),
		_ => false,
	}
}

/// Words ending -sum.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				['p', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3163);
					match s {
						[] => true,
						['o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1360);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'u', 'q', 'a', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1363);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1364);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tum.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['b', 'b', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5166);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5167);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5168);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6907);
									match s {
										[] => true,
										['u', 'i', '\u{1}', s @ ..] => {
											super::push(out, 8304);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6908);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8305);
											s.is_empty()
										}
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
					super::push(out, 3168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['h', 'p', 'a', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3171);
					match s {
						[] => true,
						['h', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5170);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3172);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', 'a', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6909);
									s.is_empty()
								}
								['s', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3174);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5172);
							match s {
								[] => true,
								['r', 'g', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8306);
											s.is_empty()
										}
										['o', 'l', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8307);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['v', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['f', 'r', 'e', 'p', 'm', 'a', 'u', 'q', 's', 'u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5175);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5176);
							s.is_empty()
						}
						['m', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5177);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5178);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6911);
									s.is_empty()
								}
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5180);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5182);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 6914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5183);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5184);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['d', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5186);
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5188);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 6916);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6917);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6918);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5189);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 6920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['l', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3187);
					match s {
						[] => true,
						['u', 'p', s @ ..] => {
							super::push(out, 5191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3188);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 5192);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5193);
							s.is_empty()
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n3222(s, out),
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3190);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5198);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t2::n3225(s, out),
				['s', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				['u', s @ ..] => super::t2::n3227(s, out),
				['v', 'i', 'l', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3195);
					s.is_empty()
				}
				['x', s @ ..] => super::t2::n3229(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n1384(s, out),
		['l', s @ ..] => super::t2::n1385(s, out),
		['n', s @ ..] => super::t2::n1386(s, out),
		['o', s @ ..] => super::t2::n1387(s, out),
		['p', s @ ..] => super::t2::n1388(s, out),
		['r', s @ ..] => super::t2::n1389(s, out),
		['s', s @ ..] => super::t2::n1390(s, out),
		['u', s @ ..] => super::t2::n1391(s, out),
		['y', s @ ..] => super::t2::n1392(s, out),
		_ => false,
	}
}

/// Words ending -uum.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1379);
			match s {
				[] => true,
				['s', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['r', 't', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 5232);
							s.is_empty()
						}
						['v', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1381);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1382);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 3226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vum.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1384);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3230);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5234);
							match s {
								[] => true,
								['n', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6948);
									s.is_empty()
								}
								['r', 't', 's', 'n', 'o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 1386);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xum.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1388);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mon.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['a', 'g', 'n', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1437);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1438);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'h', 't', 'h', 'p', 'o', 'r', 'u', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 1440);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1441);
			match s {
				[] => true,
				['n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				['z', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1442);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1443);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1444);
			match s {
				[] => true,
				['n', 'o', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -non.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1445);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1446);
			match s {
				[] => true,
				['h', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3302);
					s.is_empty()
				}
				['l', 'p', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3303);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['p', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1448);
			match s {
				[] => true,
				['g', 'y', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3306);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3307);
					match s {
						[] => true,
						['c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ron.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1450);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'e', 'd', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6963);
									s.is_empty()
								}
								['s', 'o', 'c', 'i', '\u{1}', s @ ..] => {
									super::push(out, 6964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5277);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 't', 'e', 'g', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 1452);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3311);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6965);
									s.is_empty()
								}
								['o', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6966);
									s.is_empty()
								}
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
			super::push(out, 1454);
			match s {
				[] => true,
				['p', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -son.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['i', 'b', '\u{1}', s @ ..] => {
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

/// Words ending -ton.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1457);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['e', 't', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['g', 'o', 'm', 'a', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 1460);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1461);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['e', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 5279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3318);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xon.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yon.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1463);
			s.is_empty()
		}
		['h', 'p', 'i', 't', '\u{1}', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		['r', 'u', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zon.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5280);
							s.is_empty()
						}
						['o', 'n', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['t', 'a', 'p', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['n', 'g', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sio.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				['t', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3382);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1534);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3384);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5308);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5309);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 6994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3385);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5310);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3386);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5312);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 5313);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3387);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 6995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5316);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1536);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5318);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5319);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6996);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5320);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 6998);
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6999);
									s.is_empty()
								}
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
			super::push(out, 1537);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3389);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5321);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7000);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8366);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 9119);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 9120);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 8367);
											match s {
												[] => true,
												['o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 9121);
													s.is_empty()
												}
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
							super::push(out, 5322);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5323);
							match s {
								[] => true,
								['e', 'r', 'p', 'e', s @ ..] => {
									super::push(out, 7002);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 8368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5324);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5325);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5326);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7004);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['l', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', 'l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5328);
							s.is_empty()
						}
						['p', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3393);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5330);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5331);
							match s {
								[] => true,
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5332);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7007);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7008);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7009);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8369);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7010);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3394);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7012);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 8370);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t2::n1558(s, out),
		['u', s @ ..] => super::t2::n1559(s, out),
		_ => false,
	}
}

/// Words ending -tio.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['a', s @ ..] => super::t2::n1560(s, out),
		['c', s @ ..] => {
			super::push(out, 1544);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5468);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5469);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5470);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5471);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5472);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 7257);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5473);
							match s {
								[] => true,
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5474);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7259);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7260);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8574);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7261);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8575);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9212);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8576);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 5475);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 5476);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3425);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5478);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7262);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 8577);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 7263);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7264);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5479);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7266);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7267);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7268);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 8578);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7269);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5480);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7271);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7272);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7273);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5481);
							match s {
								[] => true,
								['s', s @ ..] => super::t2::n7390(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t2::n5555(s, out),
						['s', s @ ..] => {
							super::push(out, 5483);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t2::n5557(s, out),
						['v', s @ ..] => super::t2::n5558(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n3460(s, out),
				['n', s @ ..] => super::t2::n3461(s, out),
				['o', s @ ..] => super::t2::n3462(s, out),
				['u', s @ ..] => super::t2::n3463(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n1562(s, out),
		['i', s @ ..] => super::t2::n1563(s, out),
		['l', s @ ..] => super::t2::n1564(s, out),
		['n', s @ ..] => super::t2::n1565(s, out),
		['o', s @ ..] => super::t2::n1566(s, out),
		['p', s @ ..] => super::t2::n1567(s, out),
		['r', s @ ..] => super::t2::n1568(s, out),
		['s', s @ ..] => super::t2::n1569(s, out),
		['u', s @ ..] => super::t2::n1570(s, out),
		['x', s @ ..] => super::t2::n1571(s, out),
		_ => false,
	}
}

/// Words ending -ressor.
pub(crate) fn n5670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5589);
	match s {
		[] => true,
		['g', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 7366);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7367);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -issor.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3539);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 7369);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7370);
					match s {
						[] => true,
						['r', 'p', 'e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ussor.
pub(crate) fn n3577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3540);
	match s {
		[] => true,
		['i', 'e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5591);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usor.
pub(crate) fn n1683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 5592);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tor.
pub(crate) fn n517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['a', s @ ..] => super::t2::n1684(s, out),
		['c', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3557);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5683);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5684);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 5685);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5686);
							match s {
								[] => true,
								['t', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5687);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7469);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 8674);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5688);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7471);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5689);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7473);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5690);
							match s {
								[] => true,
								['s', 'n', s @ ..] => {
									super::push(out, 7475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5691);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5692);
							match s {
								[] => true,
								['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7477);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3559);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5694);
							match s {
								[] => true,
								['a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5695);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7480);
									s.is_empty()
								}
								['l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5696);
							s.is_empty()
						}
						['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3560);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5698);
							match s {
								[] => true,
								['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3561);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5699);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5700);
							match s {
								[] => true,
								['s', 'i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3562);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5701);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7485);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7486);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 8675);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7487);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5702);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 8676);
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 8677);
											s.is_empty()
										}
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
			super::push(out, 1668);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['r', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3565);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3566);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5705);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5706);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				['d', s @ ..] => super::t2::n3606(s, out),
				['g', s @ ..] => super::t2::n3607(s, out),
				['l', s @ ..] => {
					super::push(out, 3570);
					s.is_empty()
				}
				['n', s @ ..] => super::t2::n3609(s, out),
				['p', 'u', 'c', '\u{1}', s @ ..] => super::t2::n3610(s, out),
				['r', s @ ..] => super::t2::n3611(s, out),
				['s', s @ ..] => super::t2::n3612(s, out),
				['t', s @ ..] => super::t2::n3613(s, out),
				['v', s @ ..] => super::t2::n3614(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n1688(s, out),
		['n', s @ ..] => super::t2::n1689(s, out),
		['o', s @ ..] => super::t2::n1690(s, out),
		['p', s @ ..] => super::t2::n1691(s, out),
		['r', s @ ..] => super::t2::n1692(s, out),
		['s', s @ ..] => super::t2::n1693(s, out),
		['u', s @ ..] => super::t2::n1694(s, out),
		_ => false,
	}
}

/// Words ending -uor.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	s.is_empty()
}

/// Words ending -vor.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1678);
			s.is_empty()
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 1679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xor.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	s.is_empty()
}

/// Words ending -tas.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'm', 'o', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5747);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5748);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5749);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7511);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 8681);
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 7512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n1730(s, out),
		['l', s @ ..] => {
			super::push(out, 1712);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['u', 'l', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 3630);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ties.
pub(crate) fn n1767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1748);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3657);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5831);
					match s {
						[] => true,
						['i', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5832);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5833);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5834);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7630);
							s.is_empty()
						}
						['g', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 7631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5835);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7632);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7633);
							s.is_empty()
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5836);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7635);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5837);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5838);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7637);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['s', 'r', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -luvies.
pub(crate) fn n3699(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 5840);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5841);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5842);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -les.
pub(crate) fn n555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				['c', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				['i', 't', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3661);
					s.is_empty()
				}
				['s', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1752);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 5844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 3664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1754);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3666);
					s.is_empty()
				}
				['d', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1757);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mes.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3668);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['e', 's', 'r', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1762);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nes.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1763);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				['m', 'o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['g', 'i', 'r', 'o', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1766);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3677);
					match s {
						[] => true,
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3678);
					s.is_empty()
				}
				['i', 'r', 'e', 'g', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pes.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 556);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1767);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5847);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3682);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3684);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1770);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1772);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1773);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3688);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5850);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 5851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -res.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 557);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5852);
							s.is_empty()
						}
						['a', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3693);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['o', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1779);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['l', 'o', 'l', 'o', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'o', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				['i', 'm', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3698);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ses.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1781);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				['l', 'i', 'p', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3702);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['c', 'o', 'e', 'h', 'p', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', 'g', 'y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1785);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['o', 'c', 'o', 'n', 'e', 'h', 'r', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				['p', 'o', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['b', 'a', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['a', 'p', 'o', 'e', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5857);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['e', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						['y', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 5860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3711);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['e', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5861);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5862);
							match s {
								[] => true,
								['c', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7639);
									s.is_empty()
								}
								['h', 'c', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'o', 's', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3713);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3714);
					match s {
						[] => true,
						['e', 'd', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5863);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5864);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3715);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5866);
							match s {
								[] => true,
								['p', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1788);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1789);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3716);
					match s {
						[] => true,
						['l', 'e', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1790);
			match s {
				[] => true,
				['i', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3719);
					match s {
						[] => true,
						['i', 's', 'u', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3720);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3721);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3722);
					match s {
						[] => true,
						['n', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1792);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3724);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ues.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 1793);
			match s {
				[] => true,
				['r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 3725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word dives.
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	s.is_empty()
}

/// Words ending -nis.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3750);
					s.is_empty()
				}
				['i', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1821);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1823);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1824);
			match s {
				[] => true,
				['e', 'l', 'u', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['o', 'c', 'i', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pis.
pub(crate) fn n575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 1829);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				['b', 'a', 'n', 'n', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3758);
					s.is_empty()
				}
				['c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3759);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				['h', 't', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3761);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3763);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 7652);
									s.is_empty()
								}
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				['p', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3767);
					s.is_empty()
				}
				['m', 'e', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3768);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				['t', 'p', 'o', 'y', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['a', 'e', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3771);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3776);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3777);
					s.is_empty()
				}
				['l', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3778);
					s.is_empty()
				}
				['t', 'i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1836);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1837);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3780);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['s', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1840);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3783);
					match s {
						[] => true,
						['r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3784);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5884);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1841);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3787);
					s.is_empty()
				}
				['g', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3789);
					match s {
						[] => true,
						['c', 'e', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5886);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5887);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3790);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3791);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7655);
									match s {
										[] => true,
										['o', 'o', '\u{1}', s @ ..] => {
											super::push(out, 8766);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3792);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5889);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1843);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1844);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5891);
							match s {
								[] => true,
								['l', 'e', 'f', 'e', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5892);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						['u', 'n', 'a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1845);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3796);
					s.is_empty()
				}
				['e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3797);
					s.is_empty()
				}
				['l', 'u', 'c', 'r', 'e', 'b', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 3798);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5895);
							match s {
								[] => true,
								['l', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8767);
											s.is_empty()
										}
										['o', 'i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8768);
											s.is_empty()
										}
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
			super::push(out, 1846);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3800);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['l', 'l', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 5896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3802);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['u', 'n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tis.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['i', 'n', 'o', 'm', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3805);
					s.is_empty()
				}
				['m', 'e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3806);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3807);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1850);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1852);
			match s {
				[] => true,
				['b', 'a', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3810);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3811);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5898);
							match s {
								[] => true,
								['r', 'e', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', 'r', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['i', 'n', 'e', 'o', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5900);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['i', 'n', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5902);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5903);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3813);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3814);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3815);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5904);
							match s {
								[] => true,
								['r', 'a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3817);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5907);
							match s {
								[] => true,
								['t', 'a', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5908);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5909);
							match s {
								[] => true,
								['l', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5910);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3819);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						['s', 'e', 'a', 'h', 'p', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1853);
			match s {
				[] => true,
				['e', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1854);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1855);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3822);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 5915);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5916);
							match s {
								[] => true,
								['p', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3823);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 5918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uis.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['g', 'n', 'a', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vis.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1860);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3828);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3829);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3830);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1862);
			match s {
				[] => true,
				['r', 'b', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 1865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xis.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1868);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1869);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -icus.
pub(crate) fn n1980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1961);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3913);
			match s {
				[] => true,
				['m', 'e', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3914);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5943);
					match s {
						[] => true,
						['r', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5944);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3915);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5945);
					match s {
						[] => true,
						['l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7672);
							s.is_empty()
						}
						['n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3916);
			match s {
				[] => true,
				['b', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5946);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7674);
							s.is_empty()
						}
						['c', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7675);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 7676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3917);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5950);
					s.is_empty()
				}
				['e', 'd', 'a', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5951);
					s.is_empty()
				}
				['i', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5952);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5953);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 7678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3918);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5954);
					match s {
						[] => true,
						['h', 'c', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 5955);
					s.is_empty()
				}
				['i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5956);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5957);
					match s {
						[] => true,
						['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5958);
					s.is_empty()
				}
				['u', 't', 'n', 'i', 's', 's', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 3919);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 3920);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3921);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5960);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 7681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'm', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5961);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5962);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5963);
					match s {
						[] => true,
						['l', 'a', 'd', 'n', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7682);
							s.is_empty()
						}
						['p', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5965);
					match s {
						[] => true,
						['c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7684);
							s.is_empty()
						}
						['e', 'm', 'o', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7685);
							s.is_empty()
						}
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5966);
					match s {
						[] => true,
						['g', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3922);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3923);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7688);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7689);
							match s {
								[] => true,
								['e', 'h', 't', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5969);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5970);
					match s {
						[] => true,
						['i', 'r', 'e', 't', 'c', 'a', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7690);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'n', 'o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 3924);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iscus.
pub(crate) fn n3976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3931);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5974);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5975);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5976);
			s.is_empty()
		}
		['m', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5977);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5978);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7692);
					s.is_empty()
				}
				['e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7693);
					s.is_empty()
				}
				['m', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		['t', 'n', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5980);
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 5981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucus.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1967);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3933);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3934);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 3935);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3936);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 5982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dus.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1969);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1970);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3937);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3938);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3939);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3940);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3941);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3942);
					match s {
						[] => true,
						['f', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1971);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3943);
					s.is_empty()
				}
				['l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 3944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1972);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3946);
					match s {
						[] => true,
						['i', 's', 'r', 'e', 'v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['a', 'p', 'o', 'l', 'e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1975);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3951);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3952);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eus.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3953);
					s.is_empty()
				}
				['c', 'o', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3954);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3955);
					s.is_empty()
				}
				['l', 'i', 'l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3956);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3957);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5987);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5988);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3958);
					s.is_empty()
				}
				['s', 'i', 'r', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1977);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3960);
					match s {
						[] => true,
						['b', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5989);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5990);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 7696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'd', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3962);
					s.is_empty()
				}
				['n', 'o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 3963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1980);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1981);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['e', 'b', 'o', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3964);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5991);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5992);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5993);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3966);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3967);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3968);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3969);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5996);
							match s {
								[] => true,
								['a', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5997);
							match s {
								[] => true,
								['c', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 't', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1984);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1985);
			match s {
				[] => true,
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3971);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3972);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1986);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gus.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1989);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3974);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3975);
					match s {
						[] => true,
						['c', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5999);
							s.is_empty()
						}
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', 'o', 'r', 'h', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7700);
									s.is_empty()
								}
								['s', 'e', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3976);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6000);
							s.is_empty()
						}
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3977);
					s.is_empty()
				}
				['v', 'o', 'r', 'y', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3979);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7702);
									match s {
										[] => true,
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8770);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 'l', 'a', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 3980);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3981);
					s.is_empty()
				}
				['t', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 3982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1991);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1992);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['i', 'r', 'a', s @ ..] => {
					super::push(out, 3983);
					match s {
						[] => true,
						['h', 't', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3984);
					match s {
						[] => true,
						['a', 'd', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3985);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6004);
							match s {
								[] => true,
								['e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7704);
									s.is_empty()
								}
								['t', 'e', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6005);
							match s {
								[] => true,
								['h', 't', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7706);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7707);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7708);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 8771);
											s.is_empty()
										}
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
			super::push(out, 1995);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3986);
					match s {
						[] => true,
						['g', 'y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6006);
							s.is_empty()
						}
						['h', 't', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3987);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3988);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6008);
							s.is_empty()
						}
						['r', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6009);
							s.is_empty()
						}
						['t', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6010);
							s.is_empty()
						}
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
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1996);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3989);
					match s {
						[] => true,
						['r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3990);
					match s {
						[] => true,
						['y', 'h', 'r', 'o', 'h', 't', 'i', 'n', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3991);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3992);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7709);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7710);
									s.is_empty()
								}
								['r', 'e', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7711);
									s.is_empty()
								}
								['t', 'n', 'o', 'c', 'e', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3993);
					match s {
						[] => true,
						['d', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6013);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6014);
							s.is_empty()
						}
						['t', 'p', 'e', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 6015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3994);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6016);
							s.is_empty()
						}
						['r', 'g', 'o', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7713);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7714);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7715);
									match s {
										[] => true,
										['o', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8772);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'z', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3995);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3996);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3997);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6018);
							s.is_empty()
						}
						['z', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4000);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4001);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4002);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7716);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7717);
									match s {
										[] => true,
										['y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8773);
											s.is_empty()
										}
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
					super::push(out, 4003);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 6020);
							match s {
								[] => true,
								['e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4004);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['c', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ius.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2000);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4006);
					s.is_empty()
				}
				['r', 'e', 'v', 'i', 'n', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['a', 'n', 'i', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4008);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['l', 'a', 'n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6022);
							s.is_empty()
						}
						['n', 'u', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6023);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6024);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'd', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 6026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['s', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2003);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4014);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6029);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7720);
									s.is_empty()
								}
								['i', 'u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4016);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['b', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4018);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['a', 'r', 'g', 'g', 'r', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['r', 'i', 'v', 'i', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2007);
			match s {
				[] => true,
				['p', 'a', 'r', 'g', 'r', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4022);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4024);
					match s {
						[] => true,
						['d', 'n', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6032);
							s.is_empty()
						}
						['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2009);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4027);
					match s {
						[] => true,
						['c', 'r', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4028);
					match s {
						[] => true,
						['l', 'l', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['d', 'e', 'c', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6036);
							s.is_empty()
						}
						['m', 'e', 'a', 'd', 'e', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2011);
			s.is_empty()
		}
		['r', s @ ..] => super::t2::n2031(s, out),
		['s', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4037);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['t', 'i', 'd', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6060);
							s.is_empty()
						}
						['l', 'c', 'i', 't', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6061);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6062);
							match s {
								[] => true,
								['g', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4042);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lus.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4044);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['i', 't', 's', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6063);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4046);
					match s {
						[] => true,
						['r', 'a', 'p', 'o', 'l', 'e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4047);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4048);
					match s {
						[] => true,
						['y', 'h', 't', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 6066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6069);
							s.is_empty()
						}
						['o', 'r', 'c', 'o', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 4052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['a', 'n', 'u', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4053);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4054);
					match s {
						[] => true,
						['n', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2018);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4056);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4057);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['a', 'c', 'o', 'i', 'h', 't', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4059);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 4060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2019);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4061);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4062);
					match s {
						[] => true,
						['n', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6073);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7785);
									s.is_empty()
								}
								['h', 'p', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4064);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4066);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4067);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4068);
					match s {
						[] => true,
						['y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['b', 'i', 'r', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6077);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 6078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4070);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6079);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7787);
									match s {
										[] => true,
										['d', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8844);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6080);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6081);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7789);
									s.is_empty()
								}
								['i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6082);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6083);
							match s {
								[] => true,
								['a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6084);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7793);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7794);
									match s {
										[] => true,
										['r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 8845);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 7795);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7796);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8846);
											s.is_empty()
										}
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
					super::push(out, 4071);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6085);
							match s {
								[] => true,
								['i', 'd', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6086);
							match s {
								[] => true,
								['o', 'c', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6087);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6088);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7799);
									match s {
										[] => true,
										['c', 'i', 'r', 't', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8847);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6089);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => super::t2::n7927(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t2::n6184(s, out),
						['v', s @ ..] => super::t2::n6185(s, out),
						['x', s @ ..] => super::t2::n6186(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n4118(s, out),
				['y', s @ ..] => super::t2::n4119(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n2040(s, out),
		['u', s @ ..] => super::t2::n2041(s, out),
		['y', s @ ..] => super::t2::n2042(s, out),
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2024);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 4096);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4098);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4099);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4100);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2027);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4102);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 6140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 4103);
					s.is_empty()
				}
				['l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4104);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4105);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4106);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4107);
					match s {
						[] => true,
						['i', 'g', 'e', 'l', 'l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2029);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4109);
					match s {
						[] => true,
						['a', 'r', 'o', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4110);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4111);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4112);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6143);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 7845);
									s.is_empty()
								}
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
			super::push(out, 2032);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['i', 's', 'u', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 4114);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4115);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6145);
							match s {
								[] => true,
								['d', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6146);
							match s {
								[] => true,
								['a', 't', 'o', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'e', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6147);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6148);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7848);
									match s {
										[] => true,
										['i', 'c', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 8880);
											s.is_empty()
										}
										['r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8881);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'l', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6149);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7850);
									match s {
										[] => true,
										['g', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8882);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7851);
									match s {
										[] => true,
										['h', 'p', 'y', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6150);
							match s {
								[] => true,
								['a', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6151);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6152);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7854);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 8884);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', 'a', 'h', 'p', 'l', 'a', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7855);
									s.is_empty()
								}
								['i', 'l', 'o', 'p', 'o', 'm', 's', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4116);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 4118);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 4119);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4120);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4121);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4123);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4124);
					s.is_empty()
				}
				['n', 'o', 'i', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6155);
							s.is_empty()
						}
						['e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 6156);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6157);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8885);
											s.is_empty()
										}
										['c', '\u{1}', s @ ..] => {
											super::push(out, 8886);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'r', 'p', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 6158);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4127);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6160);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6161);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7857);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4128);
					match s {
						[] => true,
						['r', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4129);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4130);
					match s {
						[] => true,
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4131);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4132);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6166);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4133);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6168);
							match s {
								[] => true,
								['i', 'f', 'i', 'l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 7859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 't', 's', 'i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6169);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4134);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6171);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6172);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6173);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 7860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6174);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 6175);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6176);
							match s {
								[] => true,
								['s', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6177);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6178);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6179);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4136);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4137);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4138);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6181);
							match s {
								[] => true,
								['m', 'o', 'h', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6182);
							s.is_empty()
						}
						['n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4140);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4141);
					match s {
						[] => true,
						['i', 'z', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 4142);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4143);
					s.is_empty()
				}
				['l', 'i', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4144);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4145);
					match s {
						[] => true,
						['u', 'h', 'c', 'e', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4146);
					s.is_empty()
				}
				['r', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 4147);
					s.is_empty()
				}
				['t', 'a', 'p', 'o', 'r', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['i', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4151);
					match s {
						[] => true,
						['a', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6186);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6187);
							match s {
								[] => true,
								['m', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7865);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4152);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4153);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						['p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n4200(s, out),
				['m', s @ ..] => super::t2::n4201(s, out),
				['n', s @ ..] => {
					super::push(out, 4156);
					s.is_empty()
				}
				['p', s @ ..] => super::t2::n4203(s, out),
				['r', s @ ..] => super::t2::n4204(s, out),
				['s', s @ ..] => super::t2::n4205(s, out),
				['t', s @ ..] => super::t2::n4206(s, out),
				['v', s @ ..] => super::t2::n4207(s, out),
				['x', s @ ..] => super::t2::n4208(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n2058(s, out),
		['m', s @ ..] => super::t2::n2059(s, out),
		['n', s @ ..] => super::t2::n2060(s, out),
		['o', s @ ..] => super::t2::n2061(s, out),
		['r', s @ ..] => super::t2::n2062(s, out),
		['u', s @ ..] => super::t2::n2063(s, out),
		_ => false,
	}
}

/// Words ending -ous.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 2045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pus.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2046);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4172);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4173);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6214);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4174);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2048);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 4177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2050);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['o', 'p', 'o', 'h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4179);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6216);
							match s {
								[] => true,
								['i', 'p', 'e', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4181);
					match s {
						[] => true,
						['e', 't', 'c', 'y', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6218);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 6219);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4182);
					match s {
						[] => true,
						['c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						['o', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2053);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2054);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4184);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2056);
			match s {
				[] => true,
				['l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4186);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4187);
					s.is_empty()
				}
				['s', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4188);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2057);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['a', 'l', 'e', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				['i', 'l', 'l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4194);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4195);
					match s {
						[] => true,
						['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2061);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4196);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6228);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4199);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4200);
					match s {
						[] => true,
						['p', 'o', 'c', 'i', 'n', 'e', 'o', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4201);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['c', 'o', 'n', 'a', 'b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['h', 'p', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4204);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2066);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4206);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 4207);
					s.is_empty()
				}
				['g', 'a', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4209);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7876);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7877);
									match s {
										[] => true,
										['o', 'z', '\u{1}', s @ ..] => {
											super::push(out, 8887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 4211);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2067);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4214);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6232);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4215);
					match s {
						[] => true,
						['i', 'n', 'e', 'o', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 4216);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4218);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4219);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4220);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6235);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2070);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6238);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6240);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2072);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4225);
					s.is_empty()
				}
				['g', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4226);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4227);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6242);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'r', 'a', 'd', 'n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2073);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2075);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4231);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6245);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7879);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8888);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9328);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 8889);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6246);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7880);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['l', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4233);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6247);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 6248);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4235);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7882);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7883);
									match s {
										[] => true,
										['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8890);
											s.is_empty()
										}
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
			super::push(out, 2080);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4236);
					match s {
						[] => true,
						['r', 'p', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4237);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6251);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7884);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7885);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 8891);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7886);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6252);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8892);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 9329);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 8893);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 8894);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8895);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 7888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6253);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4238);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7890);
									s.is_empty()
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4239);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4241);
					match s {
						[] => true,
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2084);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6255);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6256);
							match s {
								[] => true,
								['r', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 7892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6257);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6258);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6259);
							match s {
								[] => true,
								['d', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7893);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7894);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 'l', '\u{1}', s @ ..] => {
													super::push(out, 9330);
													s.is_empty()
												}
												['n', 'g', 'i', 's', '\u{1}', s @ ..] => {
													super::push(out, 9331);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 7895);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8896);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 's', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7896);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7897);
									match s {
										[] => true,
										['o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8897);
											s.is_empty()
										}
										['u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8898);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6260);
							match s {
								[] => true,
								['v', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4245);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4246);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6262);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8899);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8900);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 9332);
													s.is_empty()
												}
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9333);
													s.is_empty()
												}
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
							super::push(out, 6263);
							match s {
								[] => true,
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 7899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6264);
							match s {
								[] => true,
								['g', 'a', 'd', 'e', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4247);
					match s {
						[] => true,
						['p', 'i', 'l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n4297(s, out),
				['l', s @ ..] => super::t2::n4298(s, out),
				['m', s @ ..] => {
					super::push(out, 4250);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6278);
							match s {
								[] => true,
								['l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t2::n6376(s, out),
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => super::t2::n6377(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n4300(s, out),
				['p', s @ ..] => super::t2::n4301(s, out),
				['r', s @ ..] => super::t2::n4302(s, out),
				['s', s @ ..] => super::t2::n4303(s, out),
				['t', s @ ..] => super::t2::n4304(s, out),
				['v', s @ ..] => super::t2::n4305(s, out),
				['y', 'c', '\u{1}', s @ ..] => super::t2::n4306(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n2104(s, out),
		['e', s @ ..] => super::t2::n2105(s, out),
		['i', s @ ..] => super::t2::n2106(s, out),
		['l', s @ ..] => super::t2::n2107(s, out),
		['n', s @ ..] => super::t2::n2108(s, out),
		['o', s @ ..] => super::t2::n2109(s, out),
		['p', s @ ..] => super::t2::n2110(s, out),
		['r', s @ ..] => super::t2::n2111(s, out),
		['s', s @ ..] => super::t2::n2112(s, out),
		['t', s @ ..] => super::t2::n2113(s, out),
		['u', s @ ..] => super::t2::n2114(s, out),
		['x', s @ ..] => super::t2::n2115(s, out),
		['y', s @ ..] => super::t2::n2116(s, out),
		_ => false,
	}
}

/// Words ending -uus.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['l', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2098);
			s.is_empty()
		}
		['n', 'r', 'o', 'c', 'i', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 2099);
			s.is_empty()
		}
		['r', 't', 'a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vus.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2101);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4313);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 6385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2103);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4315);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4316);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4317);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6387);
							match s {
								[] => true,
								['c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'n', 'u', 'i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6388);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6389);
							match s {
								[] => true,
								['n', 'i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2104);
			match s {
				[] => true,
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2105);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 4319);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xus.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2106);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4320);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4321);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2107);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2108);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 4323);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4324);
					match s {
						[] => true,
						['f', 'n', s @ ..] => {
							super::push(out, 6393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trix.
pub(crate) fn n2184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2162);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4358);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6400);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7971);
							match s {
								[] => true,
								['r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7972);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8923);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 9343);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9344);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 9470);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 8924);
									match s {
										[] => true,
										['b', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9345);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 9346);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8925);
									match s {
										[] => true,
										['a', 'v', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7973);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7974);
							match s {
								[] => true,
								['d', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6401);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7975);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6402);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6403);
					match s {
						[] => true,
						['a', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7977);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7978);
							match s {
								[] => true,
								['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7979);
							match s {
								[] => true,
								['u', 'v', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6404);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7980);
							match s {
								[] => true,
								['i', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7981);
							match s {
								[] => true,
								['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6405);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7983);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7984);
							match s {
								[] => true,
								['p', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7985);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8932);
									s.is_empty()
								}
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7986);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7987);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8934);
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8935);
									s.is_empty()
								}
								['p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6406);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7989);
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 7990);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7991);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 8937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7992);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7993);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 8938);
									match s {
										[] => true,
										['i', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9348);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 9349);
											match s {
												[] => true,
												['n', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 9471);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 's', 'a', 'r', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8939);
									s.is_empty()
								}
								['v', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7994);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 8941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'r', 'u', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 6408);
					s.is_empty()
				}
				['r', s @ ..] => super::t2::n6507(s, out),
				['s', s @ ..] => super::t2::n6508(s, out),
				['t', s @ ..] => super::t2::n6509(s, out),
				['u', 'n', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6412);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6413);
					match s {
						[] => true,
						['r', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n4409(s, out),
		['e', s @ ..] => super::t2::n4410(s, out),
		['i', s @ ..] => super::t2::n4411(s, out),
		['l', s @ ..] => super::t2::n4412(s, out),
		['n', s @ ..] => {
			super::push(out, 4363);
			s.is_empty()
		}
		['o', s @ ..] => super::t2::n4414(s, out),
		['p', s @ ..] => super::t2::n4415(s, out),
		['r', 'e', 's', 'e', 'd', '\u{1}', s @ ..] => super::t2::n4416(s, out),
		['s', s @ ..] => super::t2::n4417(s, out),
		['u', s @ ..] => super::t2::n4418(s, out),
		_ => false,
	}
}

/// The word mythologia.
pub(crate) fn n9512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9350);
	s.is_empty()
}

/// The word zythologia.
pub(crate) fn n9513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9351);
	s.is_empty()
}

/// The word isometria.
pub(crate) fn n8186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8052);
	s.is_empty()
}

/// The word schematometria.
pub(crate) fn n8187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8053);
	s.is_empty()
}

/// Words ending -ntria.
pub(crate) fn n2435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2407);
	s.is_empty()
}

/// Words ending -stria.
pub(crate) fn n2436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2408);
	match s {
		[] => true,
		['i', 'c', 'u', 'b', 'm', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 4497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uria.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2409);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2410);
			s.is_empty()
		}
		['h', 'c', 's', 'i', '\u{1}', s @ ..] => {
			super::push(out, 2411);
			s.is_empty()
		}
		['i', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 2412);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 2413);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2414);
			s.is_empty()
		}
		['n', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2415);
			s.is_empty()
		}
		['x', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntia.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 858);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4524);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4525);
					s.is_empty()
				}
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 4526);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4527);
					match s {
						[] => true,
						['a', 'v', 'a', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6495);
							s.is_empty()
						}
						['e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6496);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4528);
					match s {
						[] => true,
						['p', 'o', 'c', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 6498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4529);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4530);
					match s {
						[] => true,
						['o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4532);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6500);
							match s {
								[] => true,
								['b', 'u', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8054);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8055);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8056);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8057);
									s.is_empty()
								}
								['v', 'e', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6501);
							s.is_empty()
						}
						['o', 'n', 'g', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4533);
					match s {
						[] => true,
						['e', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 6503);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6504);
							match s {
								[] => true,
								['g', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6505);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8060);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8061);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8970);
											s.is_empty()
										}
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
					super::push(out, 4534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4535);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4536);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6506);
							match s {
								[] => true,
								['d', 'e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8062);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 8063);
									match s {
										[] => true,
										['i', s @ ..] => super::t2::n9128(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t2::n6607(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n4593(s, out),
				['g', s @ ..] => super::t2::n4594(s, out),
				['i', s @ ..] => super::t2::n4595(s, out),
				['l', s @ ..] => super::t2::n4596(s, out),
				['m', s @ ..] => super::t2::n4597(s, out),
				['n', s @ ..] => super::t2::n4598(s, out),
				['r', s @ ..] => super::t2::n4599(s, out),
				['s', s @ ..] => {
					super::push(out, 4544);
					s.is_empty()
				}
				['t', s @ ..] => super::t2::n4601(s, out),
				['u', s @ ..] => super::t2::n4602(s, out),
				['v', s @ ..] => {
					super::push(out, 4547);
					match s {
						[] => true,
						['i', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t2::n6635(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n2473(s, out),
		_ => false,
	}
}

/// Words ending -icula.
pub(crate) fn n2553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2524);
	match s {
		[] => true,
		['b', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4621);
			s.is_empty()
		}
		['c', 'l', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4622);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4623);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6554);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6555);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6556);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 4624);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4625);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6558);
					match s {
						[] => true,
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4626);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4627);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6559);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 8103);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6560);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6561);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 8105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6562);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6563);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8106);
							s.is_empty()
						}
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 6565);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6566);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8109);
							s.is_empty()
						}
						['s', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8110);
							s.is_empty()
						}
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 8111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6567);
					match s {
						[] => true,
						['c', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 8112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6568);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4630);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6570);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8113);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6571);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6572);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 8115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6573);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6574);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6575);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6576);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 8116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 4631);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8117);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 8118);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ncula.
pub(crate) fn n2554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2525);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6578);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 8120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6579);
					match s {
						[] => true,
						['a', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6580);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8122);
							s.is_empty()
						}
						['n', 'i', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8123);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8124);
							match s {
								[] => true,
								['a', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8995);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8996);
									s.is_empty()
								}
								['u', 'l', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8125);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8998);
									match s {
										[] => true,
										['c', 'e', 'r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9364);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 9365);
											s.is_empty()
										}
										['n', 'e', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9366);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 9367);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 9477);
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 9478);
													match s {
														[] => true,
														['u', 'c', 'u', 'l', '\u{1}', s @ ..] => super::t2::n9659(s, out),
														_ => false,
													}
												}
												['o', '\u{1}', s @ ..] => {
													super::push(out, 9479);
													s.is_empty()
												}
												['r', 'a', 'n', '\u{1}', s @ ..] => {
													super::push(out, 9480);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 9368);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 8999);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 9369);
											match s {
												[] => true,
												['i', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 9481);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 9370);
											match s {
												[] => true,
												['d', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 9482);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9371);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 9000);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9372);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 9001);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 9002);
									match s {
										[] => true,
										['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 9374);
											s.is_empty()
										}
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

/// The word radula.
pub(crate) fn n2559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2530);
	s.is_empty()
}

/// Words ending -edula.
pub(crate) fn n2560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2531);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4638);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6584);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4639);
			s.is_empty()
		}
		['r', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4640);
			s.is_empty()
		}
		['t', 'i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndula.
pub(crate) fn n2561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2532);
	s.is_empty()
}

/// Words ending -fula.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 897);
	match s {
		[] => true,
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 2533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gula.
pub(crate) fn n903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2534);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 4642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2536);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 4644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
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

/// Words ending -hula.
pub(crate) fn n904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 899);
	s.is_empty()
}

/// Words ending -lula.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 900);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4645);
					match s {
						[] => true,
						['b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4646);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 6587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mula.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2539);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2540);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 4647);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 4648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2542);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2543);
			match s {
				[] => true,
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2544);
			s.is_empty()
		}
		['y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nula.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 902);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2546);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4650);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4651);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6588);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2548);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4652);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4653);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4654);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4655);
					s.is_empty()
				}
				['r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4656);
					s.is_empty()
				}
				['v', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2549);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2550);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2551);
			s.is_empty()
		}
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2552);
			s.is_empty()
		}
		_ => false,
	}
}

