use flexion::Path;

/// Words ending -icken.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1485);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3205);
					match s {
						[] => true,
						['m', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3206);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3207);
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
		['h', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5363);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5364);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5365);
							match s {
								[] => true,
								['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5366);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 5367);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5368);
							match s {
								[] => true,
								['i', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5369);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5370);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5371);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 5372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1488);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3208);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5373);
							s.is_empty()
						}
						['b', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5374);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5375);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5376);
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
						['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5377);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5378);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6854);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5379);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5380);
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
				['f', s @ ..] => {
					super::push(out, 3209);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5381);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5382);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5383);
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
		['n', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3211);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5384);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5385);
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
				['n', s @ ..] => super::t5::n4032(s, out),
				['u', s @ ..] => super::t5::n4033(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t5::n1678(s, out),
		['r', s @ ..] => super::t5::n1679(s, out),
		['t', s @ ..] => super::t5::n1680(s, out),
		['u', s @ ..] => {
			super::push(out, 1494);
			s.is_empty()
		}
		['w', s @ ..] => super::t5::n1682(s, out),
		['z', s @ ..] => super::t5::n1683(s, out),
		_ => false,
	}
}

/// Words ending -ocken.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3219);
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
		['d', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3220);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3221);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5395);
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
				['s', s @ ..] => {
					super::push(out, 3222);
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
			super::push(out, 1499);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3224);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5396);
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
					super::push(out, 3227);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3228);
					match s {
						[] => true,
						['e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3229);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3230);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3231);
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
		['n', s @ ..] => {
			super::push(out, 1501);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5398);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5399);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5400);
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
		['z', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3232);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3233);
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

/// Words ending -ucken.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		['f', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3235);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5401);
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5402);
							s.is_empty()
						}
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1509);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				['m', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3238);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5404);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 7505);
											match s {
												[] => true,
												['n', 'u', s @ ..] => {
													match s {
														[] => true,
														['n', 'i', 'h', '\u{1}', s @ ..] => super::t5::n14274(s, out),
														['r', '\u{1}', s @ ..] => super::t5::n14275(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 7506);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5405);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5406);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6861);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6862);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5407);
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
			super::push(out, 1514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5408);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5409);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5410);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5411);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7507);
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
						['r', s @ ..] => {
							super::push(out, 5412);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5413);
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
		['z', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3242);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3243);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ücken.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1516);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3246);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6863);
									s.is_empty()
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5414);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5415);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ß', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5416);
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
			super::push(out, 1519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3248);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3249);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5417);
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
				['d', s @ ..] => {
					super::push(out, 3250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5418);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5419);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5420);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5421);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5422);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5423);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6864);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6865);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6866);
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
							super::push(out, 5424);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7508);
											s.is_empty()
										}
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7509);
											s.is_empty()
										}
										['t', 'n', 'u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7744);
													s.is_empty()
												}
												['n', 'i', 'h', '\u{1}', s @ ..] => {
													super::push(out, 7745);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 7746);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14276(s, out),
														['e', 'h', '\u{1}', s @ ..] => super::t5::n14277(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 7510);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 7511);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5425);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5426);
							match s {
								[] => true,
								['s', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5427);
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
				['e', s @ ..] => {
					super::push(out, 3251);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3252);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5428);
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
				['n', s @ ..] => {
					super::push(out, 3253);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5430);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t5::n7666(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t5::n4095(s, out),
				['s', s @ ..] => super::t5::n4096(s, out),
				['t', s @ ..] => super::t5::n4097(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t5::n1707(s, out),
		['z', s @ ..] => super::t5::n1708(s, out),
		_ => false,
	}
}

/// Words ending -tdenken.
pub(crate) fn n4131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3281);
	match s {
		[] => true,
		['i', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -henken.
pub(crate) fn n1720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3282);
			s.is_empty()
		}
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5443);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5444);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5445);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5446);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7512);
									match s {
										[] => true,
										['i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 7513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7747);
											s.is_empty()
										}
										['r', 'e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 7748);
											s.is_empty()
										}
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
					super::push(out, 5447);
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
		['r', s @ ..] => {
			super::push(out, 3283);
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

/// Words ending -lenken.
pub(crate) fn n1721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1533);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3284);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3285);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3286);
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

/// Words ending -renken.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1534);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3287);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3288);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3289);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3290);
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

/// Words ending -senken.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1535);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3291);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3292);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3293);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3294);
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

/// Words ending -wenken.
pub(crate) fn n1724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1536);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5448);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5449);
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

/// Words ending -inken.
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1537);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3295);
					match s {
						[] => true,
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3296);
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
		['m', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3297);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6873);
									s.is_empty()
								}
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
			super::push(out, 1540);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5451);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5452);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5453);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5454);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ö', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 6874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5455);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 7514);
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
								['o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5456);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 5457);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5458);
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
			super::push(out, 1541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3300);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6876);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 6877);
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6878);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 6879);
									s.is_empty()
								}
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
			super::push(out, 1543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3304);
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
		['z', s @ ..] => {
			super::push(out, 1544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3305);
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

/// Words ending -unken.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'e', 'h', 'c', 's', 'i', 'w', 'z', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3307);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3308);
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

/// Words ending -änken.
pub(crate) fn n516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5459);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5460);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5461);
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
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5462);
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

