use flexion::Path;

/// Words ending -use (3 syllables).
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1784);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4486);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4487);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7260);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7261);
					match s {
						[] => true,
						['a', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9297);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9298);
							match s {
								[] => true,
								['a', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10403);
									s.is_empty()
								}
								['o', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 10404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 's', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9299);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 9300);
							match s {
								[] => true,
								['c', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10405);
									s.is_empty()
								}
								['o', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10406);
									s.is_empty()
								}
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7262);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 10408);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 10409);
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
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10976);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10977);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'p', 'm', s @ ..] => {
									match s {
										[] => true,
										['a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10978);
											s.is_empty()
										}
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9301);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9302);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7263);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9303);
							match s {
								[] => true,
								['d', 'n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10411);
									s.is_empty()
								}
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

/// Words ending -noscente (3 syllables).
pub(crate) fn n12109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10980);
	match s {
		[] => true,
		['g', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 11261);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 11262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ch (2 syllables).
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 664);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4656);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4657);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7369);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7370);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7371);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7372);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 4658);
					s.is_empty()
				}
				['g', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4659);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4660);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4661);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7373);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7374);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7375);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 9338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4662);
					s.is_empty()
				}
				['n', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4663);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4664);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7377);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4665);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4666);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 7379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4667);
					s.is_empty()
				}
				['t', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4669);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7380);
							s.is_empty()
						}
						['r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 7381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4670);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4671);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 9339);
									s.is_empty()
								}
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 9340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['b', 'b', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4673);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4674);
					match s {
						[] => true,
						['m', 'i', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7383);
							s.is_empty()
						}
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4675);
					match s {
						[] => true,
						['d', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7385);
							s.is_empty()
						}
						['h', 'c', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4676);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 9341);
									s.is_empty()
								}
								['y', 'v', '\u{1}', s @ ..] => {
									super::push(out, 9342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7387);
							s.is_empty()
						}
						['p', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7388);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7389);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9343);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1927);
			match s {
				[] => true,
				['e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4678);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4679);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4680);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 9345);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 9346);
									s.is_empty()
								}
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
			super::push(out, 1928);
			match s {
				[] => true,
				['m', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 4681);
					s.is_empty()
				}
				['p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4682);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7390);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 7391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4683);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7392);
							s.is_empty()
						}
						['d', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7393);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7394);
							match s {
								[] => true,
								['s', s @ ..] => super::t3::n10213(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t3::n8015(s, out),
						['l', 'e', 'e', 'h', 'w', '\u{1}', s @ ..] => super::t3::n8016(s, out),
						['m', s @ ..] => super::t3::n8017(s, out),
						['n', s @ ..] => super::t3::n8018(s, out),
						['o', s @ ..] => super::t3::n8019(s, out),
						['p', s @ ..] => super::t3::n8020(s, out),
						['r', s @ ..] => super::t3::n8021(s, out),
						['s', 'a', 'i', 'h', 't', '\u{1}', s @ ..] => super::t3::n8022(s, out),
						['t', 's', 'n', s @ ..] => super::t3::n8023(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n5019(s, out),
				['o', s @ ..] => super::t3::n5020(s, out),
				['u', s @ ..] => super::t3::n5021(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t3::n2077(s, out),
		['t', s @ ..] => super::t3::n2078(s, out),
		['u', s @ ..] => super::t3::n2079(s, out),
		['y', s @ ..] => super::t3::n2080(s, out),
		_ => false,
	}
}

/// Words ending -ch (3 syllables).
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 665);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1934);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4698);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7412);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4699);
					match s {
						[] => true,
						['a', 'g', 'g', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7414);
							s.is_empty()
						}
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['g', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 9362);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 9363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7415);
							match s {
								[] => true,
								['g', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 9364);
									s.is_empty()
								}
								['h', 's', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4700);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4701);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7417);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9366);
									match s {
										[] => true,
										['a', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 10430);
											s.is_empty()
										}
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
					super::push(out, 4702);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7418);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4703);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7420);
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
				['e', s @ ..] => {
					super::push(out, 4704);
					s.is_empty()
				}
				['l', 'e', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4705);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4706);
					match s {
						[] => true,
						['a', 's', 'a', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7421);
							s.is_empty()
						}
						['e', 's', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4707);
					match s {
						[] => true,
						['e', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7423);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 's', 'i', 'r', 'a', 'v', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 1935);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['l', 'i', 'm', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4708);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4709);
					match s {
						[] => true,
						['a', 'v', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 7425);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 's', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9367);
									s.is_empty()
								}
								['o', 't', 'p', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4710);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4711);
					match s {
						[] => true,
						['e', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 's', 't', '\u{1}', s @ ..] => {
											super::push(out, 10431);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10432);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 10433);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10434);
											s.is_empty()
										}
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
					super::push(out, 4712);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4713);
					match s {
						[] => true,
						['r', 'b', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9369);
									s.is_empty()
								}
								['e', 'n', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9370);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9371);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10435);
											s.is_empty()
										}
										['n', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 10436);
											s.is_empty()
										}
										['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10437);
											s.is_empty()
										}
										['t', 'p', 'y', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10438);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 9372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4714);
					match s {
						[] => true,
						['l', 'e', 'h', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7428);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4715);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 't', 'u', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7431);
							s.is_empty()
						}
						['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4716);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'h', 'r', 'o', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n2087(s, out),
		['s', s @ ..] => {
			super::push(out, 1940);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4723);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7439);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4724);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7440);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 9385);
									s.is_empty()
								}
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
			super::push(out, 1941);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4725);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n2090(s, out),
		['y', s @ ..] => {
			super::push(out, 1943);
			match s {
				[] => true,
				['t', s @ ..] => super::t3::n5066(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ish (2 syllables).
pub(crate) fn n2122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4747);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4748);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4749);
			match s {
				[] => true,
				['a', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7449);
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9388);
							s.is_empty()
						}
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9389);
							s.is_empty()
						}
						['m', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9390);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 10441);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 10442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10443);
									s.is_empty()
								}
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 10444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9391);
							s.is_empty()
						}
						['l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9392);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 10985);
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											super::push(out, 10986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 10445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9393);
							s.is_empty()
						}
						['r', 'o', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 9394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7450);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9395);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10987);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['p', 's', '\u{1}', s @ ..] => {
													super::push(out, 11265);
													s.is_empty()
												}
												['t', 's', '\u{1}', s @ ..] => {
													super::push(out, 11266);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 10988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 10447);
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9397);
							s.is_empty()
						}
						['c', 'o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10449);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 10450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9398);
							s.is_empty()
						}
						['n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9399);
							s.is_empty()
						}
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9400);
							s.is_empty()
						}
						['w', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 10451);
									s.is_empty()
								}
								['e', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 10452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9402);
							s.is_empty()
						}
						['w', 'a', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9404);
							s.is_empty()
						}
						['i', 't', '\u{1}', s @ ..] => {
							super::push(out, 9405);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10453);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9406);
							s.is_empty()
						}
						['w', 'o', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7451);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9408);
							s.is_empty()
						}
						['m', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9409);
							s.is_empty()
						}
						['s', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7452);
					s.is_empty()
				}
				['s', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7453);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10455);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 10456);
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9411);
							s.is_empty()
						}
						['r', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9412);
							s.is_empty()
						}
						['s', 'e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'h', 'd', '\u{1}', s @ ..] => super::t3::n8090(s, out),
				['w', 'o', 'l', 'b', '\u{1}', s @ ..] => super::t3::n8091(s, out),
				['x', 'o', 'f', '\u{1}', s @ ..] => super::t3::n8092(s, out),
				['y', s @ ..] => super::t3::n8093(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n5087(s, out),
		['l', s @ ..] => super::t3::n5088(s, out),
		['m', s @ ..] => super::t3::n5089(s, out),
		['n', s @ ..] => super::t3::n5090(s, out),
		['r', 'i', '\u{1}', s @ ..] => super::t3::n5091(s, out),
		['t', s @ ..] => super::t3::n5092(s, out),
		['w', 's', 't', 'i', 'k', 's', '\u{1}', s @ ..] => super::t3::n5093(s, out),
		_ => false,
	}
}

/// Words ending -sh (3 syllables).
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4759);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7460);
							match s {
								[] => true,
								['e', 'z', 'e', 'q', '\u{1}', s @ ..] => {
									super::push(out, 9418);
									s.is_empty()
								}
								['i', 'z', 'i', s @ ..] => {
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 10458);
											s.is_empty()
										}
										['q', '\u{1}', s @ ..] => {
											super::push(out, 10459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'z', 'y', s @ ..] => {
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 10460);
											s.is_empty()
										}
										['q', '\u{1}', s @ ..] => {
											super::push(out, 10461);
											s.is_empty()
										}
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
					super::push(out, 4760);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7461);
							match s {
								[] => true,
								['a', 'm', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9419);
									s.is_empty()
								}
								['e', 'm', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1977);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4762);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7462);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 9421);
									s.is_empty()
								}
								['g', 'd', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 9422);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 10462);
											s.is_empty()
										}
										['o', 'w', '\u{1}', s @ ..] => {
											super::push(out, 10463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => super::t3::n10304(s, out),
								['m', 'a', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 9423);
									s.is_empty()
								}
								['n', s @ ..] => super::t3::n10306(s, out),
								['r', s @ ..] => super::t3::n10307(s, out),
								['t', s @ ..] => super::t3::n10308(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t3::n8102(s, out),
						['n', s @ ..] => super::t3::n8103(s, out),
						['o', s @ ..] => super::t3::n8104(s, out),
						['r', s @ ..] => super::t3::n8105(s, out),
						['s', s @ ..] => super::t3::n8106(s, out),
						['t', s @ ..] => super::t3::n8107(s, out),
						['w', 'o', 'l', 'l', 'a', 'w', 's', '\u{1}', s @ ..] => super::t3::n8108(s, out),
						['y', s @ ..] => super::t3::n8109(s, out),
						_ => false,
					}
				}
				['m', 'o', s @ ..] => super::t3::n5102(s, out),
				['n', s @ ..] => super::t3::n5103(s, out),
				['r', s @ ..] => super::t3::n5104(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t3::n2128(s, out),
		['u', s @ ..] => super::t3::n2129(s, out),
		_ => false,
	}
}

/// The word necktooth (2 syllables).
pub(crate) fn n12128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10997);
	s.is_empty()
}

/// The word bucktooth (2 syllables).
pub(crate) fn n12129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10998);
	s.is_empty()
}

/// Words ending -shi (3 syllables).
pub(crate) fn n2231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2064);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['m', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9448);
							s.is_empty()
						}
						['n', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9449);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7503);
					match s {
						[] => true,
						['h', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9451);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7504);
					s.is_empty()
				}
				['h', 's', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7505);
					s.is_empty()
				}
				['t', 'k', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7506);
					s.is_empty()
				}
				['v', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 7508);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9453);
							match s {
								[] => true,
								['g', 'n', 'a', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 10999);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 11000);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'n', 'j', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7509);
					s.is_empty()
				}
				['k', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7510);
					s.is_empty()
				}
				['r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4858);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7512);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 9454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7513);
					s.is_empty()
				}
				['r', 'a', 'h', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'd', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4859);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7515);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'e', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 7516);
					s.is_empty()
				}
				['d', 'n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7517);
					s.is_empty()
				}
				['e', 'j', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7518);
					s.is_empty()
				}
				['j', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7519);
					s.is_empty()
				}
				['k', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7520);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9456);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11001);
											s.is_empty()
										}
										['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11002);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9457);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10499);
									s.is_empty()
								}
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
			super::push(out, 4860);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'j', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 7522);
					s.is_empty()
				}
				['h', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7523);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -thi (3 syllables).
pub(crate) fn n2232(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['p', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7524);
					s.is_empty()
				}
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4861);
			s.is_empty()
		}
		['u', 'k', 'o', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ori (3 syllables).
pub(crate) fn n2314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2132);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['m', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 7594);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4961);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4962);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9468);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7596);
					s.is_empty()
				}
				['s', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4963);
			s.is_empty()
		}
		['m', 'a', 'w', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4964);
			s.is_empty()
		}
		['n', 't', 'u', 'y', '\u{1}', s @ ..] => {
			super::push(out, 4965);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4966);
			s.is_empty()
		}
		['p', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 4967);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4968);
			s.is_empty()
		}
		['s', 'n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4969);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7598);
					s.is_empty()
				}
				['o', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7599);
					s.is_empty()
				}
				['s', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4970);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uri (3 syllables).
pub(crate) fn n2315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2133);
	match s {
		[] => true,
		['g', 'a', 't', 'e', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4972);
			s.is_empty()
		}
		['k', 'k', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 4973);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4974);
			match s {
				[] => true,
				['l', 'u', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4975);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4976);
			match s {
				[] => true,
				['i', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 9470);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10502);
									s.is_empty()
								}
								['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10503);
									s.is_empty()
								}
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

/// Words ending -ium (2 syllables).
pub(crate) fn n2560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2367);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7664);
					s.is_empty()
				}
				['m', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'e', s @ ..] => {
			super::push(out, 5142);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7667);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7668);
					s.is_empty()
				}
				['i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7669);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9495);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'a', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5143);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9497);
							s.is_empty()
						}
						['n', 'o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9498);
							s.is_empty()
						}
						['s', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 9500);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7671);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 9502);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7672);
					s.is_empty()
				}
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7673);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 9504);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 9505);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 9506);
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
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9507);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10504);
									s.is_empty()
								}
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 't', s @ ..] => {
					super::push(out, 7674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7675);
					s.is_empty()
				}
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 7676);
					s.is_empty()
				}
				['o', 'u', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'o', s @ ..] => {
					super::push(out, 7678);
					match s {
						[] => true,
						['o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 9508);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 9509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 7679);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7680);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7681);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9511);
							s.is_empty()
						}
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9512);
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
				['n', s @ ..] => {
					super::push(out, 7682);
					s.is_empty()
				}
				['s', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -num (2 syllables).
pub(crate) fn n2564(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5151);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 7687);
					s.is_empty()
				}
				['h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7688);
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7689);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7691);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 7693);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 7694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5152);
			s.is_empty()
		}
		['r', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 5153);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bryozoum (2 syllables).