/// Words ending -ünken.
pub(crate) fn n517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3309);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word verfehlen.
pub(crate) fn n7781(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -hehlen.
pub(crate) fn n1762(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3345);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 3346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kehlen.
pub(crate) fn n1763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	s.is_empty()
}

/// Words ending -tehlen.
pub(crate) fn n1764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5481);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5482);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5483);
					match s {
						[] => true,
						['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 5484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ohlen.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1570);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3348);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3349);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uhlen.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ählen.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		['r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 1573);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1574);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3351);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3352);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3353);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3354);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6891);
									match s {
										[] => true,
										['e', 'i', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
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
					super::push(out, 3355);
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
		['z', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3356);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3357);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3358);
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
					super::push(out, 3359);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5485);
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
					super::push(out, 3360);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6892);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3361);
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

/// Words ending -öhlen.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3362);
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

/// Words ending -ühlen.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3363);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3364);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3365);
					match s {
						[] => true,
						['h', 'o', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3366);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5487);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5488);
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
					super::push(out, 3367);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3368);
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
		['k', s @ ..] => {
			super::push(out, 1578);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3369);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3370);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3371);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3373);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3374);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3375);
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

/// Words ending -teilen.
pub(crate) fn n1791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3388);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5491);
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
		['f', s @ ..] => {
			super::push(out, 3390);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5492);
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
		['i', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3392);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5495);
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
			super::push(out, 3393);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6893);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6894);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7519);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5497);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6896);
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
				['u', s @ ..] => {
					super::push(out, 5498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6897);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 3394);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5499);
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
			super::push(out, 3395);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5500);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3396);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5501);
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

/// Words ending -reinfallen.
pub(crate) fn n13085(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -hinfallen.
pub(crate) fn n11046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6899);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfallen.
pub(crate) fn n4304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3408);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5508);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6900);
					match s {
						[] => true,
						['ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6901);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6903);
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
				['v', s @ ..] => {
					super::push(out, 6904);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6905);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 7520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6906);
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
		['o', s @ ..] => {
			super::push(out, 5509);
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

/// Words ending -sfallen.
pub(crate) fn n4305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3409);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5510);
			match s {
				[] => true,
				['i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5511);
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

/// Words ending -tfallen.
pub(crate) fn n4306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3410);
	match s {
		[] => true,
		['h', 'c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5512);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5513);
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

/// Words ending -ufallen.
pub(crate) fn n4307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3411);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 5514);
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

/// Words ending -hallen.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3413);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6907);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6908);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3414);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3415);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5516);
					match s {
						[] => true,
						['d', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6909);
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
		_ => false,
	}
}

/// Words ending -lallen.
pub(crate) fn n1798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1598);
	s.is_empty()
}

/// Words ending -nallen.
pub(crate) fn n1799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1599);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3416);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6910);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6911);
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
		['k', s @ ..] => {
			super::push(out, 3417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5517);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5518);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 5519);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5520);
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

/// Words ending -rallen.
pub(crate) fn n1800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1600);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3418);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5521);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5522);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5523);
					match s {
						[] => true,
						['e', 'd', 'n', 'a', 'n', 'i', 'e', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wallen.
pub(crate) fn n1801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1601);
	s.is_empty()
}

/// Words ending -ellen.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1602);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3420);
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
			super::push(out, 1603);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3421);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3422);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6912);
									s.is_empty()
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3423);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3424);
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
			super::push(out, 1605);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3425);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6913);
									s.is_empty()
								}
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
			super::push(out, 1606);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5524);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5525);
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
		['s', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3426);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6914);
									s.is_empty()
								}
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
			super::push(out, 1609);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5526);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5527);
							match s {
								[] => true,
								['a', s @ ..] => super::t5::n11080(s, out),
								_ => false,
							}
						}
						['e', s @ ..] => super::t5::n7877(s, out),
						['f', s @ ..] => super::t5::n7878(s, out),
						['g', s @ ..] => super::t5::n7879(s, out),
						['h', s @ ..] => super::t5::n7880(s, out),
						['i', s @ ..] => super::t5::n7881(s, out),
						['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => super::t5::n7882(s, out),
						['m', s @ ..] => super::t5::n7883(s, out),
						['n', s @ ..] => super::t5::n7884(s, out),
						['r', s @ ..] => super::t5::n7885(s, out),
						['s', s @ ..] => super::t5::n7886(s, out),
						['t', s @ ..] => super::t5::n7887(s, out),
						['u', s @ ..] => super::t5::n7888(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n1810(s, out),
		['w', s @ ..] => super::t5::n1811(s, out),
		_ => false,
	}
}

/// Words ending -illen.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5541);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6933);
									s.is_empty()
								}
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
			super::push(out, 1613);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5542);
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

/// Words ending -ollen.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1614);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3431);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3432);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3433);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5543);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5544);
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
					super::push(out, 3434);
					match s {
						[] => true,
						['e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3435);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1617);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3436);
					s.is_empty()
				}
				['f', 'u', 'a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3438);
					match s {
						[] => true,
						['h', 'o', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3439);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6934);
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
				['s', 'u', 'a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3441);
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

/// Words ending -ullen.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3442);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5545);
							s.is_empty()
						}
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

/// Words ending -ällen.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3443);
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
		['g', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üllen.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3445);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3446);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3447);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3448);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3449);
					match s {
						[] => true,
						['l', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3450);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5547);
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
					super::push(out, 3451);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 3452);
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
			super::push(out, 1624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3453);
					s.is_empty()
				}
				['m', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3454);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5548);
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
				['u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3456);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3457);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3458);
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
			super::push(out, 1626);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5549);
							s.is_empty()
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5550);
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

/// The word beklimmen.
pub(crate) fn n7996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5584);
	s.is_empty()
}

/// The word erklimmen.
pub(crate) fn n7997(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rimmen.
pub(crate) fn n1894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1676);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 3509);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -timmen.
pub(crate) fn n1895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1677);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5585);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5586);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6940);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7536);
									s.is_empty()
								}
								['t', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5587);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7537);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7753);
											match s {
												[] => true,
												['h', 'r', 'o', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 7754);
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
								['t', s @ ..] => {
									super::push(out, 7538);
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
				['i', s @ ..] => {
					super::push(out, 5588);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5589);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5590);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6941);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6942);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7755);
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
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5591);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6943);
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
				['u', s @ ..] => {
					super::push(out, 5592);
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

/// Words ending -wimmen.
pub(crate) fn n1896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1678);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3511);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3512);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6944);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6945);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6946);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7756);
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
							super::push(out, 6947);
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

/// Words ending -ommen.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3513);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3514);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5593);
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
					super::push(out, 3515);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6948);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6949);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6950);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 6951);
									match s {
										[] => true,
										['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6952);
									match s {
										[] => true,
										['u', 'a', 'r', s @ ..] => {
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
								['t', s @ ..] => {
									super::push(out, 6953);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 6954);
									match s {
										[] => true,
										['z', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5595);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3516);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 3517);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3518);
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
					super::push(out, 3519);
					match s {
						[] => true,
						['e', s @ ..] => super::t5::n8014(s, out),
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => super::t5::n4485(s, out),
				['n', s @ ..] => super::t5::n4486(s, out),
				['r', s @ ..] => super::t5::n4487(s, out),
				['s', s @ ..] => super::t5::n4488(s, out),
				['t', s @ ..] => super::t5::n4489(s, out),
				['u', s @ ..] => super::t5::n4490(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n1898(s, out),
		_ => false,
	}
}

/// Words ending -ummen.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1681);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3526);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3527);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3528);
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
			super::push(out, 1683);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5601);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5602);
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
		['s', s @ ..] => {
			super::push(out, 1684);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ämmen.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3529);
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
		['k', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3530);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3531);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ümmen.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	s.is_empty()
}

/// Words ending -nrechnen.
pub(crate) fn n8108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5631);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6969);
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
			super::push(out, 6970);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrechnen.
pub(crate) fn n8109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5632);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6971);
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

/// Words ending -urechnen.
pub(crate) fn n8111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5634);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ichnen.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1720);
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5635);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5636);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5637);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5638);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5639);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6972);
							match s {
								[] => true,
								['g', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5640);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 7758);
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
					super::push(out, 5641);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5642);
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

/// Words ending -ehnen.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3599);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3600);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3604);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3606);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3607);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3608);
					match s {
						[] => true,
						['u', 'a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3609);
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
		['s', s @ ..] => {
			super::push(out, 1723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3610);
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

/// Words ending -ohnen.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3611);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['w', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3612);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5643);
							match s {
								[] => true,
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
					super::push(out, 3613);
					match s {
						[] => true,
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

/// Words ending -ähnen.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 593);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3614);
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
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3615);
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

/// Words ending -öhnen.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1727);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1729);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5645);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1731);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5647);
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
			super::push(out, 1732);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3618);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6974);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6975);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6976);
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
				['g', s @ ..] => {
					super::push(out, 3619);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sühnen.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	s.is_empty()
}

/// Words ending -rkennen.
pub(crate) fn n4651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3635);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6989);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6990);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6991);
					match s {
						[] => true,
						['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6992);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6993);
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

/// Words ending -lennen.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1747);
	s.is_empty()
}

/// Words ending -nennen.
pub(crate) fn n1987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1748);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3637);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3638);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6994);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
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

/// Words ending -pennen.
pub(crate) fn n1988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3639);
			s.is_empty()
		}
		['n', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 3640);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3641);
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

/// Words ending -rennen.
pub(crate) fn n1989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1750);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5657);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5658);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5659);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6996);
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
					super::push(out, 5660);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7544);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7545);
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
					super::push(out, 5661);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5662);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5663);
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
		['g', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5664);
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
			super::push(out, 3644);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3645);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3646);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3647);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['u', 'a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5667);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5668);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5669);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5670);
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

/// Words ending -innen.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1751);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5671);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5672);
							match s {
								[] => true,
								['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5673);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1754);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5675);
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
					super::push(out, 3654);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3655);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3656);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6998);
									match s {
										[] => true,
										['ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 6999);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7000);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 7001);
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
					super::push(out, 3657);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3658);
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
		['s', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3660);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 7002);
									match s {
										[] => true,
										['c', 'ü', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3661);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3662);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3664);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3665);
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
		['w', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3666);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7003);
									s.is_empty()
								}
								['b', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7546);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', 'i', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
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
		['z', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3667);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 7004);
									s.is_empty()
								}
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