pub(crate) fn n2565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2371);
	s.is_empty()
}

/// Words ending -pum (2 syllables).
pub(crate) fn n2566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2372);
	s.is_empty()
}

/// Words ending -rum (2 syllables).
pub(crate) fn n2567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2373);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5154);
			s.is_empty()
		}
		['c', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 5155);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5156);
			s.is_empty()
		}
		['h', 't', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7695);
					s.is_empty()
				}
				['i', 'e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7696);
					s.is_empty()
				}
				['r', 'a', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5157);
			match s {
				[] => true,
				['l', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 7698);
					s.is_empty()
				}
				['u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 7699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 7700);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7701);
					s.is_empty()
				}
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7702);
					s.is_empty()
				}
				['l', 'i', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9513);
							s.is_empty()
						}
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 7703);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 9515);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 9516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9517);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9518);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9519);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9520);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9521);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10507);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 10508);
									s.is_empty()
								}
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

/// Words ending -sum (2 syllables).
pub(crate) fn n2568(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5158);
			s.is_empty()
		}
		['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tum (2 syllables).
pub(crate) fn n2569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2374);
	match s {
		[] => true,
		['a', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 5160);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7704);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 9522);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5161);
			s.is_empty()
		}
		['n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5162);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 7706);
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 7707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'e', s @ ..] => {
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7708);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'y', 'x', '\u{1}', s @ ..] => {
			super::push(out, 5163);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7710);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 7711);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vum (2 syllables).
pub(crate) fn n2570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2375);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5165);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 5166);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word saxum (2 syllables).
pub(crate) fn n2571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2376);
	s.is_empty()
}

/// Words ending -zum (2 syllables).
pub(crate) fn n2572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2377);
	match s {
		[] => true,
		['t', 'm', 'i', 'z', 't', '\u{1}', s @ ..] => {
			super::push(out, 5167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -um (3 syllables).
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2378);
			match s {
				[] => true,
				['b', 'n', 'e', 'n', 'n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 5168);
					s.is_empty()
				}
				['n', 'r', 'a', 'h', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2379);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2380);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7712);
							s.is_empty()
						}
						['x', 'a', 'r', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7714);
							s.is_empty()
						}
						['h', 'c', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7715);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9524);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 10509);
											s.is_empty()
										}
										['y', 'c', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10510);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 10511);
											s.is_empty()
										}
										['r', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 10512);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7716);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7717);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 9526);
									s.is_empty()
								}
								['c', 'a', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 10513);
											s.is_empty()
										}
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10514);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7718);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5171);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'u', 'c', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5172);
					s.is_empty()
				}
				['e', 's', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5173);
					s.is_empty()
				}
				['i', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 5174);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5175);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7721);
							match s {
								[] => true,
								['r', 'o', 'm', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9528);
									s.is_empty()
								}
								['s', 'i', 'v', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9529);
									s.is_empty()
								}
								['z', 'i', 'v', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9531);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['d', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 10515);
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 10516);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 9532);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 9533);
									match s {
										[] => true,
										['f', s @ ..] => {
											match s {
												[] => true,
												['e', 'r', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11003);
													s.is_empty()
												}
												['f', 'i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 11004);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 10517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9534);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9535);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10518);
											s.is_empty()
										}
										['u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10519);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'o', 'p', 'o', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', s @ ..] => super::t3::n8401(s, out),
						['g', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7723);
							s.is_empty()
						}
						['n', 'e', 'h', 't', 'a', '\u{1}', s @ ..] => super::t3::n8403(s, out),
						_ => false,
					}
				}
				['c', 'e', 'a', 'n', 'y', 'g', '\u{1}', s @ ..] => super::t3::n5573(s, out),
				['d', 'o', 't', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => super::t3::n5574(s, out),
				['g', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => super::t3::n5575(s, out),
				['l', s @ ..] => super::t3::n5576(s, out),
				['n', s @ ..] => super::t3::n5577(s, out),
				['p', s @ ..] => super::t3::n5578(s, out),
				['r', s @ ..] => super::t3::n5579(s, out),
				['s', s @ ..] => super::t3::n5580(s, out),
				['t', 's', 'o', 'n', 'e', 'o', 'c', '\u{1}', s @ ..] => super::t3::n5581(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t3::n2578(s, out),
		['h', s @ ..] => super::t3::n2579(s, out),
		['i', s @ ..] => super::t3::n2580(s, out),
		['j', 'o', 'c', 'u', 'e', 'l', '\u{1}', s @ ..] => super::t3::n2581(s, out),
		['k', 'o', 'o', 'h', 'l', 'l', 'u', 'b', 's', 'u', 'h', '\u{1}', s @ ..] => super::t3::n2582(s, out),
		['l', s @ ..] => super::t3::n2583(s, out),
		['m', s @ ..] => super::t3::n2584(s, out),
		['n', s @ ..] => super::t3::n2585(s, out),
		['p', 'a', 'n', 'a', 'w', '\u{1}', s @ ..] => super::t3::n2586(s, out),
		['r', s @ ..] => super::t3::n2587(s, out),
		['s', s @ ..] => super::t3::n2588(s, out),
		['t', s @ ..] => super::t3::n2589(s, out),
		['u', s @ ..] => super::t3::n2590(s, out),
		['v', s @ ..] => super::t3::n2591(s, out),
		['z', 'z', 'i', 'c', 'y', 'a', 'r', '\u{1}', s @ ..] => super::t3::n2592(s, out),
		_ => false,
	}
}

/// The word saidan (2 syllables).
pub(crate) fn n8599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7873);
	s.is_empty()
}

/// The word lamdan (2 syllables).
pub(crate) fn n5663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5239);
	s.is_empty()
}

/// Words ending -ean (2 syllables).
pub(crate) fn n2605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2406);
	s.is_empty()
}

/// Words ending -fan (2 syllables).
pub(crate) fn n2606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2407);
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			super::push(out, 5240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gan (2 syllables).
pub(crate) fn n2607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2408);
	match s {
		[] => true,
		['n', 'a', s @ ..] => {
			super::push(out, 5241);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 7874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -han (2 syllables).
pub(crate) fn n2608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2409);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 5242);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 7875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ian (2 syllables).
pub(crate) fn n2609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2410);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5244);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5245);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7877);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 5246);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7878);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 9729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jan (2 syllables).
pub(crate) fn n2610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2411);
	s.is_empty()
}

/// Words ending -kan (2 syllables).
pub(crate) fn n2611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2412);
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 5247);
			s.is_empty()
		}
		['h', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5248);
			s.is_empty()
		}
		['i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5249);
			s.is_empty()
		}
		['k', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5250);
			s.is_empty()
		}
		['o', 'y', '\u{1}', s @ ..] => {
			super::push(out, 5251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lan (2 syllables).
pub(crate) fn n2612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2413);
	s.is_empty()
}

/// Words ending -man (2 syllables).
pub(crate) fn n2613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2414);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5252);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 7879);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 7880);
					s.is_empty()
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7881);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 9730);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 7882);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9732);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7883);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 7884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5253);
			match s {
				[] => true,
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 7885);
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5254);
			match s {
				[] => true,
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7887);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 7888);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9734);
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
					super::push(out, 7889);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 7891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7892);
					s.is_empty()
				}
				['o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5255);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7894);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9736);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9738);
							s.is_empty()
						}
						['o', 'o', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9739);
							s.is_empty()
						}
						['t', 'o', 'o', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7895);
					s.is_empty()
				}
				['t', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5256);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7897);
					s.is_empty()
				}
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7898);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7899);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 7900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5257);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7901);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 9741);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 9742);
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
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9743);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 9744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7902);
					s.is_empty()
				}
				['r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 7903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7904);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9745);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 7906);
					s.is_empty()
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5258);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5259);
			match s {
				[] => true,
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7908);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9747);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 7910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5260);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7911);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 9749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7912);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 9750);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 9751);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9753);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9754);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 9756);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9758);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9759);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5262);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7914);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 9761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 7915);
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
						['\u{1}', s @ ..] => {
							super::push(out, 9762);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7916);
					s.is_empty()
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7917);
					s.is_empty()
				}
				['r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5263);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 7919);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7920);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7921);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7922);
					s.is_empty()
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 7923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5264);
			s.is_empty()
		}
		['y', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nan (2 syllables).
pub(crate) fn n2614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2415);
	match s {
		[] => true,
		['e', 'x', '\u{1}', s @ ..] => {
			super::push(out, 5266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oan (2 syllables).
pub(crate) fn n2615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2416);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 5267);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7924);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9764);
							s.is_empty()
						}
						['y', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pan (2 syllables).
pub(crate) fn n2616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2417);
	match s {
		[] => true,
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5268);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5269);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 7925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ran (2 syllables).
pub(crate) fn n2617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2418);
	match s {
		[] => true,
		['i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -san (2 syllables).
pub(crate) fn n2618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2419);
	match s {
		[] => true,
		['k', 't', 'i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tan (2 syllables).
pub(crate) fn n2619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2420);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5273);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'y', s @ ..] => {
			super::push(out, 5274);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7927);
					s.is_empty()
				}
				['y', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uan (2 syllables).
pub(crate) fn n2620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2421);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5276);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5277);
			s.is_empty()
		}
		['r', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5278);
			s.is_empty()
		}
		['t', 'g', 'n', 'i', 'q', '\u{1}', s @ ..] => {
			super::push(out, 5279);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5280);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7929);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -van (2 syllables).
pub(crate) fn n2621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2422);
	s.is_empty()
}

/// Words ending -wan (2 syllables).
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2423);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yan (2 syllables).
pub(crate) fn n2623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2424);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5282);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5283);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9767);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9768);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zan (2 syllables).
pub(crate) fn n2624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2425);
	s.is_empty()
}

/// Words ending -an (3 syllables).
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2426);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 5284);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2428);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5286);
					match s {
						[] => true,
						['s', 't', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2429);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2430);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2431);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7934);
							match s {
								[] => true,
								['r', 'a', 't', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'r', 'o', s @ ..] => {
							super::push(out, 7935);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 9771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', 'b', s @ ..] => {
					super::push(out, 5287);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5288);
					match s {
						[] => true,
						['k', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2432);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2433);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5289);
					match s {
						[] => true,
						['i', 'r', 'o', s @ ..] => {
							super::push(out, 7938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5290);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7939);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2434);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2435);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['d', 'a', 't', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 5293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5294);
					match s {
						[] => true,
						['b', 'a', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7942);
							s.is_empty()
						}
						['c', 'a', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7943);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7944);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 7945);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7946);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 9774);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7947);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9776);
									s.is_empty()
								}
								['r', 'a', 'y', '\u{1}', s @ ..] => {
									super::push(out, 9777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'h', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7948);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9778);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 9779);
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5295);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5296);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 9781);
									s.is_empty()
								}
								['r', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 9782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 9783);
									s.is_empty()
								}
								['s', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5297);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5298);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7949);
							s.is_empty()
						}
						['d', 'n', 'u', 'k', '\u{1}', s @ ..] => super::t3::n8689(s, out),
						['h', 's', 'o', 'r', 'i', 'h', '\u{1}', s @ ..] => super::t3::n8690(s, out),
						['k', 'a', 'y', '\u{1}', s @ ..] => super::t3::n8691(s, out),
						['l', 'o', 'o', 'k', '\u{1}', s @ ..] => super::t3::n8692(s, out),
						['r', 'e', s @ ..] => super::t3::n8693(s, out),
						['s', 'i', 'm', '\u{1}', s @ ..] => super::t3::n8694(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n5730(s, out),
				['m', 'a', 'd', 'n', 'a', 'l', '\u{1}', s @ ..] => super::t3::n5731(s, out),
				['o', s @ ..] => super::t3::n5732(s, out),
				['r', s @ ..] => super::t3::n5733(s, out),
				['s', s @ ..] => super::t3::n5734(s, out),
				['t', s @ ..] => super::t3::n5735(s, out),
				['u', s @ ..] => super::t3::n5736(s, out),
				['y', s @ ..] => super::t3::n5737(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t3::n2637(s, out),
		['o', s @ ..] => super::t3::n2638(s, out),
		['p', s @ ..] => super::t3::n2639(s, out),
		['r', s @ ..] => super::t3::n2640(s, out),
		['s', s @ ..] => super::t3::n2641(s, out),
		['t', s @ ..] => super::t3::n2642(s, out),
		['u', s @ ..] => super::t3::n2643(s, out),
		['v', s @ ..] => super::t3::n2644(s, out),
		['w', s @ ..] => super::t3::n2645(s, out),
		['x', s @ ..] => super::t3::n2646(s, out),
		['y', s @ ..] => super::t3::n2647(s, out),
		['z', s @ ..] => super::t3::n2648(s, out),
		_ => false,
	}
}

/// The word demonkin (3 syllables).
pub(crate) fn n10796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9832);
	s.is_empty()
}

/// Words ending -herkin (3 syllables).
pub(crate) fn n5814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5380);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 8009);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 9833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -skin (3 syllables).
pub(crate) fn n5815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5381);
	s.is_empty()
}

/// Words ending -ukin (3 syllables).
pub(crate) fn n5816(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'i', 'h', 'c', 'i', '\u{1}', s @ ..] => {
			super::push(out, 8010);
			s.is_empty()
		}
		['k', 'a', 't', 'o', '\u{1}', s @ ..] => {
			super::push(out, 8011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lin (3 syllables).
pub(crate) fn n2704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2503);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5382);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5383);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 8012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5384);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8013);
					match s {
						[] => true,
						['n', 'u', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 9834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8014);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9835);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10718);
									s.is_empty()
								}
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