/// Words ending -onnen.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	s.is_empty()
}

/// Words ending -önnen.
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1758);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3668);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ß', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3671);
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
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3672);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['o', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'r', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 3673);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
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
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3674);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'u', s @ ..] => {
											match s {
												[] => true,
												['n', 'i', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14288(s, out),
														['d', '\u{1}', s @ ..] => super::t5::n14289(s, out),
														['e', 'h', '\u{1}', s @ ..] => super::t5::n14290(s, out),
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
						['ü', 'f', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5677);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
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
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5678);
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
						_ => false,
					}
				}
				['u', 'z', 'a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ünnen.
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ieren.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1801);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3731);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3732);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7008);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7548);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7009);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7549);
											s.is_empty()
										}
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
			super::push(out, 1802);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5706);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 7550);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 7759);
													s.is_empty()
												}
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
							super::push(out, 5707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1803);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3733);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7010);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7551);
											s.is_empty()
										}
										_ => false,
									}
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
				['d', s @ ..] => {
					super::push(out, 3734);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7011);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7012);
									match s {
										[] => true,
										['z', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3735);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 5708);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 5709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7552);
											match s {
												[] => true,
												['e', 'd', 'n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 7760);
													s.is_empty()
												}
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
							super::push(out, 5710);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 7553);
											s.is_empty()
										}
										['s', 'e', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3736);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
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
								['m', s @ ..] => {
									super::push(out, 7013);
									match s {
										[] => true,
										['m', 'o', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['b', s @ ..] => super::t5::n14062(s, out),
												['e', 'r', '\u{1}', s @ ..] => super::t5::n14063(s, out),
												['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => super::t5::n14064(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 't', s @ ..] => super::t5::n8291(s, out),
						['u', 'f', s @ ..] => super::t5::n8292(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t5::n4803(s, out),
				['u', s @ ..] => super::t5::n4804(s, out),
				['ä', 'l', s @ ..] => super::t5::n4805(s, out),
				_ => false,
			}
		}
		['f', 'f', s @ ..] => super::t5::n2061(s, out),
		['g', s @ ..] => super::t5::n2062(s, out),
		['h', s @ ..] => super::t5::n2063(s, out),
		['i', 'l', s @ ..] => super::t5::n2064(s, out),
		['k', s @ ..] => super::t5::n2065(s, out),
		['l', s @ ..] => super::t5::n2066(s, out),
		['m', s @ ..] => super::t5::n2067(s, out),
		['n', s @ ..] => super::t5::n2068(s, out),
		['p', s @ ..] => super::t5::n2069(s, out),
		['r', s @ ..] => super::t5::n2070(s, out),
		['s', s @ ..] => super::t5::n2071(s, out),
		['t', s @ ..] => super::t5::n2072(s, out),
		['u', s @ ..] => super::t5::n2073(s, out),
		['v', s @ ..] => super::t5::n2074(s, out),
		['z', s @ ..] => super::t5::n2075(s, out),
		_ => false,
	}
}

/// Words ending -ckfahren.
pub(crate) fn n8436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5757);
	match s {
		[] => true,
		['ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfahren.
pub(crate) fn n4887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3796);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5758);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mfahren.
pub(crate) fn n4888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3797);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5759);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7059);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7060);
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
		_ => false,
	}
}

/// Words ending -nfahren.
pub(crate) fn n4889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3798);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5761);
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
			super::push(out, 5762);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
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
		['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfahren.
pub(crate) fn n4890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3799);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7062);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7063);
					match s {
						[] => true,
						['ü', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7780);
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7064);
					match s {
						[] => true,
						['i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7065);
					match s {
						[] => true,
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7066);
					match s {
						[] => true,
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7067);
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
		['o', s @ ..] => {
			super::push(out, 5764);
			match s {
				[] => true,
				['p', 'm', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7068);
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

/// Words ending -sfahren.
pub(crate) fn n4891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3800);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5765);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5766);
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

/// Words ending -tfahren.
pub(crate) fn n4892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3801);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5767);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5768);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zfahren.
pub(crate) fn n4893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3802);
	match s {
		[] => true,
		['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wahren.
pub(crate) fn n2082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1820);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3803);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3804);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7069);
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
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ehren.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1821);
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3805);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3806);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7070);
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
		['k', s @ ..] => {
			super::push(out, 1823);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3807);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3808);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3809);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3810);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5769);
							match s {
								[] => true,
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5770);
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
					super::push(out, 3811);
					match s {
						[] => true,
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5771);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5772);
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
					super::push(out, 3812);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7071);
									match s {
										[] => true,
										['e', 'i', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 7072);
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
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3813);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5774);
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
				['u', s @ ..] => {
					super::push(out, 3814);
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
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3815);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3816);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1824);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3817);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3819);
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
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3820);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3821);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3822);
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