/// Words ending -min (3 syllables).
pub(crate) fn n2705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2504);
	match s {
		[] => true,
		['e', 'l', s @ ..] => {
			super::push(out, 5385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nin (3 syllables).
pub(crate) fn n2706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2505);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			super::push(out, 5386);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'k', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5387);
			s.is_empty()
		}
		['i', 'r', s @ ..] => {
			super::push(out, 5388);
			match s {
				[] => true,
				['n', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', 'o', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 5389);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rrin (3 syllables).
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2506);
	match s {
		[] => true,
		['e', 'h', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -misin (3 syllables).
pub(crate) fn n2708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2507);
	match s {
		[] => true,
		['i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ratin (3 syllables).
pub(crate) fn n2709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2508);
	match s {
		[] => true,
		['a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5392);
			s.is_empty()
		}
		['r', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -win (3 syllables).
pub(crate) fn n2710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2509);
	s.is_empty()
}

/// Words ending -xin (3 syllables).
pub(crate) fn n2711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2510);
	s.is_empty()
}

/// Words ending -zin (3 syllables).
pub(crate) fn n2712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2511);
	s.is_empty()
}

/// The word pereion (2 syllables).
pub(crate) fn n8779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8030);
	s.is_empty()
}

/// Words ending -gion (2 syllables).
pub(crate) fn n5848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5413);
	match s {
		[] => true,
		['o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 8031);
			s.is_empty()
		}
		['r', 'o', '\u{1}', s @ ..] => {
			super::push(out, 8032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lion (2 syllables).
pub(crate) fn n5849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5414);
	match s {
		[] => true,
		['g', 'n', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 8033);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9836);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8034);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9838);
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9839);
					s.is_empty()
				}
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 9840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 8035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -emion (2 syllables).
pub(crate) fn n5850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5415);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 8036);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9841);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inion (2 syllables).
pub(crate) fn n5851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5416);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8037);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8038);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9843);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 10719);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pion (2 syllables).
pub(crate) fn n5852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5417);
	s.is_empty()
}

/// Words ending -rion (2 syllables).
pub(crate) fn n5853(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8039);
			s.is_empty()
		}
		['u', 'o', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8040);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word grosion (2 syllables).
pub(crate) fn n5854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5418);
	s.is_empty()
}

/// Words ending -tion (2 syllables).
pub(crate) fn n5855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5419);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8041);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 9844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jon (2 syllables).
pub(crate) fn n2729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2527);
	match s {
		[] => true,
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 5420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kon (2 syllables).
pub(crate) fn n2730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2528);
	s.is_empty()
}

/// Words ending -lon (2 syllables).
pub(crate) fn n2731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2529);
	match s {
		[] => true,
		['h', 'p', 'y', 't', '\u{1}', s @ ..] => {
			super::push(out, 5421);
			s.is_empty()
		}
		['l', 'y', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5422);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5423);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5424);
			match s {
				[] => true,
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8042);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 8043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mon (2 syllables).
pub(crate) fn n2732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2530);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5425);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5426);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 8045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 5427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -non (2 syllables).
pub(crate) fn n2733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2531);
	match s {
		[] => true,
		['a', 'o', 'x', '\u{1}', s @ ..] => {
			super::push(out, 5428);
			s.is_empty()
		}
		['c', 'y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5429);
			s.is_empty()
		}
		['i', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oon (2 syllables).
pub(crate) fn n2734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2532);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 5431);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8046);
					match s {
						[] => true,
						['y', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pon (2 syllables).
pub(crate) fn n2735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2533);
	s.is_empty()
}

/// Words ending -ron (2 syllables).
pub(crate) fn n2736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2534);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5432);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5433);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9846);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5434);
			match s {
				[] => true,
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 8048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 8049);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5435);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8051);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -son (2 syllables).
pub(crate) fn n2737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2535);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5436);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 8052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5437);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ton (2 syllables).
pub(crate) fn n2738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2536);
	match s {
		[] => true,
		['i', 'h', s @ ..] => {
			super::push(out, 5438);
			s.is_empty()
		}
		['k', 'n', s @ ..] => {
			super::push(out, 5439);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5440);
			match s {
				[] => true,
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5441);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5442);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word taxon (2 syllables).
pub(crate) fn n2739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2537);
	s.is_empty()
}

/// Words ending -yon (2 syllables).
pub(crate) fn n2740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2538);
	match s {
		[] => true,
		['n', 'i', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zon (2 syllables).
pub(crate) fn n2741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2539);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5444);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8057);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on (3 syllables).
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2540);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'h', 't', 'e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8058);
							s.is_empty()
						}
						['r', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'o', 'h', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8060);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 9850);
									match s {
										[] => true,
										['t', 'e', '\u{1}', s @ ..] => {
											super::push(out, 10721);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'c', 'a', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'y', 't', '\u{1}', s @ ..] => {
							super::push(out, 8061);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8062);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 'c', 'o', 'r', 'e', 't', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9852);
									s.is_empty()
								}
								['o', 'i', 'd', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 8063);
							match s {
								[] => true,
								['e', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10722);
											s.is_empty()
										}
										['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10723);
											s.is_empty()
										}
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
			super::push(out, 2542);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5445);
					match s {
						[] => true,
						['b', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8064);
							s.is_empty()
						}
						['p', 's', 'a', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2543);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5446);
					match s {
						[] => true,
						['h', 'c', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9854);
									s.is_empty()
								}
								['o', 'l', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'o', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5447);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8066);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5448);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8068);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['h', 'p', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10724);
											s.is_empty()
										}
										['r', 'u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11122);
													s.is_empty()
												}
												['e', 'l', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11123);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10725);
											s.is_empty()
										}
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
		['g', s @ ..] => {
			super::push(out, 2544);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5449);
					s.is_empty()
				}
				['o', 'l', s @ ..] => {
					super::push(out, 5450);
					match s {
						[] => true,
						['a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8069);
							s.is_empty()
						}
						['o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2545);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5451);
					match s {
						[] => true,
						['i', 'j', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5452);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5453);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8072);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9856);
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 11124);
													s.is_empty()
												}
												['i', 'p', 'e', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11125);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'r', 'c', 'o', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5454);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8074);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8075);
							match s {
								[] => true,
								['l', 'o', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9857);
									s.is_empty()
								}
								['o', 'l', 'o', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n2748(s, out),
		['k', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['a', 'r', 't', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5469);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'h', 't', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8118);
							s.is_empty()
						}
						['p', 'y', 't', '\u{1}', s @ ..] => {
							super::push(out, 8119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2548);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8120);
							match s {
								[] => true,
								['p', 'e', 'c', s @ ..] => super::t3::n10893(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'm', 'o', s @ ..] => super::t3::n5912(s, out),
				['i', 't', s @ ..] => {
					super::push(out, 5472);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5473);
					s.is_empty()
				}
				['o', s @ ..] => super::t3::n5915(s, out),
				['y', s @ ..] => super::t3::n5916(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n2751(s, out),
		['n', s @ ..] => super::t3::n2752(s, out),
		['o', s @ ..] => super::t3::n2753(s, out),
		['p', s @ ..] => super::t3::n2754(s, out),
		['r', s @ ..] => super::t3::n2755(s, out),
		['s', s @ ..] => super::t3::n2756(s, out),
		['t', s @ ..] => super::t3::n2757(s, out),
		['x', s @ ..] => super::t3::n2758(s, out),
		_ => false,
	}
}

/// The word stratovolcano (3 syllables).
pub(crate) fn n12350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11191);
	s.is_empty()
}

/// The word cryovolcano (3 syllables).
pub(crate) fn n12351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11192);
	s.is_empty()
}

/// The word supervolcano (3 syllables).
pub(crate) fn n12352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11193);
	s.is_empty()
}

/// The word hypervolcano (3 syllables).
pub(crate) fn n12353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11194);
	s.is_empty()
}

/// The word peperoncino (3 syllables).
pub(crate) fn n12354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11195);
	s.is_empty()
}

/// The word pepperoncino (3 syllables).
pub(crate) fn n12355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11196);
	s.is_empty()
}

/// Words ending -mino (3 syllables).
pub(crate) fn n6104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5646);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8259);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8260);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8261);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9993);
					s.is_empty()
				}
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9994);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9995);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word bolognino (3 syllables).
pub(crate) fn n9036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8263);
	s.is_empty()
}

/// The word alpino (3 syllables).
pub(crate) fn n6106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5648);
	s.is_empty()
}

/// Words ending -rino (3 syllables).
pub(crate) fn n6107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5649);
	match s {
		[] => true,
		['e', 'h', s @ ..] => {
			super::push(out, 8264);
			match s {
				[] => true,
				['c', 'c', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 9998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', s @ ..] => {
			super::push(out, 8265);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9999);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10806);
							match s {
								[] => true,
								['m', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 11197);
									s.is_empty()
								}
								['n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 11198);
									s.is_empty()
								}
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

/// The word grissino (3 syllables).
pub(crate) fn n6108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5650);
	s.is_empty()
}

/// Words ending -tino (3 syllables).
pub(crate) fn n6109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5651);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8266);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8267);
			match s {
				[] => true,
				['a', 'g', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 10000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ottavino (3 syllables).
pub(crate) fn n6110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5652);
	s.is_empty()
}

/// Words ending -zino (3 syllables).
pub(crate) fn n6111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5653);
	match s {
		[] => true,
		['z', 'e', 'm', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 8268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ono (3 syllables).
pub(crate) fn n2893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	match s {
		[] => true,
		['d', 'i', 'r', 'u', 's', 't', '\u{1}', s @ ..] => {
			super::push(out, 5654);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 10001);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10807);
									s.is_empty()
								}
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 10808);
									s.is_empty()
								}
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

/// Words ending -orno (3 syllables).
pub(crate) fn n2894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	match s {
		[] => true,
		['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -affito (3 syllables).
pub(crate) fn n11004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10010);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 11199);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11343);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lito (3 syllables).
pub(crate) fn n6166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5707);
	s.is_empty()
}

/// The word bonito (3 syllables).
pub(crate) fn n6167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5708);
	s.is_empty()
}

/// Words ending -orito (3 syllables).
pub(crate) fn n6168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5709);
	match s {
		[] => true,
		['n', '\u{1}', s @ ..] => {
			super::push(out, 8302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tito (3 syllables).
pub(crate) fn n6169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5710);
	match s {
		[] => true,
		['n', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uito (3 syllables).
pub(crate) fn n6170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5711);
	match s {
		[] => true,
		['q', 's', s @ ..] => {
			super::push(out, 8304);
			match s {
				[] => true,
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10011);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word filakto (3 syllables).
pub(crate) fn n2938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2729);
	s.is_empty()
}

/// Words ending -lto (3 syllables).
pub(crate) fn n2939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2730);
	match s {
		[] => true,
		['a', 'v', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nto (3 syllables).
pub(crate) fn n2940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2731);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5713);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8305);
					s.is_empty()
				}
				['m', 'i', s @ ..] => {
					super::push(out, 8306);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 10012);
							match s {
								[] => true,
								['n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10809);
									s.is_empty()
								}
								['r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oto (3 syllables).
pub(crate) fn n2941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2732);
	s.is_empty()
}

/// Words ending -sto (3 syllables).
pub(crate) fn n2942(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5715);
			s.is_empty()
		}
		['o', 'p', 'p', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tto (3 syllables).
pub(crate) fn n2943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2733);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8308);
					s.is_empty()
				}
				['d', 'u', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8309);
					s.is_empty()
				}
				['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8310);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'a', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 10013);
							s.is_empty()
						}
						['g', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 10014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'g', 'i', 'r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8311);
					s.is_empty()
				}
				['m', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8312);
					s.is_empty()
				}
				['n', 'g', 'a', 'c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8313);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['p', 'u', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10015);
							s.is_empty()
						}
						['u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8314);
					s.is_empty()
				}
				['z', 'z', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'd', 'n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5718);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 's', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8316);
					s.is_empty()
				}
				['i', 'c', 'c', 'i', 't', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8317);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8318);
					match s {
						[] => true,
						['a', 'z', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10017);
							s.is_empty()
						}
						['e', 'z', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'i', 'c', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uto (3 syllables).
pub(crate) fn n2944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2734);
	s.is_empty()
}

/// Words ending -er (3 syllables).
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 930);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2807);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2808);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5780);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 8352);
							match s {
								[] => true,
								['e', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 10029);
									s.is_empty()
								}
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
			super::push(out, 2809);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2810);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5782);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 8353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5783);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2812);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5784);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 8355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2813);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2814);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2815);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5787);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8356);
							match s {
								[] => true,
								['s', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10030);
									s.is_empty()
								}
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
			super::push(out, 2816);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5788);
					match s {
						[] => true,
						['a', 'b', 'o', 't', s @ ..] => {
							super::push(out, 8357);
							match s {
								[] => true,
								['o', 'z', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5789);
					match s {
						[] => true,
						['a', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2817);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2818);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8359);
							s.is_empty()
						}
						['k', 'c', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word cacoethes (3 syllables).
pub(crate) fn n9184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8405);
	s.is_empty()
}

/// Words ending -ies (3 syllables).
pub(crate) fn n3174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2957);
	s.is_empty()
}

/// Words ending -les (3 syllables).
pub(crate) fn n3175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2958);
	match s {
		[] => true,
		['l', 'u', 'c', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mes (3 syllables).
pub(crate) fn n3176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2959);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5920);
			match s {
				[] => true,
				['c', 'e', 'c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8406);
					s.is_empty()
				}
				['r', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 8407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nes (3 syllables).
pub(crate) fn n3177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2960);
	match s {
		[] => true,
		['n', 'a', 'h', 'o', 'j', '\u{1}', s @ ..] => {
			super::push(out, 5921);
			s.is_empty()
		}
		['t', 'n', 'a', 's', 'a', 'e', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oes (3 syllables).
pub(crate) fn n3178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2961);
	match s {
		[] => true,
		['d', 'n', 'e', 'p', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 5923);
			s.is_empty()
		}
		['m', 'l', 'e', 'p', 'm', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5924);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word jackanapes (3 syllables).
pub(crate) fn n3179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2962);
	s.is_empty()
}

/// Words ending -ses (3 syllables).
pub(crate) fn n3180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2963);
	s.is_empty()
}