/// Words ending -ohren.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3823);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5776);
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -ähren.
pub(crate) fn n675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 641);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 1826);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3825);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3826);
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
			super::push(out, 1827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'g', 'n', 'a', 'w', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3829);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3830);
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
				['t', s @ ..] => {
					super::push(out, 3831);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ühren.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 643);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1829);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3832);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3833);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5778);
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
				['e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3834);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5779);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 's', 'r', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3835);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5780);
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
					super::push(out, 3836);
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
				['i', 'e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3837);
					match s {
						[] => true,
						['l', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3838);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3839);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5781);
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
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5782);
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
					super::push(out, 3840);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5783);
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7073);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 7074);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 7597);
											match s {
												[] => true,
												['e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5784);
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
					super::push(out, 3841);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5785);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5786);
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
					super::push(out, 3842);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5787);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3843);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 5788);
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
			super::push(out, 1831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3844);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3845);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3846);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3847);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5790);
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
					super::push(out, 3848);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 7075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word dazugehören.
pub(crate) fn n14145(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -beschwören.
pub(crate) fn n11649(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7602);
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

/// Words ending -nschwören.
pub(crate) fn n8541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5804);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rschwören.
pub(crate) fn n8542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5805);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rchreisen.
pub(crate) fn n11672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7091);
	match s {
		[] => true,
		['u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kreisen.
pub(crate) fn n5042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3914);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5828);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5829);
			match s {
				[] => true,
				['ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5830);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mreisen.
pub(crate) fn n5043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3915);
	match s {
		[] => true,
		['i', 'e', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nreisen.
pub(crate) fn n5044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3916);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5832);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7093);
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
		['o', s @ ..] => {
			super::push(out, 5833);
			match s {
				[] => true,
				['v', 'a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -preisen.
pub(crate) fn n5045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3917);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5834);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5835);
			match s {
				[] => true,
				['o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5836);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7095);
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

/// Words ending -rreisen.
pub(crate) fn n5046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3918);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7097);
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

/// Words ending -sreisen.
pub(crate) fn n5047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3919);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -treisen.
pub(crate) fn n5048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3920);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5837);
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
		_ => false,
	}
}

/// Words ending -ureisen.
pub(crate) fn n5049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3921);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 5838);
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

/// Words ending -seisen.
pub(crate) fn n2173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1885);
	match s {
		[] => true,
		['o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teisen.
pub(crate) fn n2174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1886);
	match s {
		[] => true,
		['n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weisen.
pub(crate) fn n2175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1887);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3923);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3924);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5839);
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
			super::push(out, 3925);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3926);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5840);
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
			super::push(out, 3927);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5841);
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
			super::push(out, 3928);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3929);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3930);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5843);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7098);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7604);
									s.is_empty()
								}
								['s', 'g', 'n', 'a', 'w', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7099);
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
			super::push(out, 3931);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 7605);
									match s {
										[] => true,
										['c', 'ü', 'r', '\u{1}', s @ ..] => {
											super::push(out, 7781);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5845);
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
			super::push(out, 3932);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5846);
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
			super::push(out, 3933);
			match s {
				[] => true,
				['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3934);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5847);
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

/// Words ending -lassen.
pub(crate) fn n2201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1900);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3961);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3962);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5856);
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
		['b', s @ ..] => {
			super::push(out, 3963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5857);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5858);
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
				['b', s @ ..] => {
					super::push(out, 5859);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5860);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7606);
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'e', 's', 'i', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3965);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3966);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5862);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3967);
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
		['i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3968);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7100);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 7782);
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5864);
					match s {
						[] => true,
						['b', 'i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7101);
							s.is_empty()
						}
						['f', 'f', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7102);
							match s {
								[] => true,
								['n', 'ä', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7103);
							s.is_empty()
						}
						['z', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5865);
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
								['s', s @ ..] => {
									super::push(out, 7608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7783);
											s.is_empty()
										}
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
			super::push(out, 3969);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 7105);
							s.is_empty()
						}
						['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7106);
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7107);
							s.is_empty()
						}
						['t', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7784);
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7785);
											s.is_empty()
										}
										_ => false,
									}
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
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 5866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3970);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5867);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5868);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7609);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7610);
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5869);
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

/// The word prassen.
pub(crate) fn n8669(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rprassen.
pub(crate) fn n8670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5871);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -essen.
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1904);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3978);
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
		['f', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3979);
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
			super::push(out, 1907);
			match s {
				[] => true,
				['a', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1908);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5872);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3981);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7108);
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7109);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7110);
									s.is_empty()
								}
								['g', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7111);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7112);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'b', 'e', 'n', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7611);
											s.is_empty()
										}
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7113);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'q', '\u{1}', s @ ..] => {
													super::push(out, 7786);
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
										['o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 7612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7114);
									s.is_empty()
								}
								['t', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 7115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3982);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5873);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7116);
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7117);
									s.is_empty()
								}
								['m', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											super::push(out, 7613);
											s.is_empty()
										}
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 7614);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7118);
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5874);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7615);
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7616);
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7617);
											s.is_empty()
										}
										['m', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7618);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 7787);
													s.is_empty()
												}
												['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													super::push(out, 7788);
													s.is_empty()
												}
												['i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 7789);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
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
										['s', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7619);
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 7620);
											s.is_empty()
										}
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
		['m', s @ ..] => super::t5::n2211(s, out),
		['n', s @ ..] => super::t5::n2212(s, out),
		['r', s @ ..] => super::t5::n2213(s, out),
		['t', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['i', s @ ..] => super::t5::n5145(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n2215(s, out),
		_ => false,
	}
}