/// Words ending -tes (3 syllables).
pub(crate) fn n3181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2964);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 5925);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5926);
			match s {
				[] => true,
				['h', 't', 'o', 'n', 'o', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8409);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8410);
					s.is_empty()
				}
				['r', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5927);
			s.is_empty()
		}
		['o', 'x', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 5928);
			s.is_empty()
		}
		['r', 'a', 'h', 't', 'a', 'c', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5929);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5930);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8412);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10042);
							match s {
								[] => true,
								['e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 'l', 'u', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5931);
			s.is_empty()
		}
		['y', 'l', 'b', 'o', 'r', 'y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmis (2 syllables).
pub(crate) fn n6435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5964);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ermis (2 syllables).
pub(crate) fn n6437(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 8424);
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 8425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nis (2 syllables).
pub(crate) fn n3222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3004);
	match s {
		[] => true,
		['e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5966);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5967);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 8426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ois (2 syllables).
pub(crate) fn n3223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3005);
	match s {
		[] => true,
		['k', 'r', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 5969);
			s.is_empty()
		}
		['m', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5970);
			s.is_empty()
		}
		['n', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5971);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5972);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5973);
			s.is_empty()
		}
		['u', 'q', 'r', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 5974);
			s.is_empty()
		}
		['v', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 5975);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pis (2 syllables).
pub(crate) fn n3224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3006);
	match s {
		[] => true,
		['l', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5976);
			s.is_empty()
		}
		['o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5977);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 8427);
					s.is_empty()
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ris (2 syllables).
pub(crate) fn n3225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3007);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5978);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 8429);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 8430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 's', 'i', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		['p', 'y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5980);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sis (2 syllables).
pub(crate) fn n3226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3008);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8431);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5983);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5984);
			s.is_empty()
		}
		['i', '\u{1}', s @ ..] => {
			super::push(out, 5985);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5986);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5987);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 8433);
					s.is_empty()
				}
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 8434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5988);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10043);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 5989);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tis (2 syllables).
pub(crate) fn n3227(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5990);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8436);
					s.is_empty()
				}
				['y', 'l', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5991);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5992);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5993);
			s.is_empty()
		}
		['s', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 5994);
			s.is_empty()
		}
		['u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uis (2 syllables).
pub(crate) fn n3228(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 5996);
			s.is_empty()
		}
		['n', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 5997);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8439);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vis (2 syllables).
pub(crate) fn n3229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3009);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5999);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8440);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8442);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8443);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wis (2 syllables).
pub(crate) fn n3230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3010);
	s.is_empty()
}

/// Words ending -xis (2 syllables).
pub(crate) fn n3231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3011);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6001);
			s.is_empty()
		}
		['e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6002);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 6003);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 8444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -is (3 syllables).
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 972);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3012);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6004);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3013);
			match s {
				[] => true,
				['a', 'n', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6006);
					s.is_empty()
				}
				['r', 'o', 'n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3014);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', 'b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8445);
							s.is_empty()
						}
						['u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3015);
			match s {
				[] => true,
				['l', 'l', 'u', 'c', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6008);
					s.is_empty()
				}
				['r', 'k', 'n', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3016);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6010);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'r', 'e', 'a', 'h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 6011);
					s.is_empty()
				}
				['t', 'a', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3017);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6013);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8448);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8449);
							s.is_empty()
						}
						['e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8450);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8451);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8452);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10045);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 10813);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 10814);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 8453);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'm', 'u', 'h', 'i', 'h', 'p', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 10047);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10048);
									s.is_empty()
								}
								['y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8454);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10050);
									s.is_empty()
								}
								['p', 'i', 'h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 10051);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 10052);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8455);
							match s {
								[] => true,
								['i', 'g', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6014);
					match s {
						[] => true,
						['h', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', 'r', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8458);
							s.is_empty()
						}
						['c', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6015);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8460);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 8461);
							match s {
								[] => true,
								['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 10055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6017);
					match s {
						[] => true,
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8462);
							s.is_empty()
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3018);
			match s {
				[] => true,
				['a', 'r', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6018);
					s.is_empty()
				}
				['e', 'h', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6019);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 6020);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10056);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 10816);
											s.is_empty()
										}
										['r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 10817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'd', 'i', 'd', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3019);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6022);
					match s {
						[] => true,
						['c', 'i', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8464);
							s.is_empty()
						}
						['t', 'y', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6023);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 10057);
									s.is_empty()
								}
								['i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10058);
									s.is_empty()
								}
								['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'l', 'a', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6024);
					s.is_empty()
				}
				['n', 'e', 'p', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6025);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6026);
					match s {
						[] => true,
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6027);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3020);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6028);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6029);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6030);
					match s {
						[] => true,
						['i', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', 'd', 'r', 'i', 'o', 'v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6031);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6032);
					match s {
						[] => true,
						['a', 'v', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6033);
					match s {
						[] => true,
						['n', s @ ..] => super::t3::n9254(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t3::n3241(s, out),
		['r', s @ ..] => super::t3::n3242(s, out),
		['s', s @ ..] => super::t3::n3243(s, out),
		['t', s @ ..] => super::t3::n3244(s, out),
		['v', s @ ..] => super::t3::n3245(s, out),
		['x', s @ ..] => super::t4::n3246(s, out),
		_ => false,
	}
}

/// Words ending -os (3 syllables).
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3091);
			match s {
				[] => true,
				['a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6138);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6139);
					match s {
						[] => true,
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'i', 'l', 'o', 'h', 't', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3092);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3093);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6141);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8549);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6142);
					match s {
						[] => true,
						['n', 'r', 'u', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 8550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'o', 's', 'p', 'a', 'h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6143);
					s.is_empty()
				}
				['o', 'n', 'a', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3094);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3095);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3096);
			match s {
				[] => true,
				['e', 'i', 'j', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6145);
					s.is_empty()
				}
				['i', 'l', 'o', 'h', 't', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6146);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6147);
					match s {
						[] => true,
						['i', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3097);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6148);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8552);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 10112);
									s.is_empty()
								}
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'h', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6149);
					s.is_empty()
				}
				['l', 'a', 'b', 'y', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3098);
			match s {
				[] => true,
				['l', 'a', 'h', 't', 'h', 'p', 'o', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6152);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6153);
					s.is_empty()
				}
				['r', 'e', 'd', 'o', 'p', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				['s', 'o', 'c', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3099);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6156);
					match s {
						[] => true,
						['b', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3100);
			match s {
				[] => true,
				['f', 'a', 'x', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				['l', 'u', 'c', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 6159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', s @ ..] => {
			super::push(out, 3101);
			match s {
				[] => true,
				['o', 'c', 'n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3102);
			match s {
				[] => true,
				['d', 'y', 'h', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6162);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8554);
							match s {
								[] => true,
								['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10114);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10115);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', 'h', 'r', '\u{1}', s @ ..] => {
													super::push(out, 11217);
													s.is_empty()
												}
												['o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 11218);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'i', 's', 'o', 'r', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8555);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8556);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6163);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8557);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 8558);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10117);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 10850);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 11219);
													s.is_empty()
												}
												['e', '\u{1}', s @ ..] => {
													super::push(out, 11220);
													s.is_empty()
												}
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
				['t', 'a', 'b', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6164);
					s.is_empty()
				}
				['y', 'p', 's', 'o', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6166);
					s.is_empty()
				}
				['s', 'e', 'b', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -us (2 syllables).
pub(crate) fn n1083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3184);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6219);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6220);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 8574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6221);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6222);
					match s {
						[] => true,
						['a', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8575);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 10124);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 10125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 8576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6223);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6224);
					match s {
						[] => true,
						['i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 6225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3185);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6226);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6227);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6228);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 8579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6229);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8580);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 8581);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 8582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 8583);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6230);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 8585);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6231);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 8587);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 10127);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 10128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					super::push(out, 6232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3186);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8589);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3187);
			match s {
				[] => true,
				['c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6233);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 8591);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8592);
							s.is_empty()
						}
						['u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6234);
					match s {
						[] => true,
						['p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6235);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8595);
							s.is_empty()
						}
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8596);
							s.is_empty()
						}
						['u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'l', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 8598);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 3188);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3189);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6236);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8600);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 10129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 8601);
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 10130);
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 10131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8603);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 8604);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 8605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6238);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8606);
							s.is_empty()
						}
						['r', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 8607);
							s.is_empty()
						}
						['y', 't', '\u{1}', s @ ..] => {
							super::push(out, 8608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', 'i', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8609);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8611);
							s.is_empty()
						}
						['h', 'c', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8612);
							s.is_empty()
						}
						['n', 'a', 'i', s @ ..] => {
							super::push(out, 8613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3191);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6239);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8614);
							s.is_empty()
						}
						['r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 8615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t4::n6730(s, out),
				['l', 'l', 'o', 'r', 't', '\u{1}', s @ ..] => super::t4::n6731(s, out),
				['m', 'r', 'e', 'b', '\u{1}', s @ ..] => super::t4::n6732(s, out),
				['n', s @ ..] => super::t4::n6733(s, out),
				['s', 'l', 'e', 'c', '\u{1}', s @ ..] => super::t4::n6734(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t4::n3421(s, out),
		['l', s @ ..] => super::t4::n3422(s, out),
		['m', s @ ..] => super::t4::n3423(s, out),
		['n', s @ ..] => super::t4::n3424(s, out),
		['o', s @ ..] => super::t4::n3425(s, out),
		['p', s @ ..] => super::t4::n3426(s, out),
		['r', s @ ..] => super::t4::n3427(s, out),
		['s', s @ ..] => super::t4::n3428(s, out),
		['t', s @ ..] => super::t4::n3429(s, out),
		['u', 't', 'i', 'l', '\u{1}', s @ ..] => super::t4::n3430(s, out),
		['v', s @ ..] => super::t4::n3431(s, out),
		['x', s @ ..] => super::t4::n3432(s, out),
		_ => false,
	}
}

/// Words ending -us (3 syllables).
pub(crate) fn n1084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3204);
			match s {
				[] => true,
				['a', 'l', 'l', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 6284);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8663);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6285);
					match s {
						[] => true,
						['r', 't', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6286);
					match s {
						[] => true,
						['a', 'i', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8666);
							s.is_empty()
						}
						['i', 'n', 'o', 'l', 'u', 'm', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8667);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8668);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6287);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6288);
					s.is_empty()
				}
				['u', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8671);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3205);
			match s {
				[] => true,
				['a', 'm', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6289);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6290);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6291);
					match s {
						[] => true,
						['d', 'n', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8674);
							s.is_empty()
						}
						['t', 'r', s @ ..] => {
							super::push(out, 8675);
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10852);
											s.is_empty()
										}
										['i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 10853);
											s.is_empty()
										}
										['o', 't', 'p', 'y', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10854);
											s.is_empty()
										}
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
						['d', 'o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8676);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10151);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 10855);
											s.is_empty()
										}
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10856);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8677);
							s.is_empty()
						}
						['p', 's', 'u', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8679);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10153);
									s.is_empty()
								}
								['m', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'a', 'c', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6292);
					s.is_empty()
				}
				['y', 'l', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3206);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8680);
							s.is_empty()
						}
						['l', 'c', 'o', 't', 'p', 'y', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3207);
			match s {
				[] => true,
				['a', 'h', 'p', 'y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6295);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6296);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6297);
					s.is_empty()
				}
				['h', 'p', 'y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6298);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8682);
							s.is_empty()
						}
						['o', 't', 's', 'i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6299);
					s.is_empty()
				}
				['r', 'o', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3208);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6301);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 8684);
							s.is_empty()
						}
						['r', 'a', 'p', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'a', 't', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6302);
					s.is_empty()
				}
				['l', 'o', 'm', 'o', 'n', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6303);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8686);
							match s {
								[] => true,
								['l', 'i', 'n', 'n', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10155);
									s.is_empty()
								}
								['p', 's', 'o', 'r', 'o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8687);
							s.is_empty()
						}
						['u', 'f', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', 'e', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 10157);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10158);
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
						['b', 'a', 'r', 'e', 'k', 'n', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 8688);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6304);
					match s {
						[] => true,
						['a', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8690);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['i', 'm', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10159);
									s.is_empty()
								}
								['t', 'a', 'm', 'u', 'a', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 10160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t4::n3438(s, out),
		['i', s @ ..] => super::t4::n3439(s, out),
		['k', s @ ..] => {
			super::push(out, 3211);
			s.is_empty()
		}
		['l', s @ ..] => super::t4::n3441(s, out),
		['m', s @ ..] => super::t4::n3442(s, out),
		['n', s @ ..] => super::t4::n3443(s, out),
		['o', s @ ..] => {
			super::push(out, 3215);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6329);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8754);
							s.is_empty()
						}
						['g', 'i', 'l', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6330);
					s.is_empty()
				}
				['v', s @ ..] => super::t4::n6848(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t4::n3445(s, out),
		['r', s @ ..] => super::t4::n3446(s, out),
		['s', s @ ..] => super::t4::n3447(s, out),
		['t', s @ ..] => super::t4::n3448(s, out),
		['u', 'q', 'i', 'l', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3220);
			s.is_empty()
		}
		['x', s @ ..] => super::t4::n3450(s, out),
		_ => false,
	}
}

/// Words ending -spacecraft (3 syllables).
pub(crate) fn n12401(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11360);
			s.is_empty()
		}
		['o', 'n', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 11361);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word saucercraft (3 syllables).
pub(crate) fn n12402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11237);
	s.is_empty()
}