/// Words ending -issen.
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1915);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5886);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5887);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5888);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5890);
							s.is_empty()
						}
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7621);
											s.is_empty()
										}
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											super::push(out, 7622);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4002);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7121);
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
				['m', s @ ..] => {
					super::push(out, 4003);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7122);
									s.is_empty()
								}
								['m', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7123);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7124);
									s.is_empty()
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 'a', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7623);
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7624);
											s.is_empty()
										}
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
					super::push(out, 4004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5892);
							s.is_empty()
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5895);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7125);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
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
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7126);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7127);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5896);
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
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7625);
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
									super::push(out, 7129);
									s.is_empty()
								}
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'r', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 4005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1917);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4006);
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
		['p', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4008);
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
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ossen.
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1921);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7131);
									s.is_empty()
								}
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5898);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['e', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'a', 'n', 'i', 'e', 'f', 'u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 7626);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 7627);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 7132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ussen.
pub(crate) fn n741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1923);
			s.is_empty()
		}
		['h', 'f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1924);
			s.is_empty()
		}
		['r', '\u{1}', s @ ..] => {
			super::push(out, 1925);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yssen.
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ässen.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 704);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4009);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4011);
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

/// Words ending -üssen.
pub(crate) fn n744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 705);
	match s {
		[] => true,
		['h', 'c', 's', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1928);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['c', 'a', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4014);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4016);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5899);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -treten.
pub(crate) fn n2271(s: &[char], out: &mut Path<'_>) -> bool {
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
				['a', s @ ..] => {
					super::push(out, 5906);
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
			super::push(out, 4046);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4047);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5907);
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
			super::push(out, 4048);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5908);
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
			super::push(out, 4049);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4050);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5909);
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
		['k', s @ ..] => {
			super::push(out, 4051);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4052);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5910);
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
			super::push(out, 4053);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5911);
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
				['i', s @ ..] => {
					super::push(out, 5912);
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
			super::push(out, 4054);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'ä', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7628);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4055);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5913);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5914);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4056);
			match s {
				[] => true,
				['s', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4057);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'h', '\u{1}', s @ ..] => {
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

/// The word auskundschaften.
pub(crate) fn n14314(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word erwirtschaften.
pub(crate) fn n14315(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herunterwirtschaften.
pub(crate) fn n14316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7815);
	s.is_empty()
}

/// Words ending -rschmachten.
pub(crate) fn n13522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7635);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nachten.
pub(crate) fn n5266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4075);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5923);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pachten.
pub(crate) fn n5267(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5924);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rachten.
pub(crate) fn n5268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4076);
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
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7142);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
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
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7143);
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

/// The word missachten.
pub(crate) fn n5269(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -gutachten.
pub(crate) fn n5270(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5925);
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word mißachten.
pub(crate) fn n5271(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -echten.
pub(crate) fn n2286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1970);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5926);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5927);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5928);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5929);
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
			super::push(out, 4078);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4079);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7144);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7145);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7146);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7147);
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
		['n', s @ ..] => {
			super::push(out, 4080);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ichten.
pub(crate) fn n2287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1971);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4082);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5930);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5931);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5932);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5933);
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
			super::push(out, 4083);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4084);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7148);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7149);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7150);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7151);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7152);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5934);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5935);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5936);
					match s {
						[] => true,
						['p', 'i', s @ ..] => {
							super::push(out, 7153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5937);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7637);
									s.is_empty()
								}
								['m', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7638);
									s.is_empty()
								}
								['n', 'i', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7792);
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7793);
											s.is_empty()
										}
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
			super::push(out, 4086);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5938);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5939);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5940);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'n', 'u', 'r', 'g', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5941);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5942);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7156);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7639);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 7640);
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
					super::push(out, 5943);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 7641);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 5944);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5945);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5946);
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
		['s', s @ ..] => {
			super::push(out, 4088);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rchten.
pub(crate) fn n2288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1972);
	match s {
		[] => true,
		['ü', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uchten.
pub(crate) fn n2289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4090);
			match s {
				[] => true,
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5948);
					s.is_empty()
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4091);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7157);
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
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7158);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7159);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7160);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7161);
							match s {
								[] => true,
								['i', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7162);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7643);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7163);
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
		['s', s @ ..] => {
			super::push(out, 4092);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 4093);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word beychten.
pub(crate) fn n2290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1974);
	s.is_empty()
}

/// Words ending -ächten.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1975);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üchten.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1976);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4094);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word fighten.
pub(crate) fn n776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	s.is_empty()
}