/// Words ending -vercraft (3 syllables).
pub(crate) fn n12403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11238);
	match s {
		[] => true,
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 11362);
			s.is_empty()
		}
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 11363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trix (3 syllables).
pub(crate) fn n7095(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8911);
			match s {
				[] => true,
				['c', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10268);
					s.is_empty()
				}
				['g', 'i', 'v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 10269);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10911);
							s.is_empty()
						}
						['l', 'i', 'm', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10913);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10914);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 11242);
									s.is_empty()
								}
								['r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 11243);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10915);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11364);
											s.is_empty()
										}
										['y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 11365);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 11244);
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
						['a', 'p', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 10916);
							s.is_empty()
						}
						['e', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10917);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11245);
									s.is_empty()
								}
								['b', 'a', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11247);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11248);
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
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 11249);
									s.is_empty()
								}
								['e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 11250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10918);
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
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10919);
							s.is_empty()
						}
						['s', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 't', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 10270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10271);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 10272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8912);
			match s {
				[] => true,
				['c', 'i', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 10273);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 10921);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10922);
							s.is_empty()
						}
						['i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 10923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'h', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 10274);
					s.is_empty()
				}
				['t', 'e', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10275);
					s.is_empty()
				}
				['v', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 10276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'r', 'e', 'f', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 8913);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8914);
			match s {
				[] => true,
				['b', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 10277);
					s.is_empty()
				}
				['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 10279);
					match s {
						[] => true,
						['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10924);
							s.is_empty()
						}
						['x', 'e', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'd', 'a', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word proctostomy (3 syllables).
pub(crate) fn n12417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11251);
	s.is_empty()
}

/// Words ending -iaxiality (3 syllables).
pub(crate) fn n12418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11252);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 11366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tability (3 syllables).
pub(crate) fn n12419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11253);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 11367);
			match s {
				[] => true,
				['p', 'e', s @ ..] => {
					super::push(out, 11390);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 11403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word decennalia (3 syllables).
pub(crate) fn n12541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11368);
	s.is_empty()
}

/// The word quindecennalia (3 syllables).
pub(crate) fn n12542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11369);
	s.is_empty()
}

/// The word tricennalia (3 syllables).
pub(crate) fn n12543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11370);
	s.is_empty()
}

/// The word vicennalia (3 syllables).
pub(crate) fn n12544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11371);
	s.is_empty()
}

/// The word cyamella (3 syllables).
pub(crate) fn n9871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9024);
	s.is_empty()
}

/// Words ending -nella (3 syllables).
pub(crate) fn n7379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6821);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 10336);
					s.is_empty()
				}
				['r', 'b', 'm', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 9025);
			s.is_empty()
		}
		['m', 'u', 'l', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9026);
			s.is_empty()
		}
		['o', 'l', 'l', 'i', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 9027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pella (3 syllables).
pub(crate) fn n7380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6822);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9028);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 9029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rella (3 syllables).
pub(crate) fn n7381(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9030);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 10339);
					match s {
						[] => true,
						['n', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9031);
			s.is_empty()
		}
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9032);
			s.is_empty()
		}
		['o', 'l', s @ ..] => {
			super::push(out, 9033);
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10946);
							s.is_empty()
						}
						['o', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 10947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 9034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sella (3 syllables).
pub(crate) fn n7382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6823);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9035);
			match s {
				[] => true,
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 10340);
					s.is_empty()
				}
				['u', 'v', '\u{1}', s @ ..] => {
					super::push(out, 10341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 9036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tella (3 syllables).
pub(crate) fn n7383(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9037);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10342);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10948);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 11257);
									match s {
										[] => true,
										['a', 't', '\u{1}', s @ ..] => {
											super::push(out, 11372);
											s.is_empty()
										}
										['o', 'f', 's', '\u{1}', s @ ..] => {
											super::push(out, 11373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 10343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 10344);
					s.is_empty()
				}
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 9038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zella (3 syllables).
pub(crate) fn n7384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6824);
	match s {
		[] => true,
		['z', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -illa (3 syllables).
pub(crate) fn n4153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3875);
	match s {
		[] => true,
		['b', 'r', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6825);
			s.is_empty()
		}
		['c', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6826);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6827);
			s.is_empty()
		}
		['f', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 6828);
			s.is_empty()
		}
		['h', 't', 'a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 6829);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9040);
					s.is_empty()
				}
				['m', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6830);
			s.is_empty()
		}
		['p', 'a', s @ ..] => {
			super::push(out, 6831);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10346);
							s.is_empty()
						}
						['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6832);
			match s {
				[] => true,
				['b', 'i', s @ ..] => {
					super::push(out, 9043);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 10348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10949);
									s.is_empty()
								}
								['o', 'r', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 10950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'n', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 6833);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9044);
					s.is_empty()
				}
				['n', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9045);
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6834);
			s.is_empty()
		}
		['v', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 6835);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 6836);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10349);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10951);
									s.is_empty()
								}
								['o', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 10952);
									s.is_empty()
								}
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
			super::push(out, 6837);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word abolla (3 syllables).
pub(crate) fn n4154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3876);
	s.is_empty()
}

/// The word papulla (3 syllables).
pub(crate) fn n4155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3877);
	s.is_empty()
}

/// Words ending -ola (3 syllables).
pub(crate) fn n1469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1349);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3878);
			match s {
				[] => true,
				['i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3879);
			match s {
				[] => true,
				['h', 'c', 'a', 'h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6839);
					s.is_empty()
				}
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6840);
					s.is_empty()
				}
				['t', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6841);
					s.is_empty()
				}
				['v', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3880);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6843);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3881);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6844);
					match s {
						[] => true,
						['a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9049);
							s.is_empty()
						}
						['s', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 6845);
					s.is_empty()
				}
				['r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		['p', 'p', 'e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 3883);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3884);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3885);
			match s {
				[] => true,
				['t', 'o', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sla (3 syllables).
pub(crate) fn n1470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1350);
	s.is_empty()
}

/// Words ending -ula (3 syllables).
pub(crate) fn n1471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9051);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9053);
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9055);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6848);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10350);
									s.is_empty()
								}
								['i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9058);
							s.is_empty()
						}
						['u', 'g', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9060);
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9061);
							s.is_empty()
						}
						['d', 'u', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9062);
							s.is_empty()
						}
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9063);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9064);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10352);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									match s {
										[] => true,
										['c', 'i', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10953);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 10954);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9065);
							s.is_empty()
						}
						['u', 'g', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9066);
							s.is_empty()
						}
						['v', 'a', 'n', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6850);
					s.is_empty()
				}
				['n', 'a', 'l', 'g', 'o', 'h', 'p', 'm', 'y', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6852);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6853);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9068);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 6854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3888);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3889);
			match s {
				[] => true,
				['i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6855);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6856);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9070);
							s.is_empty()
						}
						['i', 'c', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9071);
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
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6857);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9072);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9073);
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
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9074);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 9075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6858);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9077);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9078);
							s.is_empty()
						}
						['r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9079);
							s.is_empty()
						}
						['t', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6859);
					s.is_empty()
				}
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6860);
					s.is_empty()
				}
				['r', 'e', 'v', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6861);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9081);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3890);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10353);
									s.is_empty()
								}
								['o', 's', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'o', 'v', 'l', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6863);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9084);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 6864);
					s.is_empty()
				}
				['y', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3891);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6866);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6867);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9086);
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', 's', 'o', s @ ..] => {
					super::push(out, 6869);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6870);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6871);
					s.is_empty()
				}
				['n', 'a', 'l', 'p', s @ ..] => super::t4::n7447(s, out),
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6872);
					s.is_empty()
				}
				['r', s @ ..] => super::t4::n7449(s, out),
				['s', s @ ..] => super::t4::n7450(s, out),
				['t', 'u', 'g', '\u{1}', s @ ..] => super::t4::n7451(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => super::t4::n4178(s, out),
		_ => false,
	}
}

/// Words ending -yla (3 syllables).
pub(crate) fn n1472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	match s {
		[] => true,
		['h', 'p', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 3892);
			s.is_empty()
		}
		['r', 'e', 't', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3893);
			s.is_empty()
		}
		['t', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ima (3 syllables).
pub(crate) fn n1489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3941);
			match s {
				[] => true,
				['n', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'a', 'y', '\u{1}', s @ ..] => {
			super::push(out, 3942);
			s.is_empty()
		}
		['r', 'a', 'v', 'a', 't', 't', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3943);
			s.is_empty()
		}
		['s', 'e', s @ ..] => {
			super::push(out, 3944);
			match s {
				[] => true,
				['g', 'a', 'u', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3945);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mma (3 syllables).
pub(crate) fn n1491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3949);
			match s {
				[] => true,
				['m', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6909);
					s.is_empty()
				}
				['r', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', s @ ..] => {
			super::push(out, 3950);
			match s {
				[] => true,
				['o', 'y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oma (3 syllables).
pub(crate) fn n1492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['c', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3951);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3952);
			s.is_empty()
		}
		['h', 'p', 'm', 'y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3953);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6912);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10360);
									s.is_empty()
								}
								['s', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6913);
					s.is_empty()
				}
				['x', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3954);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6915);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', 'r', 'g', 'o', 'r', 'c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9117);
							s.is_empty()
						}
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3955);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6917);
					s.is_empty()
				}
				['d', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6918);
					s.is_empty()
				}
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6919);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6920);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9119);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 9120);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6921);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6922);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9122);
							s.is_empty()
						}
						['h', 't', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'g', '\u{1}', s @ ..] => {
									super::push(out, 10363);
									s.is_empty()
								}
								['n', 'a', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10364);
									s.is_empty()
								}
								['s', 'i', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'y', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9123);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9124);
							s.is_empty()
						}
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3958);
			match s {
				[] => true,
				['a', 'm', 'e', 'h', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6923);
					s.is_empty()
				}
				['e', 'c', 'y', 'm', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6924);
					s.is_empty()
				}
				['p', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6925);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9126);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9127);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10366);
									s.is_empty()
								}
								['i', 'h', 'c', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10367);
									s.is_empty()
								}
								['p', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6926);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rma (3 syllables).
pub(crate) fn n1493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	match s {
		[] => true,
		['e', 'd', 'o', s @ ..] => {
			match s {
				[] => true,
				['c', 'y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6927);
					s.is_empty()
				}
				['p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 's', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sma (3 syllables).
pub(crate) fn n1494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6929);
					match s {
						[] => true,
						['p', 'o', 'x', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 9128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6930);
					s.is_empty()
				}
				['t', 'n', 'a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'e', 't', 'o', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3960);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['p', 'e', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9129);
							s.is_empty()
						}
						['t', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 6932);
					s.is_empty()
				}
				['m', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tma (3 syllables).
pub(crate) fn n1495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1374);
	s.is_empty()
}

/// Words ending -uma (3 syllables).
pub(crate) fn n1496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1375);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 3962);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 6934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'p', 'o', 'l', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yma (3 syllables).
pub(crate) fn n1497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3964);
			s.is_empty()
		}
		['h', 'c', 'n', 'e', s @ ..] => {
			match s {
				[] => true,
				['n', 'e', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6935);
					s.is_empty()
				}
				['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6936);
					s.is_empty()
				}
				['t', 'c', 'e', 'l', 'p', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -search (2 syllables).
pub(crate) fn n10213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9347);
	match s {
		[] => true,
		['a', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 10425);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 10426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iarch (2 syllables).
pub(crate) fn n8015(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9348);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9349);
			s.is_empty()
		}
		['s', 'u', '\u{1}', s @ ..] => {
			super::push(out, 9350);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 9351);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wheelarch (2 syllables).
pub(crate) fn n8016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7395);
	s.is_empty()
}

/// Words ending -march (2 syllables).
pub(crate) fn n8017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7396);
	match s {
		[] => true,
		['d', 'o', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 9352);
			s.is_empty()
		}
		['g', 'o', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 9353);
			s.is_empty()
		}
		['h', 't', 'a', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 9354);
			s.is_empty()
		}
		['s', 'k', 'l', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 9355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -narch (2 syllables).
pub(crate) fn n8018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7397);
	s.is_empty()
}

/// Words ending -oarch (2 syllables).
pub(crate) fn n8019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7398);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9356);
			match s {
				[] => true,
				['a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10428);
					s.is_empty()
				}
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -parch (2 syllables).
pub(crate) fn n8020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7399);
	s.is_empty()
}

/// Words ending -rarch (2 syllables).
pub(crate) fn n8021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7400);
	s.is_empty()
}

/// The word thiasarch (2 syllables).
pub(crate) fn n8022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7401);
	s.is_empty()
}

/// Words ending -nstarch (2 syllables).
pub(crate) fn n8023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7402);
	match s {
		[] => true,
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 9357);
			s.is_empty()
		}
		['r', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erch (2 syllables).
pub(crate) fn n5019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4684);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 7403);
			match s {
				[] => true,
				['t', 'u', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orch (2 syllables).
pub(crate) fn n5020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4685);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 7404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urch (2 syllables).
pub(crate) fn n5021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4686);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sch (2 syllables).
pub(crate) fn n2077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', 'k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7406);
					s.is_empty()
				}
				['t', 'm', 'm', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 7407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'm', 't', 'f', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tch (2 syllables).
pub(crate) fn n2078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4688);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7408);
					match s {
						[] => true,
						['a', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9360);
							s.is_empty()
						}
						['e', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uch (2 syllables).
pub(crate) fn n2079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4690);
			s.is_empty()
		}
		['h', 'c', 'k', 'e', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4691);
			s.is_empty()
		}
		['l', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4692);
			s.is_empty()
		}
		['n', 'u', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4693);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4694);
			match s {
				[] => true,
				['g', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7409);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4695);
			s.is_empty()
		}
		['w', 'f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ych (2 syllables).
pub(crate) fn n2080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 4697);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rch (3 syllables).
pub(crate) fn n2087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1939);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4719);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7434);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7435);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9374);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10439);
									match s {
										[] => true,
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 11263);
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 11264);
													s.is_empty()
												}
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
							super::push(out, 9375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'u', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9376);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 'n', 'u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7436);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 7437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9378);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 10440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9379);
							s.is_empty()
						}
						['o', 'r', 't', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'p', 'e', 'k', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4720);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4721);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9381);
							s.is_empty()
						}
						['o', 'b', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4722);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7438);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uch (3 syllables).
pub(crate) fn n2090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1942);
	match s {
		[] => true,
		['e', 't', 'a', 't', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4727);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tych (3 syllables).
pub(crate) fn n5066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4729);
	s.is_empty()
}

/// The word dhufish (2 syllables).
pub(crate) fn n8090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7454);
	s.is_empty()
}

/// The word blowfish (2 syllables).
pub(crate) fn n8091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7455);
	s.is_empty()
}

/// The word foxfish (2 syllables).
pub(crate) fn n8092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7456);
	s.is_empty()
}

/// Words ending -yfish (2 syllables).
pub(crate) fn n8093(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9414);
			s.is_empty()
		}
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 9415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hish (2 syllables).
pub(crate) fn n5087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4750);
	s.is_empty()
}

/// Words ending -lish (2 syllables).
pub(crate) fn n5088(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 7457);
			s.is_empty()
		}
		['i', '\u{1}', s @ ..] => {
			super::push(out, 7458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mish (2 syllables).
pub(crate) fn n5089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4751);
	s.is_empty()
}

/// Words ending -nish (2 syllables).
pub(crate) fn n5090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4752);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9416);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 9417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7459);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word irish (2 syllables).
pub(crate) fn n5091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4753);
	s.is_empty()
}

/// Words ending -tish (2 syllables).
pub(crate) fn n5092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4754);
	s.is_empty()
}

/// The word skitswish (2 syllables).
pub(crate) fn n5093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4755);
	s.is_empty()
}

/// Words ending -lefish (3 syllables).
pub(crate) fn n10304(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10989);
					s.is_empty()
				}
				['e', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 10990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 't', '\u{1}', s @ ..] => {
			super::push(out, 10464);
			s.is_empty()
		}
		['p', 'p', 'a', 'e', 'n', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10465);
			s.is_empty()
		}
		['t', 't', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 10991);
					s.is_empty()
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 10992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nefish (3 syllables).
pub(crate) fn n10306(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'p', 'u', 'c', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10466);
			s.is_empty()
		}
		['o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 10467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -refish (3 syllables).
pub(crate) fn n10307(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 10468);
			s.is_empty()
		}
		['i', 'u', 'q', 's', '\u{1}', s @ ..] => {
			super::push(out, 10469);
			s.is_empty()
		}
		['o', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 10470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tefish (3 syllables).
pub(crate) fn n10308(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 10471);
			s.is_empty()
		}
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 10472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfish (3 syllables).
pub(crate) fn n8102(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10473);
					s.is_empty()
				}
				['v', 'o', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 10474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 'n', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nfish (3 syllables).
pub(crate) fn n8103(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'k', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 9425);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'e', 'v', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 10475);
					s.is_empty()
				}
				['r', 'a', 'd', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10993);
							s.is_empty()
						}
						['p', 'r', 'o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 10994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 10477);
					s.is_empty()
				}
				['t', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 'n', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ofish (3 syllables).
pub(crate) fn n8104(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 9427);
			s.is_empty()
		}
		['e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfish (3 syllables).
pub(crate) fn n8105(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10479);
					s.is_empty()
				}
				['t', 'i', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 10480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'm', 'u', 'c', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10481);
					s.is_empty()
				}
				['d', 'n', 'u', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 10482);
					s.is_empty()
				}
				['f', 'f', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10483);
					s.is_empty()
				}
				['g', 'g', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 10484);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', 'b', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10995);
							s.is_empty()
						}
						['g', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 't', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 10485);
					s.is_empty()
				}
				['v', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 10486);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sfish (3 syllables).
pub(crate) fn n8106(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', 'a', 'k', 'c', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 9429);
			s.is_empty()
		}
		['u', 'g', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 9430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tfish (3 syllables).
pub(crate) fn n8107(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 10487);
					s.is_empty()
				}
				['p', 'm', 'u', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 10488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'g', 'i', 'l', 'd', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 9431);
			s.is_empty()
		}
		['i', 'b', 'b', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 9432);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 10489);
					s.is_empty()
				}
				['e', 'r', 'r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 10490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word swallowfish (3 syllables).
pub(crate) fn n8108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7463);
	s.is_empty()
}

/// Words ending -yfish (3 syllables).
pub(crate) fn n8109(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'k', 'r', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 9433);
			s.is_empty()
		}
		['l', 'l', 'e', 'j', '\u{1}', s @ ..] => {
			super::push(out, 9434);
			s.is_empty()
		}
		['r', 'r', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omish (3 syllables).
pub(crate) fn n5102(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'o', 'n', 's', '\u{1}', s @ ..] => {
			super::push(out, 7464);
			s.is_empty()
		}
		['n', 'i', 'w', 's', '\u{1}', s @ ..] => {
			super::push(out, 7465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nish (3 syllables).
pub(crate) fn n5103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4763);
	s.is_empty()
}

/// Words ending -rish (3 syllables).
pub(crate) fn n5104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4764);
	s.is_empty()
}

/// Words ending -osh (3 syllables).
pub(crate) fn n2128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1979);
	s.is_empty()
}

/// Words ending -ush (3 syllables).
pub(crate) fn n2129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1980);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4765);
			s.is_empty()
		}
		['c', 'y', 'a', 'm', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odaeum (3 syllables).
pub(crate) fn n8401(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'n', 'y', 'h', 'r', '\u{1}', s @ ..] => {
			super::push(out, 9536);
			s.is_empty()
		}
		['t', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9537);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word athenaeum (3 syllables).
pub(crate) fn n8403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7724);
	s.is_empty()
}

/// The word gynaeceum (3 syllables).
pub(crate) fn n5573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5178);
	s.is_empty()
}

/// The word proctodeum (3 syllables).
pub(crate) fn n5574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5179);
	s.is_empty()
}

/// The word hypogeum (3 syllables).
pub(crate) fn n5575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5180);
	s.is_empty()
}

/// Words ending -leum (3 syllables).
pub(crate) fn n5576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5181);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9538);
					s.is_empty()
				}
				['r', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9539);
					s.is_empty()
				}
				['s', 'u', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9540);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -neum (3 syllables).
pub(crate) fn n5577(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'l', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10520);
							s.is_empty()
						}
						['o', 'l', 'a', 'g', 'a', 'r', 't', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', 'c', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'h', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7725);
			s.is_empty()
		}
		['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7726);
			s.is_empty()
		}
		['o', 't', s @ ..] => {
			super::push(out, 7727);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10522);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 11005);
									s.is_empty()
								}
								['m', 'u', 'e', 'n', 'p', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11006);
									s.is_empty()
								}
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

/// Words ending -peum (3 syllables).
pub(crate) fn n5578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5182);
	s.is_empty()
}

/// Words ending -reum (3 syllables).
pub(crate) fn n5579(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'z', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7728);
			s.is_empty()
		}
		['o', 't', 's', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -seum (3 syllables).
pub(crate) fn n5580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5183);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 7730);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word coenosteum (3 syllables).
pub(crate) fn n5581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5184);
	s.is_empty()
}

/// Words ending -gum (3 syllables).
pub(crate) fn n2578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2381);
	match s {
		[] => true,
		['n', 'o', 'l', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5185);
			s.is_empty()
		}
		['r', 'e', 't', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hum (3 syllables).
pub(crate) fn n2579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2382);
	match s {
		[] => true,
		['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5187);
			s.is_empty()
		}
		['r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 5188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ium (3 syllables).
pub(crate) fn n2580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2383);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9543);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7731);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 7732);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9545);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 10523);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10524);
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
						['l', s @ ..] => {
							super::push(out, 9546);
							match s {
								[] => true,
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10525);
									s.is_empty()
								}
								['y', 'x', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10527);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'd', 'n', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9547);
							s.is_empty()
						}
						['t', 'a', 'm', 'e', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9548);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9549);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 't', 't', 'y', '\u{1}', s @ ..] => {
					super::push(out, 7733);
					s.is_empty()
				}
				['u', 'r', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5189);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7735);
					match s {
						[] => true,
						['m', 'i', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7736);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9551);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10530);
									s.is_empty()
								}
								['n', 'y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 10531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 'e', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7738);
					match s {
						[] => true,
						['k', 'c', 'a', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 9552);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10532);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 10533);
									s.is_empty()
								}
								['l', 'a', 'p', 'l', s @ ..] => {
									super::push(out, 10534);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 11007);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9553);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10535);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 11008);
											s.is_empty()
										}
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
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10536);
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 10537);
									s.is_empty()
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 10538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'i', 'r', 'p', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7739);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9555);
							match s {
								[] => true,
								['m', 'y', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10539);
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 10540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => super::t4::n10463(s, out),
						['m', 'a', '\u{1}', s @ ..] => super::t4::n10464(s, out),
						['n', s @ ..] => super::t4::n10465(s, out),
						['p', s @ ..] => super::t4::n10466(s, out),
						['r', s @ ..] => super::t4::n10467(s, out),
						['s', 'a', s @ ..] => super::t4::n10468(s, out),
						['t', s @ ..] => super::t4::n10469(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t4::n8424(s, out),
				['o', s @ ..] => super::t4::n8425(s, out),
				['r', s @ ..] => super::t4::n8426(s, out),
				['u', 'l', 'e', 'r', 'p', '\u{1}', s @ ..] => super::t4::n8427(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t4::n5589(s, out),
		['h', s @ ..] => super::t4::n5590(s, out),
		['l', s @ ..] => super::t4::n5591(s, out),
		['m', s @ ..] => super::t4::n5592(s, out),
		['n', s @ ..] => super::t4::n5593(s, out),
		['p', s @ ..] => super::t4::n5594(s, out),
		['r', s @ ..] => super::t4::n5595(s, out),
		['s', s @ ..] => super::t4::n5596(s, out),
		['t', s @ ..] => super::t4::n5597(s, out),
		['u', 'q', s @ ..] => super::t4::n5598(s, out),
		['v', s @ ..] => super::t4::n5599(s, out),
		['z', s @ ..] => super::t4::n5600(s, out),
		_ => false,
	}
}

/// The word leucojum (3 syllables).
pub(crate) fn n2581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2384);
	s.is_empty()
}

/// The word husbullhookum (3 syllables).
pub(crate) fn n2582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2385);
	s.is_empty()
}

/// Words ending -lum (3 syllables).
pub(crate) fn n2583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2386);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5199);
			match s {
				[] => true,
				['b', 'y', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 7781);
					s.is_empty()
				}
				['d', 'e', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7782);
					s.is_empty()
				}
				['h', 'p', 'e', 'c', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9657);
							s.is_empty()
						}
						['o', 'p', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9659);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9660);
							s.is_empty()
						}
						['o', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'u', 'm', 'y', 'r', 'r', 'e', 'j', '\u{1}', s @ ..] => {
			super::push(out, 5200);
			s.is_empty()
		}
		['i', 'd', 'e', 'p', 'o', 'i', 'h', 'p', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5201);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9662);
							s.is_empty()
						}
						['v', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7783);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10652);
									match s {
										[] => true,
										['l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 11102);
											s.is_empty()
										}
										['r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11103);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'r', 'e', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11104);
											s.is_empty()
										}
										['o', 'l', 'u', 'b', 'i', 't', 's', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 11105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'l', s @ ..] => {
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 10653);
									s.is_empty()
								}
								['u', 'v', '\u{1}', s @ ..] => {
									super::push(out, 10654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7784);
					match s {
						[] => true,
						['r', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9664);
							s.is_empty()
						}
						['v', 'o', 'e', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 7785);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9666);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11106);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11303);
													s.is_empty()
												}
												['s', 'e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 11304);
													s.is_empty()
												}
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
		['o', 'i', 'l', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5202);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5203);
			match s {
				[] => true,
				['a', 't', 'c', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7786);
					s.is_empty()
				}
				['e', 'v', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7787);
					s.is_empty()
				}
				['n', 'o', 'e', 'g', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7788);
					s.is_empty()
				}
				['o', 'c', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7789);
					s.is_empty()
				}
				['r', 'a', 'g', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9667);
							s.is_empty()
						}
						['i', 't', 'a', s @ ..] => {
							super::push(out, 9668);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 10655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7791);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 10656);
									s.is_empty()
								}
								['t', 'c', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 10657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10658);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10660);
									s.is_empty()
								}
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10661);
									s.is_empty()
								}
								['t', 'r', 'e', 'v', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9669);
							s.is_empty()
						}
						['r', 'e', 'p', 'o', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9670);
							s.is_empty()
						}
						['s', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9671);
							s.is_empty()
						}
						['u', 'b', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7792);
					match s {
						[] => true,
						['n', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10663);
									s.is_empty()
								}
								['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7793);
					match s {
						[] => true,
						['a', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9673);
							s.is_empty()
						}
						['e', 't', 'u', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9674);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9675);
							match s {
								[] => true,
								['i', 'c', 'd', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7795);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7796);
					s.is_empty()
				}
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7797);
					s.is_empty()
				}
				['x', 'o', 'h', 't', 'n', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7798);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mum (3 syllables).
pub(crate) fn n2584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2387);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7799);
					s.is_empty()
				}
				['r', 't', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7801);
					s.is_empty()
				}
				['l', 'l', 'i', 'm', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 7802);
					s.is_empty()
				}
				['n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7803);
					s.is_empty()
				}
				['s', 's', 'i', 't', 'a', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7804);
					s.is_empty()
				}
				['t', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7805);
					s.is_empty()
				}
				['x', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9676);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9677);
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
				['d', 'o', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7806);
					s.is_empty()
				}
				['t', 's', 'o', 'n', 'i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5205);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -num (3 syllables).
pub(crate) fn n2585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2388);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5206);
			match s {
				[] => true,
				['c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7809);
					s.is_empty()
				}
				['d', 'e', 'c', 'u', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7810);
					s.is_empty()
				}
				['g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 7811);
					s.is_empty()
				}
				['p', 'm', 'y', 't', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 9678);
							s.is_empty()
						}
						['p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9679);
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9680);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 10666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'u', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10667);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 11107);
											s.is_empty()
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'a', 'g', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'e', 'r', s @ ..] => {
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7813);
					s.is_empty()
				}
				['i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 7814);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9681);
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9682);
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
				['c', 's', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7815);
					s.is_empty()
				}
				['h', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7816);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 9683);
							s.is_empty()
						}
						['b', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 9684);
							s.is_empty()
						}
						['i', 't', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7817);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 9686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 7818);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9687);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9688);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10668);
									s.is_empty()
								}
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
			super::push(out, 5207);
			s.is_empty()
		}
		['o', 'g', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5208);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5209);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9689);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9690);
							match s {
								[] => true,
								['a', 'c', 'r', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10669);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['p', 'e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11305);
													s.is_empty()
												}
												['e', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11306);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'a', 'b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11307);
													s.is_empty()
												}
												['o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11308);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'x', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7819);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9691);
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
				['e', 'j', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 7820);
					s.is_empty()
				}
				['j', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 7821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'g', 'i', 'p', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5210);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wanapum (3 syllables).