/// Words ending -arbeiten.
pub(crate) fn n5293(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5951);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5952);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7164);
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
			super::push(out, 5953);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5954);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7165);
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
			super::push(out, 5955);
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
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5956);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7166);
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
			super::push(out, 5957);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7167);
					match s {
						[] => true,
						['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7168);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7644);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7645);
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
			super::push(out, 5958);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7646);
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 7647);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7794);
									match s {
										[] => true,
										['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 5959);
			match s {
				[] => true,
				['o', 'l', 'f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7169);
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
			super::push(out, 5960);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7170);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7171);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5961);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 7172);
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
		['z', s @ ..] => {
			super::push(out, 5962);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leiten.
pub(crate) fn n2295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1979);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4095);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5963);
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
			super::push(out, 4096);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5965);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5966);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5967);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5968);
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
		['h', s @ ..] => {
			super::push(out, 4098);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4099);
			match s {
				[] => true,
				['h', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4100);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5969);
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
			super::push(out, 4101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5971);
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
			super::push(out, 4102);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 7173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7174);
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
			super::push(out, 4103);
			match s {
				[] => true,
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5972);
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
		['u', s @ ..] => {
			super::push(out, 4104);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5973);
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
		['ß', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reiten.
pub(crate) fn n2296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1980);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4106);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5974);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5975);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7648);
									match s {
										[] => true,
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 7795);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 7649);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7796);
											match s {
												[] => true,
												['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 5976);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4108);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7175);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7176);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7177);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7178);
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
		['f', s @ ..] => {
			super::push(out, 4109);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5978);
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
			super::push(out, 4110);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7179);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7180);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7181);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7182);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7183);
							match s {
								[] => true,
								['a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7650);
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
							super::push(out, 7184);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7797);
											match s {
												[] => true,
												['n', 'i', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7185);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7186);
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
		['p', 's', '\u{1}', s @ ..] => {
			super::push(out, 4111);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4112);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5979);
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
			super::push(out, 4113);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7187);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7188);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7651);
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
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 4114);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5981);
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

/// Words ending -halten.
pub(crate) fn n2302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1984);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4118);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5982);
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
		['c', s @ ..] => {
			super::push(out, 4120);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7189);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7190);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7191);
							match s {
								[] => true,
								['c', 'i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7192);
							match s {
								[] => true,
								['e', 'r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7193);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 7652);
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
							super::push(out, 7194);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7653);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7654);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7655);
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
							super::push(out, 7195);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7196);
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
		['e', s @ ..] => {
			super::push(out, 4121);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7197);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7198);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7199);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7200);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7657);
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
							super::push(out, 7201);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5985);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5986);
					match s {
						[] => true,
						['i', 'e', 's', 'i', 'e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4122);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4123);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5987);
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
			super::push(out, 4124);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
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
		['i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['i', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'i', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n8880(s, out),
				['i', s @ ..] => super::t5::n8881(s, out),
				['r', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n5334(s, out),
		['s', s @ ..] => super::t5::n5335(s, out),
		['t', s @ ..] => super::t5::n5336(s, out),
		['u', s @ ..] => {
			super::push(out, 4129);
			match s {
				[] => true,
				['z', s @ ..] => super::t5::n8891(s, out),
				_ => false,
			}
		}
		['ß', 'a', 'm', '\u{1}', s @ ..] => super::t5::n5338(s, out),
		_ => false,
	}
}

/// Words ending -rwalten.
pub(crate) fn n5346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4136);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5999);
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

/// Words ending -elten.
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 739);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4137);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4138);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1991);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1992);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gilten.
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	s.is_empty()
}

/// Words ending -älten.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	s.is_empty()
}

/// Words ending -nrosten.
pub(crate) fn n5419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4181);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 6014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prosten.
pub(crate) fn n5420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4182);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6015);
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

/// Words ending -rrosten.
pub(crate) fn n5421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4183);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rsten.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 765);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6016);
							s.is_empty()
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['f', 'f', s @ ..] => {
					super::push(out, 4184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6018);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6019);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6020);
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
		['ü', s @ ..] => {
			super::push(out, 2026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usten.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 766);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4186);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4187);
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
		['p', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4188);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4190);
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
		['r', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4191);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6021);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -östen.
pub(crate) fn n813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 767);
	match s {
		[] => true,
		['r', s @ ..] => {
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
						['g', s @ ..] => {
							super::push(out, 6022);
							match s {
								[] => true,
								['e', 'w', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6023);
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

/// Words ending -üsten.
pub(crate) fn n814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 768);
	match s {
		[] => true,
		['l', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6024);
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
				['f', s @ ..] => {
					super::push(out, 4194);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4195);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4196);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4198);
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
		['w', s @ ..] => {
			super::push(out, 2031);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zurückerstatten.
pub(crate) fn n14205(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -otten.
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4216);
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
		['p', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6033);
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
		['r', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4217);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7211);
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
				['n', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4219);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4220);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4221);
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

/// Words ending -ütten.
pub(crate) fn n820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7212);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 7213);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 7214);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7215);
									match s {
										[] => true,
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7216);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7217);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7218);
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
				['r', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2048);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hinunterschauen.
pub(crate) fn n14324(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word runterschauen.
pub(crate) fn n14325(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word niederhauen.
pub(crate) fn n9050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6054);
	s.is_empty()
}

/// The word verhauen.
pub(crate) fn n9051(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word zerhauen.
pub(crate) fn n9052(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -shauen.
pub(crate) fn n2432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2085);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 4263);
			match s {
				[] => true,
				['a', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uhauen.
pub(crate) fn n2433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2086);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 4264);
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

/// The word überstürzen.
pub(crate) fn n14207(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word niederstürzen.
pub(crate) fn n14208(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -fhetzen.
pub(crate) fn n5644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4322);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rhetzen.
pub(crate) fn n5645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4323);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -letzen.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2135);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4324);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -netzen.
pub(crate) fn n2504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2136);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4325);
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -petzen.
pub(crate) fn n2505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2137);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4326);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4327);
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

/// Words ending -setzen.
pub(crate) fn n2506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2138);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4328);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4329);
			match s {
				[] => true,
				['a', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 4330);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7233);
							s.is_empty()
						}
						['l', 'h', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4331);
			match s {
				[] => true,
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
		['g', s @ ..] => {
			super::push(out, 4332);
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4333);
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
			super::push(out, 4334);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7235);
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
			super::push(out, 4335);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6097);
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
			super::push(out, 4336);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6099);
					match s {
						[] => true,
						['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6100);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7237);
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
			super::push(out, 4337);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7238);
							match s {
								[] => true,
								['ü', '\u{1}', s @ ..] => {
									super::push(out, 7670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7239);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7671);
									s.is_empty()
								}
								['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7240);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
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
						['v', s @ ..] => {
							super::push(out, 7241);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7672);
									s.is_empty()
								}
								['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6102);
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
			super::push(out, 4338);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4339);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6103);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4340);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 6104);
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

/// Words ending -wetzen.
pub(crate) fn n2507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2139);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4341);
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

/// Words ending -itzen.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 834);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4342);
					s.is_empty()
				}
				['i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 4343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2140);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4344);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6106);
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
				['f', s @ ..] => {
					super::push(out, 4346);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4347);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7242);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7243);
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
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2142);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2143);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6107);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6108);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6109);
							match s {
								[] => true,
								['e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6110);
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
		['r', s @ ..] => {
			super::push(out, 2144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4348);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4349);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7244);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7245);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7246);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 7247);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7248);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7249);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 7800);
													match s {
														[] => true,
														['e', 'i', 'n', '\u{1}', s @ ..] => super::t5::n14326(s, out),
														_ => false,
													}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4350);
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
		['s', s @ ..] => {
			super::push(out, 2145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4351);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4352);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 4353);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4354);
					match s {
						[] => true,
						['n', 'u', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4355);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4356);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4357);
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
					super::push(out, 4358);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4359);
					match s {
						[] => true,
						['l', 'i', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4360);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 4361);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4362);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4363);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4364);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => super::t5::n9189(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => super::t5::n2515(s, out),
		_ => false,
	}
}

/// Words ending -otzen.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 835);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 2147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4365);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4366);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4367);
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
		['l', s @ ..] => {
			super::push(out, 2148);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6114);
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
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4369);
					match s {
						[] => true,
						['m', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 6115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
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

/// Words ending -utzen.
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 836);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 2150);
			s.is_empty()
		}
		['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2151);
			s.is_empty()
		}
		['k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2152);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4370);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4372);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4373);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4374);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4375);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4376);
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
		['p', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4378);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4379);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4380);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4381);
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
		['r', s @ ..] => {
			super::push(out, 2156);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ätzen.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 837);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2158);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2160);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6116);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6117);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 6118);
							match s {
								[] => true,
								['c', 'o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6119);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6120);
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
							super::push(out, 6121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2161);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2162);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ergötzen.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 838);
	s.is_empty()
}

/// Words ending -ützen.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 839);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2163);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6122);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6123);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6124);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 7250);
									s.is_empty()
								}
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
			super::push(out, 2164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4382);
					s.is_empty()
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6125);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6126);
							match s {
								[] => true,
								['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -fließen.
pub(crate) fn n5742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4397);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6140);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6141);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6142);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6143);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6144);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7258);
					match s {
						[] => true,
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7259);
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
			super::push(out, 6145);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
			super::push(out, 6146);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6147);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6148);
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

/// Words ending -hließen.
pub(crate) fn n5743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4398);
	match s {
		[] => true,
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7260);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7261);
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
				['f', s @ ..] => {
					super::push(out, 7262);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7263);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7677);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7678);
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
					super::push(out, 7264);
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
					super::push(out, 7265);
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
				['z', s @ ..] => {
					super::push(out, 7266);
					match s {
						[] => true,
						['r', 'u', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prießen.
pub(crate) fn n5746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4401);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7267);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7268);
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

/// Words ending -leißen.
pub(crate) fn n2556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2183);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4412);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4413);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7276);
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
		['p', s @ ..] => {
			super::push(out, 4414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -meißen.
pub(crate) fn n2557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2184);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4415);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7277);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7278);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7279);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7280);
							match s {
								[] => true,
								['e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7281);
							match s {
								[] => true,
								['u', 'a', 'r', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reißen.
pub(crate) fn n2558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2185);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4416);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4417);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6151);
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
		['f', s @ ..] => {
			super::push(out, 4418);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6152);
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
			super::push(out, 4419);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6153);
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
		['i', s @ ..] => {
			super::push(out, 4420);
			match s {
				[] => true,
				['e', 'w', 'z', 't', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4422);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4423);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6156);
					match s {
						[] => true,
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6157);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7283);
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
			super::push(out, 4424);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7284);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 7285);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4425);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6158);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6159);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4426);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6160);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6161);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6162);
					match s {
						[] => true,
						['u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weißen.
pub(crate) fn n2559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2186);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4427);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7287);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7288);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7679);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7680);
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
		_ => false,
	}
}

/// The word übersiedeln.
pub(crate) fn n14222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7801);
	s.is_empty()
}