pub(crate) fn n2586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2389);
	s.is_empty()
}

/// Words ending -rum (3 syllables).
pub(crate) fn n2587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2390);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5211);
			match s {
				[] => true,
				['b', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7822);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5212);
			match s {
				[] => true,
				['e', 'r', 'e', 'c', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', s @ ..] => {
			match s {
				[] => true,
				['l', 'u', 'b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7825);
					s.is_empty()
				}
				['s', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 7826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5213);
			match s {
				[] => true,
				['e', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 9692);
							s.is_empty()
						}
						['s', 'o', 'c', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9693);
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
				['g', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 7827);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7828);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9695);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9696);
							s.is_empty()
						}
						['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 7829);
					s.is_empty()
				}
				['v', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 't', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7831);
					s.is_empty()
				}
				['e', 'r', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7832);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7833);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5214);
			match s {
				[] => true,
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7834);
					s.is_empty()
				}
				['h', 'p', 'o', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7835);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'f', 'i', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 5215);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5216);
			match s {
				[] => true,
				['a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7837);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10671);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10672);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 11109);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'l', 'a', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 9699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9700);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9702);
							match s {
								[] => true,
								['e', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10673);
									s.is_empty()
								}
								['l', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'u', 'q', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9703);
							s.is_empty()
						}
						['i', 'n', 'a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9704);
							s.is_empty()
						}
						['u', 'g', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 'l', 'p', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sum (3 syllables).
pub(crate) fn n2588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2391);
	match s {
		[] => true,
		['a', 'm', 'o', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5218);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5219);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', 's', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7838);
					s.is_empty()
				}
				['i', 'm', 'm', 'o', 'c', 'i', 'e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7839);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tum (3 syllables).
pub(crate) fn n2589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2392);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5220);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7841);
					s.is_empty()
				}
				['l', 'o', 'r', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7842);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7843);
					match s {
						[] => true,
						['i', 't', 'l', 'u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10675);
									s.is_empty()
								}
								['n', 'e', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11110);
											s.is_empty()
										}
										['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9707);
							s.is_empty()
						}
						['t', 's', 'r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 9708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', 'p', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9709);
							s.is_empty()
						}
						['s', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 9710);
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
				['e', s @ ..] => {
					super::push(out, 7844);
					match s {
						[] => true,
						['j', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9711);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9712);
							match s {
								[] => true,
								['o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'd', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9713);
							s.is_empty()
						}
						['i', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9714);
							s.is_empty()
						}
						['r', 'e', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 9715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 7846);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9716);
							s.is_empty()
						}
						['o', 'b', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9717);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9718);
							match s {
								[] => true,
								['f', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10677);
									s.is_empty()
								}
								['h', 'p', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7847);
					match s {
						[] => true,
						['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9719);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 10679);
									s.is_empty()
								}
								['u', 'q', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10680);
									s.is_empty()
								}
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
				['c', 's', 'i', 'b', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7848);
					s.is_empty()
				}
				['n', 'o', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7849);
					s.is_empty()
				}
				['p', 'l', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7850);
					s.is_empty()
				}
				['s', 'o', s @ ..] => {
					super::push(out, 7851);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9720);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10681);
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10682);
									s.is_empty()
								}
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
			super::push(out, 5221);
			match s {
				[] => true,
				['a', 'h', 'p', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7852);
					s.is_empty()
				}
				['u', 'f', 'm', 'r', 'o', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7854);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9721);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9722);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10683);
									s.is_empty()
								}
								['g', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 10684);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11112);
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											super::push(out, 11113);
											s.is_empty()
										}
										['m', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11309);
													s.is_empty()
												}
												['i', 's', 'a', 'u', 'q', '\u{1}', s @ ..] => {
													super::push(out, 11310);
													s.is_empty()
												}
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
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7855);
					s.is_empty()
				}
				['i', 'b', 'm', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 7856);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9723);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', 's', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7857);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7858);
					match s {
						[] => true,
						['c', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9725);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'e', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7859);
					s.is_empty()
				}
				['s', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9727);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 10686);
									s.is_empty()
								}
								['p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10687);
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'g', 'g', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 5222);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5223);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5224);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7861);
					s.is_empty()
				}
				['h', 'p', 'o', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7862);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uum (3 syllables).
pub(crate) fn n2590(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 'a', 'v', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 7863);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', s @ ..] => {
			match s {
				[] => true,
				['s', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7865);
					s.is_empty()
				}
				['v', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 't', 'n', 'o', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7867);
					s.is_empty()
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7868);
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7869);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vum (3 syllables).
pub(crate) fn n2591(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'v', 'r', 'e', 'p', 'm', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 5225);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5226);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word raycizzum (3 syllables).
pub(crate) fn n2592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2393);
	s.is_empty()
}

/// The word kundiman (3 syllables).
pub(crate) fn n8689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7950);
	s.is_empty()
}

/// The word hiroshiman (3 syllables).
pub(crate) fn n8690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7951);
	s.is_empty()
}

/// The word yakiman (3 syllables).
pub(crate) fn n8691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7952);
	s.is_empty()
}

/// The word kooliman (3 syllables).
pub(crate) fn n8692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7953);
	s.is_empty()
}

/// Words ending -eriman (3 syllables).
pub(crate) fn n8693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7954);
	s.is_empty()
}

/// The word misiman (3 syllables).
pub(crate) fn n8694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7955);
	s.is_empty()
}

/// Words ending -lman (3 syllables).
pub(crate) fn n5730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5299);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 9785);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['s', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10689);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7956);
			s.is_empty()
		}
		['u', 's', s @ ..] => {
			match s {
				[] => true,
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9786);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word landamman (3 syllables).
pub(crate) fn n5731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5300);
	s.is_empty()
}

/// Words ending -oman (3 syllables).
pub(crate) fn n5732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5301);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7957);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10691);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 10692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9789);
					match s {
						[] => true,
						['v', 's', '\u{1}', s @ ..] => {
							super::push(out, 10693);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 10694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 9790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9791);
					s.is_empty()
				}
				['o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7958);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9793);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 10695);
							s.is_empty()
						}
						['l', 'k', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7959);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['v', 's', '\u{1}', s @ ..] => {
							super::push(out, 10697);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 10698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9794);
					s.is_empty()
				}
				['r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 9795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9796);
					s.is_empty()
				}
				['l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'n', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 7960);
			s.is_empty()
		}
		['r', 's', 'g', 'n', 'u', 'd', 'l', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 7961);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7962);
			match s {
				[] => true,
				['n', 'e', 'g', 'y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 9798);
					s.is_empty()
				}
				['r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 9799);
					s.is_empty()
				}
				['t', 'o', '\u{1}', s @ ..] => {
					super::push(out, 9800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7963);
			match s {
				[] => true,
				['s', 'i', s @ ..] => {
					super::push(out, 9801);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 9802);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10699);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 11114);
									match s {
										[] => true,
										['n', 'r', 'e', 'u', 'g', '\u{1}', s @ ..] => {
											super::push(out, 11311);
											s.is_empty()
										}
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

/// Words ending -rman (3 syllables).
pub(crate) fn n5733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5302);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7964);
			match s {
				[] => true,
				['h', 'r', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9804);
					s.is_empty()
				}
				['g', 'g', 'u', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sman (3 syllables).
pub(crate) fn n5734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5303);
	match s {
		[] => true,
		['e', 'j', 's', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 7965);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7966);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9806);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 10700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tman (3 syllables).
pub(crate) fn n5735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5304);
	s.is_empty()
}

/// Words ending -uman (3 syllables).
pub(crate) fn n5736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5305);
	match s {
		[] => true,
		['e', 'i', 'l', 'r', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7967);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7968);
			s.is_empty()
		}
		['j', 'u', 'j', '\u{1}', s @ ..] => {
			super::push(out, 7969);
			s.is_empty()
		}
		['k', 'i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 7970);
			s.is_empty()
		}
		['z', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yman (3 syllables).
pub(crate) fn n5737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5306);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['k', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9807);
					s.is_empty()
				}
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nan (3 syllables).
pub(crate) fn n2637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2438);
	s.is_empty()
}

/// Words ending -oan (3 syllables).
pub(crate) fn n2638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2439);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5307);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5308);
			match s {
				[] => true,
				['a', 't', 'e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9809);
							s.is_empty()
						}
						['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 't', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9811);
							s.is_empty()
						}
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9812);
							s.is_empty()
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
								['e', 'o', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10701);
									s.is_empty()
								}
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9813);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['p', 'y', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 10703);
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'e', 'h', 'o', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9814);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9815);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 'n', 'u', 'y', '\u{1}', s @ ..] => {
											super::push(out, 11115);
											s.is_empty()
										}
										['o', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 11116);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11117);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 11312);
													s.is_empty()
												}
												['i', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 11313);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['d', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10705);
									s.is_empty()
								}
								['e', 't', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10706);
									s.is_empty()
								}
								['h', 'p', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 10707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['u', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10708);
									s.is_empty()
								}
								['y', 'd', 'c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'e', s @ ..] => {
									match s {
										[] => true,
										['a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 11118);
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											super::push(out, 11119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10710);
									s.is_empty()
								}
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10711);
									s.is_empty()
								}
								['o', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11120);
											s.is_empty()
										}
										['o', 'm', s @ ..] => {
											super::push(out, 11121);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 11314);
													match s {
														[] => true,
														['a', 'h', '\u{1}', s @ ..] => super::t4::n12551(s, out),
														['h', '\u{1}', s @ ..] => super::t4::n12552(s, out),
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
						['x', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pan (3 syllables).
pub(crate) fn n2639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2440);
	s.is_empty()
}

/// Words ending -ran (3 syllables).
pub(crate) fn n2640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2441);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5309);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5310);
			match s {
				[] => true,
				['k', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -san (3 syllables).
pub(crate) fn n2641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2442);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tan (3 syllables).
pub(crate) fn n2642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2443);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 5312);
			match s {
				[] => true,
				['h', 'c', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7973);
					s.is_empty()
				}
				['i', 'p', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uan (3 syllables).
pub(crate) fn n2643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2444);
	s.is_empty()
}

/// Words ending -van (3 syllables).
pub(crate) fn n2644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2445);
	s.is_empty()
}

/// Words ending -wan (3 syllables).
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2446);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			super::push(out, 5313);
			match s {
				[] => true,
				['c', 't', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xan (3 syllables).
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2447);
	s.is_empty()
}

/// Words ending -yan (3 syllables).
pub(crate) fn n2647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2448);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 5314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zan (3 syllables).
pub(crate) fn n2648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2449);
	s.is_empty()
}

/// Words ending -ion (3 syllables).
pub(crate) fn n2748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2546);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5455);
			match s {
				[] => true,
				['p', 'o', 'r', 't', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5456);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8077);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9859);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 10726);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11126);
											s.is_empty()
										}
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
			super::push(out, 5457);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', 'o', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9860);
							s.is_empty()
						}
						['p', 'o', 'h', 'p', 'y', 'l', 'e', 'k', 'o', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8078);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9862);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 'r', 'y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9863);
							s.is_empty()
						}
						['r', 'i', 'e', 'h', 'c', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9864);
							s.is_empty()
						}
						['s', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'r', 'r', 'a', 'n', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9866);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9867);
							s.is_empty()
						}
						['p', 'o', s @ ..] => {
							super::push(out, 9868);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10728);
									match s {
										[] => true,
										['c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11127);
											s.is_empty()
										}
										['k', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5458);
			match s {
				[] => true,
				['b', 'e', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8079);
					s.is_empty()
				}
				['h', 'c', 'o', 'd', 'o', 'n', 'e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 8080);
					s.is_empty()
				}
				['k', 'y', 'r', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8081);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8082);
					match s {
						[] => true,
						['a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9870);
							s.is_empty()
						}
						['o', 'g', 'r', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5459);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8084);
					match s {
						[] => true,
						['s', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 9872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9873);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10729);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11129);
											match s {
												[] => true,
												['u', 'a', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 11315);
													s.is_empty()
												}
												['v', 'a', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 11316);
													s.is_empty()
												}
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
							super::push(out, 9874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8085);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8086);
					match s {
						[] => true,
						['l', 'o', 'm', s @ ..] => {
							super::push(out, 9875);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10730);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11130);
											s.is_empty()
										}
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
			super::push(out, 5460);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 8087);
					match s {
						[] => true,
						['t', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8088);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9877);
							match s {
								[] => true,
								['r', 'g', 'o', 'm', 's', 'e', 'r', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10731);
									s.is_empty()
								}
								['t', 'i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10732);
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
						['n', s @ ..] => {
							super::push(out, 9878);
							s.is_empty()
						}
						['s', 'i', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9879);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 9880);
							match s {
								[] => true,
								['c', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10733);
									s.is_empty()
								}
								['k', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10734);
									s.is_empty()
								}
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
			super::push(out, 5461);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9881);
							s.is_empty()
						}
						['n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9882);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9883);
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 11131);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 11132);
											s.is_empty()
										}
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
					super::push(out, 8089);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8090);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5462);
			match s {
				[] => true,
				['e', 'h', s @ ..] => {
					super::push(out, 8091);
					match s {
						[] => true,
						['c', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 9885);
							s.is_empty()
						}
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9886);
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9887);
							s.is_empty()
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9888);
							s.is_empty()
						}
						['t', 'n', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8092);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9890);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10735);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 11133);
											s.is_empty()
										}
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
					super::push(out, 8093);
					match s {
						[] => true,
						['y', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8094);
					match s {
						[] => true,
						['a', 'n', 'e', 'm', '\u{1}', s @ ..] => super::t4::n10858(s, out),
						['o', 'k', 'u', 'o', 'k', '\u{1}', s @ ..] => super::t4::n10859(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t4::n5903(s, out),
		['n', s @ ..] => super::t4::n5904(s, out),
		['p', s @ ..] => super::t4::n5905(s, out),
		['r', s @ ..] => super::t4::n5906(s, out),
		['s', s @ ..] => super::t4::n5907(s, out),
		['t', s @ ..] => super::t4::n5908(s, out),
		_ => false,
	}
}

/// Words ending -cephalon (3 syllables).
pub(crate) fn n10893(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['l', 'e', 'y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11149);
					s.is_empty()
				}
				['m', 'a', 'l', 'a', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 11150);
					s.is_empty()
				}
				['n', 'i', 'h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 11151);
					s.is_empty()
				}
				['p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 11152);
					s.is_empty()
				}
				['r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omelon (3 syllables).
pub(crate) fn n5912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5471);
	match s {
		[] => true,
		['i', 'd', 'i', '\u{1}', s @ ..] => {
			super::push(out, 8121);
			s.is_empty()
		}
		['t', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olon (3 syllables).
pub(crate) fn n5915(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8123);
			match s {
				[] => true,
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9919);
					s.is_empty()
				}
				['o', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 8124);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylon (3 syllables).
pub(crate) fn n5916(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 8125);
			match s {
				[] => true,
				['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 9921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 8126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mon (3 syllables).
pub(crate) fn n2751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2549);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5474);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8127);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9922);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'c', 'a', 'r', 'd', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 5475);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8128);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8129);
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 's', 'h', 'c', 'a', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5476);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5477);
			match s {
				[] => true,
				['c', 'o', 's', 's', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8130);
					s.is_empty()
				}
				['d', 'o', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5478);
			match s {
				[] => true,
				['k', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -non (3 syllables).
pub(crate) fn n2752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2550);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5480);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5481);
			match s {
				[] => true,
				['g', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 8133);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8134);
					match s {
						[] => true,
						['o', 'p', 's', 'i', 'r', 'e', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8135);
					match s {
						[] => true,
						['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9926);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5482);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5484);
			match s {
				[] => true,
				['g', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8138);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8139);
					s.is_empty()
				}
				['k', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', 'e', 'd', 'o', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oon (3 syllables).
pub(crate) fn n2753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 5486);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5487);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'r', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 8141);
					s.is_empty()
				}
				['s', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 8142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5488);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', 'o', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9928);
							s.is_empty()
						}
						['e', 'a', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9929);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10761);
									s.is_empty()
								}
								['d', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10763);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 11154);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10764);
									s.is_empty()
								}
								['i', 'l', 'a', 'h', 'p', 'e', 'c', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10765);
									s.is_empty()
								}
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

/// Words ending -pon (3 syllables).
pub(crate) fn n2754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			super::push(out, 5489);
			match s {
				[] => true,
				['c', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8143);
					s.is_empty()
				}
				['s', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8144);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ron (3 syllables).
pub(crate) fn n2755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5490);
			match s {
				[] => true,
				['g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8145);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8146);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9930);
							s.is_empty()
						}
						['p', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5491);
			match s {
				[] => true,
				['a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8149);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8150);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => super::t4::n11861(s, out),
								['i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10766);
									s.is_empty()
								}
								['o', s @ ..] => super::t4::n11863(s, out),
								['y', 'l', 'o', 'p', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => super::t4::n12314(s, out),
										['o', s @ ..] => super::t4::n12315(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'l', 'o', 'p', '\u{1}', s @ ..] => super::t4::n10911(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t4::n8914(s, out),
				['y', 'h', 'e', 'd', '\u{1}', s @ ..] => super::t4::n8915(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n5938(s, out),
		['g', s @ ..] => super::t4::n5939(s, out),
		['h', s @ ..] => super::t4::n5940(s, out),
		['i', s @ ..] => super::t4::n5941(s, out),
		['o', s @ ..] => super::t4::n5942(s, out),
		['r', s @ ..] => super::t4::n5943(s, out),
		['t', s @ ..] => super::t4::n5944(s, out),
		['u', s @ ..] => super::t4::n5945(s, out),
		['y', 'p', 'r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => super::t4::n5946(s, out),
		_ => false,
	}
}

/// Words ending -son (3 syllables).
pub(crate) fn n2756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2554);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5500);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5501);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5502);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5503);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5504);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9952);
							s.is_empty()
						}
						['e', 'r', 'i', 'h', 's', 'k', 'r', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 9953);
							s.is_empty()
						}
						['f', 'f', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9954);
							s.is_empty()
						}
						['h', 's', s @ ..] => {
							match s {
								[] => true,
								['i', 'n', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10780);
									s.is_empty()
								}
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'e', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9955);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10782);
									s.is_empty()
								}
								['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10783);
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
										['b', s @ ..] => {
											match s {
												[] => true,
												['m', 'u', 'c', '\u{1}', s @ ..] => {
													super::push(out, 11339);
													s.is_empty()
												}
												['y', 'c', '\u{1}', s @ ..] => {
													super::push(out, 11340);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 11172);
											s.is_empty()
										}
										['h', 't', 'a', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 11173);
											s.is_empty()
										}
										['t', 's', 'y', 'o', '\u{1}', s @ ..] => {
											super::push(out, 11174);
											s.is_empty()
										}
										['v', 'o', '\u{1}', s @ ..] => {
											super::push(out, 11175);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'o', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10785);
									s.is_empty()
								}
								['m', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10786);
									s.is_empty()
								}
								['n', 'e', 'e', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 10787);
									s.is_empty()
								}
								['r', 'o', 'o', 'd', 't', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10789);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 11176);
											s.is_empty()
										}
										['o', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 11177);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10790);
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10791);
									s.is_empty()
								}
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
			super::push(out, 5505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ton (3 syllables).
pub(crate) fn n2757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2555);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5506);
			match s {
				[] => true,
				['b', 'b', s @ ..] => {
					super::push(out, 8167);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8168);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8169);
					match s {
						[] => true,
						['m', 'a', 'r', 'g', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 9956);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9957);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', 'h', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11178);
											s.is_empty()
										}
										['u', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11341);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 11342);
													s.is_empty()
												}
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
				['n', 'y', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5507);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8171);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9958);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10792);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 11179);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'o', 'p', 'o', 'r', 'i', 'e', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8172);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5508);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5509);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8174);
					match s {
						[] => true,
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5510);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8175);
					match s {
						[] => true,
						['i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9960);
							s.is_empty()
						}
						['n', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8176);
					match s {
						[] => true,
						['a', 'l', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10793);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', 'y', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11180);
											s.is_empty()
										}
										['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11181);
											s.is_empty()
										}
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 11182);
											s.is_empty()
										}
										['h', 'c', 'y', 't', '\u{1}', s @ ..] => {
											super::push(out, 11183);
											s.is_empty()
										}
										['i', 'r', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 11184);
											s.is_empty()
										}
										['l', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 11185);
											s.is_empty()
										}
										['r', 'e', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11186);
											s.is_empty()
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11187);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11188);
											s.is_empty()
										}
										['x', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11189);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'l', 'l', 'e', 'j', '\u{1}', s @ ..] => {
									super::push(out, 10794);
									s.is_empty()
								}
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
			super::push(out, 5511);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5512);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8177);
					match s {
						[] => true,
						['c', 'e', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9962);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9963);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10795);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11190);
											s.is_empty()
										}
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
					super::push(out, 8178);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8179);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8180);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9964);
							match s {
								[] => true,
								['o', 'i', 'o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10796);
									s.is_empty()
								}
								['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'i', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5513);
			s.is_empty()
		}
		['u', 'e', s @ ..] => {
			super::push(out, 5514);
			match s {
				[] => true,
				['l', 'e', 't', 'o', 'e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5515);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8182);
					s.is_empty()
				}
				['h', 'p', 'o', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8183);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xon (3 syllables).
pub(crate) fn n2758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2556);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5516);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8184);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntois (3 syllables).
pub(crate) fn n9254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8471);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10060);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pis (3 syllables).
pub(crate) fn n3241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3021);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 6034);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ris (3 syllables).
pub(crate) fn n3242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3022);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6035);
			match s {
				[] => true,
				['c', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8473);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8474);
					match s {
						[] => true,
						['c', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10061);
							s.is_empty()
						}
						['t', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8475);
					match s {
						[] => true,
						['a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10063);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10064);
							match s {
								[] => true,
								['c', 'i', 't', 'n', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 10819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', 'p', 'e', '\u{1}', s @ ..] => {
			super::push(out, 6036);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6037);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8477);
					s.is_empty()
				}
				['m', 'e', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8478);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8479);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10065);
							s.is_empty()
						}
						['p', 'o', 'y', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10066);
							s.is_empty()
						}
						['t', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6038);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6039);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6040);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8480);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8481);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8482);
					match s {
						[] => true,
						['i', 'l', 'c', 'o', 'r', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'd', 'a', 'm', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6041);
			s.is_empty()
		}
		['t', 'e', 'l', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6042);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 6043);
			match s {
				[] => true,
				['g', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sis (3 syllables).
pub(crate) fn n3243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3023);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6044);
			match s {
				[] => true,
				['t', 's', 'o', s @ ..] => {
					super::push(out, 8484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6045);
			match s {
				[] => true,
				['h', 't', 'y', 's', 'o', 'r', 't', 'c', 'e', 'l', 'e', 'o', 't', 'o', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8485);
					s.is_empty()
				}
				['m', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8486);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8487);
					match s {
						[] => true,
						['a', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8488);
					match s {
						[] => true,
						['h', 'c', 'a', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6046);
			match s {
				[] => true,
				['u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6047);
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					super::push(out, 8490);
					match s {
						[] => true,
						['i', 'r', 't', 'o', 'r', s @ ..] => {
							super::push(out, 10071);
							match s {
								[] => true,
								['e', 't', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8491);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 10072);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10073);
							match s {
								[] => true,
								['e', 't', 'c', 'a', 'b', 'o', 'r', 't', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'm', 'o', 'n', s @ ..] => {
					super::push(out, 8492);
					match s {
						[] => true,
						['a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6048);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8493);
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10075);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10076);
							s.is_empty()
						}
						['o', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10077);
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8494);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8495);
					match s {
						[] => true,
						['l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8496);
					match s {
						[] => true,
						['c', 'r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10080);
							s.is_empty()
						}
						['d', 'a', 'l', 'l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10081);
							s.is_empty()
						}
						['i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10082);
							s.is_empty()
						}
						['l', 'e', 'p', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10083);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'a', 'l', 'a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10822);
									s.is_empty()
								}
								['o', 'c', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'l', 'o', 't', 'o', s @ ..] => {
			super::push(out, 6049);
			match s {
				[] => true,
				['h', 'p', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tis (3 syllables).
pub(crate) fn n3244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3024);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6050);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8498);
					s.is_empty()
				}
				['m', 'e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'b', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 6051);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6052);
			match s {
				[] => true,
				['c', 'i', 'd', 'n', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8500);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', 'o', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10824);
									s.is_empty()
								}
								['r', 'o', 'h', 'c', 'o', 'n', 'i', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 10825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'c', 'o', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'i', 'o', 'n', 'u', 'j', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 8501);
					s.is_empty()
				}
				['g', 'n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10085);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10086);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10826);
									match s {
										[] => true,
										['l', 'a', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11345);
													s.is_empty()
												}
												['o', 't', 'o', '\u{1}', s @ ..] => {
													super::push(out, 11346);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'r', 'a', 'l', s @ ..] => {
							super::push(out, 10087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10827);
									s.is_empty()
								}
								['o', 'g', 'n', 'y', 'r', 'a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8502);
					match s {
						[] => true,
						['c', 'n', 'o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10088);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10089);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10829);
									s.is_empty()
								}
								['m', 'y', 'l', 'o', 'n', 'e', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'i', 'r', 'y', 'b', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 's', 'a', 'f', 'o', 'y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8503);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8504);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10091);
							match s {
								[] => true,
								['h', 'p', s @ ..] => {
									match s {
										[] => true,
										['e', 'c', 'n', 'e', 'o', s @ ..] => {
											match s {
												[] => true,
												['i', 'g', 'n', 'i', 'n', 'e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 11347);
													s.is_empty()
												}
												['l', 'u', 'c', 'i', 'r', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 11348);
													s.is_empty()
												}
												['t', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 11349);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'o', '\u{1}', s @ ..] => {
											super::push(out, 11200);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10092);
							s.is_empty()
						}
						['i', 's', 'n', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 10093);
							s.is_empty()
						}
						['o', 'i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10094);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 11201);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 11202);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11350);
													match s {
														[] => true,
														['a', 'r', 'o', 'l', 'a', 'h', 'p', 'e', 'c', 'n', 'e', '\u{1}', s @ ..] => super::t4::n12565(s, out),
														_ => false,
													}
												}
												['t', 'r', 'e', 'v', 'i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 11351);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 11203);
											match s {
												[] => true,
												['a', 'v', '\u{1}', s @ ..] => {
													super::push(out, 11352);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'l', 'e', 'c', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8505);
					match s {
						[] => true,
						['r', 'e', 'd', 'o', 'y', 'p', '\u{1}', s @ ..] => super::t4::n11093(s, out),
						['y', s @ ..] => super::t4::n11094(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t4::n9291(s, out),
				['r', s @ ..] => super::t4::n9292(s, out),
				['s', s @ ..] => super::t4::n9293(s, out),
				['t', s @ ..] => super::t4::n9294(s, out),
				['v', s @ ..] => super::t4::n9295(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t4::n6528(s, out),
		['o', s @ ..] => super::t4::n6529(s, out),
		['s', s @ ..] => super::t4::n6530(s, out),
		['t', s @ ..] => super::t4::n6531(s, out),
		['u', s @ ..] => super::t4::n6532(s, out),
		['y', 'r', 't', 'o', 'b', '\u{1}', s @ ..] => super::t4::n6533(s, out),
		_ => false,
	}
}

/// Words ending -vis (3 syllables).
pub(crate) fn n3245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3025);
	match s {
		[] => true,
		['e', 'r', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6059);
			s.is_empty()
		}
		['l', 'e', 'p', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8519);
					s.is_empty()
				}
				['o', 'h', 't', 'n', 'a', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

