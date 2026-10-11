use flexion::Path;

/// Words ending -ue.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1322);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5731);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5734);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5735);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5736);
					match s {
						[] => true,
						['t', 'e', s @ ..] => {
							super::push(out, 14575);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 24826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5737);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 14576);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 24827);
									s.is_empty()
								}
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
			super::push(out, 1326);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5738);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5739);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14577);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5740);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14579);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14580);
							match s {
								[] => true,
								['r', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 24828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14581);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 33222);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 33223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14582);
							match s {
								[] => true,
								['l', 'e', 's', 'i', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 24829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5741);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['e', 'v', 'r', s @ ..] => {
					super::push(out, 5742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5743);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 24830);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 24831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5744);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14584);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14585);
							match s {
								[] => true,
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 24832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14586);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14587);
							match s {
								[] => true,
								['é', 'b', '\u{1}', s @ ..] => {
									super::push(out, 24833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5745);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 14588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5746);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 24834);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 24835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5747);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14589);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5749);
					match s {
						[] => true,
						['t', 's', 's', s @ ..] => {
							super::push(out, 14591);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 24836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5750);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14593);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', '\u{1}', s @ ..] => {
			super::push(out, 1331);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5751);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5752);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5753);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5754);
					match s {
						[] => true,
						['-', 'o', 't', '-', 'e', 'c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5755);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5756);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24837);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33224);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 33225);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 24838);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 24839);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33226);
											s.is_empty()
										}
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
					super::push(out, 5757);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14597);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14598);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5759);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5760);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14601);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 14602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5761);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5762);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14603);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 14605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5765);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5766);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14609);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 14610);
							match s {
								[] => true,
								['o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 24840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5769);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14612);
							match s {
								[] => true,
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 24841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5770);
					match s {
						[] => true,
						['n', 'e', 'f', 'f', 'o', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 24842);
									s.is_empty()
								}
								['n', 'e', 'n', s @ ..] => {
									super::push(out, 24843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5771);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14613);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 24844);
									match s {
										[] => true,
										['t', 'i', 'n', 'i', 's', 'g', s @ ..] => {
											super::push(out, 33227);
											match s {
												[] => true,
												['n', 'u', s @ ..] => {
													super::push(out, 38948);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 24845);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 24846);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 33228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 14614);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24847);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33229);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 24848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14615);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14616);
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 33230);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 33231);
											s.is_empty()
										}
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
					super::push(out, 5772);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1338);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5773);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', 'e', 'r', 'ß', s @ ..] => {
							super::push(out, 14618);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 24850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 14619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -we.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5779);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1345);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1347);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5783);
					match s {
						[] => true,
						['r', 'e', 't', s @ ..] => {
							super::push(out, 14620);
							match s {
								[] => true,
								['h', 'c', 'ä', 'w', '\u{1}', s @ ..] => {
									super::push(out, 24851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5784);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 14621);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 24852);
									s.is_empty()
								}
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

/// Words ending -xe.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1349);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5785);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 14622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ye.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['-', 'e', 'y', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5787);
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ze.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1354);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5789);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5790);
					match s {
						[] => true,
						['i', 'm', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5793);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5794);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5795);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 14624);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14626);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14627);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14628);
							match s {
								[] => true,
								['n', 'o', 'i', 't', 'k', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 24853);
									s.is_empty()
								}
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
			super::push(out, 1358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5797);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5798);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5799);
					match s {
						[] => true,
						['p', 'n', 'e', 'l', 'l', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5800);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14630);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5801);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14632);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5802);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14634);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14635);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24854);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 24855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33232);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 24856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14636);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24857);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 33233);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', 'e', s @ ..] => {
							super::push(out, 14637);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 24858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14638);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 14639);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14641);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 33234);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38949);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'h', s @ ..] => {
											super::push(out, 33235);
											s.is_empty()
										}
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
					super::push(out, 5804);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14642);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 24860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 14643);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14644);
							match s {
								[] => true,
								['o', 'l', 's', '\u{1}', s @ ..] => {
									super::push(out, 24861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5805);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14645);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5806);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14647);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 24862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 5807);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 5808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5809);
					match s {
						[] => true,
						['w', 'h', s @ ..] => {
							super::push(out, 14649);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 24863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5810);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5811);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 5812);
					match s {
						[] => true,
						['w', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24864);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 5813);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14651);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t7::n1363(s, out),
		['u', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'u', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ße.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 14672);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14673);
							match s {
								[] => true,
								['n', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 24873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 14674);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 33237);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 33238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							super::push(out, 14675);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 24874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14676);
							s.is_empty()
						}
						['s', 'e', 'ã', s @ ..] => {
							super::push(out, 14677);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 24875);
									s.is_empty()
								}
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
			super::push(out, 1365);
			match s {
				[] => true,
				['i', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14678);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14679);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14680);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 14681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24877);
									s.is_empty()
								}
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
			super::push(out, 1367);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5825);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5826);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14683);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24878);
									s.is_empty()
								}
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
			super::push(out, 1368);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5827);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5828);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5829);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5830);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 14685);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 24879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', s @ ..] => {
							super::push(out, 14686);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 24880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14687);
							match s {
								[] => true,
								['e', 'l', s @ ..] => {
									super::push(out, 24881);
									s.is_empty()
								}
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
			super::push(out, 1370);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5832);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äe.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14688);
							s.is_empty()
						}
						['y', 'r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['g', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ée.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['e', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1375);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5836);
					s.is_empty()
				}
				['ê', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1376);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14690);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5838);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5839);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5840);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5841);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 14694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5842);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['s', 'u', 'a', 'h', 'c', '-', 'e', 'd', '-', 'z', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1379);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öe.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['h', 'r', 'r', 'o', 'g', s @ ..] => {
			super::push(out, 1380);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word demograf.
pub(crate) fn n41304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38950);
	s.is_empty()
}

/// The word homograf.
pub(crate) fn n41305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38951);
	s.is_empty()
}

/// The word apograf.
pub(crate) fn n41306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38952);
	s.is_empty()
}

/// Words ending -autograf.
pub(crate) fn n41307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38953);
	s.is_empty()
}

/// Words ending -rief.
pub(crate) fn n5912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5853);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 24889);
					match s {
						[] => true,
						['n', 'a', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 24890);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33247);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 33248);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 38954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 24891);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 38955);
									s.is_empty()
								}
								['p', 'p', s @ ..] => {
									super::push(out, 38956);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'h', s @ ..] => {
											super::push(out, 42273);
											s.is_empty()
										}
										['s', 'o', s @ ..] => {
											super::push(out, 42274);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42275);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42276);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', s @ ..] => {
									super::push(out, 38957);
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
						['a', s @ ..] => {
							super::push(out, 33249);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33250);
							match s {
								[] => true,
								['a', 'h', s @ ..] => {
									super::push(out, 38959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 33251);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38960);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 24892);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ff.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1390);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5855);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 14704);
							match s {
								[] => true,
								['m', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 24894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14705);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 14706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14707);
							s.is_empty()
						}
						['s', 'k', 'c', 'e', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5858);
					match s {
						[] => true,
						['b', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 14709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5859);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5860);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5861);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 14710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5862);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5863);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14711);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14712);
							match s {
								[] => true,
								['e', 'h', 'c', 'ü', 'b', '\u{1}', s @ ..] => {
									super::push(out, 24895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14713);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24896);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 33252);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 38962);
													s.is_empty()
												}
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
					super::push(out, 5865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5866);
					match s {
						[] => true,
						['p', 'n', s @ ..] => {
							super::push(out, 14715);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 24897);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 24898);
									s.is_empty()
								}
								['r', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 24899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5867);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24900);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', 'g', s @ ..] => {
													super::push(out, 38963);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 38964);
													match s {
														[] => true,
														['i', 'a', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t7::n45017(s, out),
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
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 33253);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 33254);
											match s {
												[] => true,
												['x', s @ ..] => {
													super::push(out, 38965);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 24901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5868);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5869);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14716);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 33255);
											match s {
												[] => true,
												['s', 's', 'e', 'z', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 38966);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38967);
													s.is_empty()
												}
												['i', 'e', 'f', s @ ..] => {
													super::push(out, 38968);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 33256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 14717);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14718);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 24902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5870);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14719);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t7::n5930(s, out),
				['s', s @ ..] => {
					super::push(out, 5872);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5873);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t7::n1395(s, out),
		['u', s @ ..] => super::t7::n1396(s, out),
		['ä', s @ ..] => {
			super::push(out, 1395);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => super::t7::n1398(s, out),
		['ü', s @ ..] => {
			super::push(out, 1397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bgf.
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	s.is_empty()
}

/// Words ending -if.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14740);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24932);
									s.is_empty()
								}
								['e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 24933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 14742);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14743);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14744);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1401);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5895);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5896);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', s @ ..] => {
									super::push(out, 24935);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 24936);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 33279);
											s.is_empty()
										}
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
			super::push(out, 1404);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5897);
					match s {
						[] => true,
						['r', 'e', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5898);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5899);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lf.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5901);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5902);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5903);
					match s {
						[] => true,
						['e', 'm', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5904);
					match s {
						[] => true,
						['e', 't', 's', 'i', 'e', 'm', 't', s @ ..] => {
							super::push(out, 14748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5906);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5907);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5909);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14750);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14751);
							match s {
								[] => true,
								['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 24937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5911);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14752);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24939);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 24940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14753);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 24941);
									s.is_empty()
								}
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
			super::push(out, 1409);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 5912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'h', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nf.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['s', 'r', s @ ..] => {
					super::push(out, 5914);
					match s {
						[] => true,
						['e', 'k', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1413);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 5915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -of.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 5917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5919);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24942);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 24943);
									match s {
										[] => true,
										['b', 't', s @ ..] => {
											super::push(out, 33280);
											match s {
												[] => true,
												['s', 'r', 'ü', 'f', '\u{1}', s @ ..] => {
													super::push(out, 38991);
													s.is_empty()
												}
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
					super::push(out, 5920);
					match s {
						[] => true,
						['e', 'i', 'r', 'f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24944);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											super::push(out, 33281);
											s.is_empty()
										}
										['t', 'n', s @ ..] => {
											super::push(out, 33282);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 24945);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													match s {
														[] => true,
														['n', 'i', 'k', '\u{1}', s @ ..] => super::t7::n45042(s, out),
														['r', 'ö', 'm', 't', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => super::t7::n45043(s, out),
														_ => false,
													}
												}
												['i', 't', '\u{1}', s @ ..] => {
													super::push(out, 38992);
													s.is_empty()
												}
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
					super::push(out, 5921);
					match s {
						[] => true,
						['e', 't', 't', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14755);
							s.is_empty()
						}
						['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 14756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5922);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14757);
							s.is_empty()
						}
						['h', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24946);
									s.is_empty()
								}
								['f', 'p', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 24947);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 24948);
									match s {
										[] => true,
										['c', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 33283);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 24949);
									s.is_empty()
								}
								['s', 'u', 'b', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 33284);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 33285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 't', s @ ..] => {
							super::push(out, 14758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5923);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5924);
					match s {
						[] => true,
						['t', 'h', 'c', 'i', 'r', 'e', 'g', 'e', s @ ..] => {
							super::push(out, 14759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5925);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5926);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1419);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 5928);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 5930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['o', 'l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pf.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1425);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5934);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14761);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 24950);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33286);
											s.is_empty()
										}
										['h', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 33287);
											s.is_empty()
										}
										['l', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 33288);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'u', s @ ..] => {
									super::push(out, 24951);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24952);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 24953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 14762);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 24954);
									match s {
										[] => true,
										['h', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 33289);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'r', s @ ..] => {
									super::push(out, 24955);
									s.is_empty()
								}
								['t', 't', 'e', 'w', s @ ..] => {
									super::push(out, 24956);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 33290);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14763);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 33291);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 33292);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['d', 'a', 'w', '\u{1}', s @ ..] => super::t7::n45044(s, out),
														['t', s @ ..] => super::t7::n45045(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 33293);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 38993);
													s.is_empty()
												}
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
					super::push(out, 5935);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14764);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14765);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5936);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14768);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 24958);
									match s {
										[] => true,
										['f', 'f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 33294);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 33295);
											match s {
												[] => true,
												['o', 'o', 'b', '\u{1}', s @ ..] => {
													super::push(out, 38994);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 24959);
									match s {
										[] => true,
										['s', 'h', s @ ..] => {
											super::push(out, 33296);
											match s {
												[] => true,
												['ü', 'l', 'g', '\u{1}', s @ ..] => {
													super::push(out, 38995);
													s.is_empty()
												}
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
							super::push(out, 14769);
							match s {
								[] => true,
								['n', 'e', 'v', s @ ..] => {
									super::push(out, 24960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14770);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t7::n1429(s, out),
		['u', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5944);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24984);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 24985);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24986);
									match s {
										[] => true,
										['e', 't', 'n', 'u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39000);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 39001);
													s.is_empty()
												}
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
							super::push(out, 14787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5946);
					match s {
						[] => true,
						['c', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24987);
									s.is_empty()
								}
								['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 24988);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24989);
									s.is_empty()
								}
								['t', 'i', 'm', '\u{1}', s @ ..] => super::t7::n25877(s, out),
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

/// Words ending -qf.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rf.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1431);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24991);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 33309);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 24992);
									match s {
										[] => true,
										['e', 'g', s @ ..] => {
											super::push(out, 33310);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 33311);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 39002);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 33312);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 39003);
													s.is_empty()
												}
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
				['w', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5950);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5952);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 14789);
							s.is_empty()
						}
						['d', 'n', 'a', s @ ..] => {
							super::push(out, 14790);
							s.is_empty()
						}
						['g', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14791);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 14792);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['f', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 24993);
									s.is_empty()
								}
								['h', 'i', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 24994);
									s.is_empty()
								}
								['i', 'r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 24995);
									s.is_empty()
								}
								['s', 'u', s @ ..] => {
									super::push(out, 24996);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 24997);
									s.is_empty()
								}
								['ß', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 24998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'b', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 24999);
									s.is_empty()
								}
								['e', 't', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 33313);
											s.is_empty()
										}
										['s', 'i', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 33314);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14793);
							match s {
								[] => true,
								['g', 'n', 'i', 'l', 't', 'h', 'c', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25000);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 25001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14794);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 14795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5953);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14796);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33315);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 33316);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 33317);
											s.is_empty()
										}
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
					super::push(out, 5954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 5955);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14797);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33318);
											s.is_empty()
										}
										['n', 'e', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 33319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 14798);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14799);
							match s {
								[] => true,
								['u', 'a', 't', s @ ..] => {
									super::push(out, 25002);
									s.is_empty()
								}
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

/// Words ending -tf.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1435);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uf.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 5956);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14800);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 25003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14801);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 14802);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 14803);
							match s {
								[] => true,
								['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 25004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 14804);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 25005);
									s.is_empty()
								}
								['i', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 25006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 14805);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14806);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 33320);
											match s {
												[] => true,
												['e', 'z', s @ ..] => {
													super::push(out, 39004);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'u', s @ ..] => {
											super::push(out, 33321);
											s.is_empty()
										}
										['n', 'e', 'k', s @ ..] => {
											super::push(out, 33322);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 33323);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 39005);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 39006);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 33324);
											match s {
												[] => true,
												['l', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 39007);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 39008);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 39009);
													match s {
														[] => true,
														['a', 's', 'u', 'z', '\u{1}', s @ ..] => super::t7::n45048(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ß', s @ ..] => {
											super::push(out, 33325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'ä', s @ ..] => {
							super::push(out, 14807);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 25007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t7::n6016(s, out),
				['n', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 14819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5959);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 14820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5960);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14821);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25021);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 25022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5963);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5965);
					match s {
						[] => true,
						['r', 'd', 'n', 'a', 's', 's', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5966);
					match s {
						[] => true,
						['b', 's', s @ ..] => {
							super::push(out, 14824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5967);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['e', 'v', s @ ..] => super::t7::n15117(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t7::n6028(s, out),
				['t', s @ ..] => super::t7::n6029(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vf.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 1441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äf.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	s.is_empty()
}

/// The word mulatschag.
pub(crate) fn n15122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14829);
	s.is_empty()
}

/// Words ending -phag.
pub(crate) fn n6032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5973);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 25026);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 25027);
					match s {
						[] => true,
						['l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 33340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lag.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1445);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5974);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5975);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5977);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14832);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25028);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 25029);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33341);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 33342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 25030);
							match s {
								[] => true,
								['o', 't', 'a', 's', 'n', 'e', 'd', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 33343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'm', 'e', 'r', 'b', 'n', s @ ..] => {
							super::push(out, 25031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14833);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5978);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14834);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 25033);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 33344);
									match s {
										[] => true,
										['c', 'ü', 'r', '\u{1}', s @ ..] => {
											super::push(out, 39022);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 'g', 'a', s @ ..] => {
									super::push(out, 33345);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 39023);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 33346);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 42304);
													match s {
														[] => true,
														['a', s @ ..] => super::t7::n46851(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 42305);
													match s {
														[] => true,
														['i', s @ ..] => super::t7::n46852(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 42306);
													match s {
														[] => true,
														['e', s @ ..] => super::t7::n46853(s, out),
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
									super::push(out, 33347);
									match s {
										[] => true,
										['a', 'e', s @ ..] => {
											super::push(out, 39024);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', s @ ..] => {
													super::push(out, 42307);
													match s {
														[] => true,
														['u', 'z', '\u{1}', s @ ..] => super::t7::n46854(s, out),
														_ => false,
													}
												}
												['r', 'e', 'i', s @ ..] => {
													super::push(out, 42308);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'e', 'n', s @ ..] => {
											super::push(out, 39025);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 33348);
									match s {
										[] => true,
										['o', 'v', 's', 'g', s @ ..] => {
											super::push(out, 39026);
											match s {
												[] => true,
												['n', 'u', 's', s @ ..] => {
													super::push(out, 42309);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 33349);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 39027);
											match s {
												[] => true,
												['a', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n46855(s, out),
														['t', 'u', 'a', 'h', '\u{1}', s @ ..] => super::t7::n46856(s, out),
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
									super::push(out, 33350);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 39028);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 42310);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n46857(s, out),
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
									super::push(out, 33351);
									match s {
										[] => true,
										['i', 't', 'n', 'e', 'v', 'ä', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 39029);
											s.is_empty()
										}
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
			super::push(out, 5979);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25034);
							s.is_empty()
						}
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 25035);
							s.is_empty()
						}
						['v', s @ ..] => super::t7::n25933(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t7::n6039(s, out),
		['t', s @ ..] => super::t7::n6040(s, out),
		['u', s @ ..] => super::t7::n6041(s, out),
		_ => false,
	}
}

/// Words ending -pag.
pub(crate) fn n1448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5983);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rag.
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 5984);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14838);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33352);
									match s {
										[] => true,
										['h', 'e', 'f', 's', s @ ..] => {
											super::push(out, 39030);
											match s {
												[] => true,
												['e', 'r', 'h', 'a', 'j', '\u{1}', s @ ..] => {
													super::push(out, 42311);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 33353);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 39031);
											match s {
												[] => true,
												['l', 'o', 's', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42312);
													s.is_empty()
												}
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
				['f', s @ ..] => {
					super::push(out, 14839);
					match s {
						[] => true,
						['u', 'a', 'l', s @ ..] => {
							super::push(out, 25037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'b', 'r', s @ ..] => {
					super::push(out, 14840);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14841);
					match s {
						[] => true,
						['c', 'ü', 'r', 't', 's', 'u', 'l', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 25038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14842);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 33354);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 'r', 'e', 'd', 'n', 'a', s @ ..] => {
											super::push(out, 39032);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 39033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 25039);
							match s {
								[] => true,
								['e', 'h', 'c', 'u', 'b', 'n', s @ ..] => {
									super::push(out, 33355);
									match s {
										[] => true,
										['e', 's', 's', 'a', 'l', 'k', '\u{1}', s @ ..] => {
											super::push(out, 39034);
											s.is_empty()
										}
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
					super::push(out, 14843);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33356);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 39035);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 33357);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 39036);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 33358);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 39037);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 39038);
											match s {
												[] => true,
												['n', 'u', 'r', s @ ..] => {
													super::push(out, 42313);
													match s {
														[] => true,
														['h', s @ ..] => super::t7::n46858(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 39039);
											match s {
												[] => true,
												['o', 'i', 't', 'i', s @ ..] => {
													super::push(out, 42314);
													match s {
														[] => true,
														['l', 'a', 'o', 'k', '\u{1}', s @ ..] => super::t7::n46859(s, out),
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
							super::push(out, 25040);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 39040);
											match s {
												[] => true,
												['n', 'i', 'w', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 42315);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', 'u', s @ ..] => {
											super::push(out, 39041);
											match s {
												[] => true,
												['l', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 42316);
													s.is_empty()
												}
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
							super::push(out, 25041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5985);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tag.
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1448);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 5986);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5987);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5988);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5989);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14846);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5990);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5991);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14847);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14848);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5992);
			s.is_empty()
		}
		['k', 'r', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 5993);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5994);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 14849);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14850);
					s.is_empty()
				}
				['l', 'a', 's', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 25044);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 25045);
							match s {
								[] => true,
								['n', 'i', 'l', s @ ..] => {
									super::push(out, 33359);
									s.is_empty()
								}
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
			super::push(out, 5995);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14851);
					s.is_empty()
				}
				['n', 'o', 's', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 25046);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 25047);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 't', 'n', 'e', 'v', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 25048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', 't', s @ ..] => {
					super::push(out, 14852);
					match s {
						[] => true,
						['s', 'g', 'n', 'i', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 25049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5996);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'ä', 'l', 'h', 'c', 's', 'n', 'e', 'b', 'e', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 25050);
							s.is_empty()
						}
						['i', 'e', 'f', 's', 't', 'h', s @ ..] => {
							super::push(out, 25051);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 25052);
							match s {
								[] => true,
								['p', 'u', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 33361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'i', 'w', 't', s @ ..] => {
							super::push(out, 25053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5997);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14853);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 25054);
							s.is_empty()
						}
						['u', 'a', 'l', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 25055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14854);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 14855);
					match s {
						[] => true,
						['n', 'u', 'd', s @ ..] => {
							super::push(out, 25056);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33362);
									s.is_empty()
								}
								['n', 'ü', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'n', 'n', s @ ..] => {
					super::push(out, 14856);
					match s {
						[] => true,
						['a', 'h', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 25057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14857);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 25058);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 33364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
					super::push(out, 14858);
					match s {
						[] => true,
						['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 25059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14859);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14860);
					match s {
						[] => true,
						['a', 't', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 25060);
							s.is_empty()
						}
						['e', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33365);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 33366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'n', s @ ..] => {
							super::push(out, 25061);
							match s {
								[] => true,
								['e', 'l', 'a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 33367);
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
						['e', s @ ..] => {
							super::push(out, 25062);
							s.is_empty()
						}
						['h', 'a', 'j', 'u', s @ ..] => {
							super::push(out, 25063);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 33368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14861);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 25064);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 33369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 14862);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 25065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14863);
					match s {
						[] => true,
						['b', 'b', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 25066);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 25067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 14864);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14865);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 33370);
									s.is_empty()
								}
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 33371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14866);
					match s {
						[] => true,
						['m', s @ ..] => super::t7::n25970(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14867);
					match s {
						[] => true,
						['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 25069);
							s.is_empty()
						}
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 25070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14868);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 25071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wag.
pub(crate) fn n1451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1449);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 6000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eg.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6002);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 25072);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 39045);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 39046);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', s @ ..] => {
											super::push(out, 33375);
											s.is_empty()
										}
										['l', 'i', 'g', 'r', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 33376);
											s.is_empty()
										}
										['m', 'a', 'n', 't', 'e', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 33377);
											s.is_empty()
										}
										['r', 'u', 'p', 'p', 'i', 'k', '-', 'm', 'o', 'j', '\u{1}', s @ ..] => {
											super::push(out, 33378);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 33379);
											s.is_empty()
										}
										_ => false,
									}
								}
								['a', s @ ..] => {
									super::push(out, 25073);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33380);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 33381);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 33382);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 25074);
									match s {
										[] => true,
										['n', 'a', 'l', 'k', s @ ..] => {
											super::push(out, 33383);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 25075);
									match s {
										[] => true,
										['g', 'a', s @ ..] => {
											super::push(out, 33384);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 33385);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 25076);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 33386);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 33387);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 25077);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 39047);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 39048);
													match s {
														[] => true,
														['i', 'n', 'm', 'a', 's', '\u{1}', s @ ..] => super::t7::n45074(s, out),
														_ => false,
													}
												}
												['ß', s @ ..] => {
													super::push(out, 39049);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'e', 'h', s @ ..] => {
											super::push(out, 33388);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 33389);
											match s {
												[] => true,
												['y', 'c', '\u{1}', s @ ..] => {
													super::push(out, 39050);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 33390);
											match s {
												[] => true,
												['e', 'p', s @ ..] => {
													super::push(out, 39051);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 25078);
									match s {
										[] => true,
										['g', 'n', 'u', 'b', s @ ..] => {
											super::push(out, 33391);
											match s {
												[] => true,
												['r', 'ü', 'm', 'r', 'e', 'z', '\u{1}', s @ ..] => {
													super::push(out, 39052);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 33392);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 39053);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 25079);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 25080);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 25081);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33393);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 25082);
									match s {
										[] => true,
										['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 33394);
											s.is_empty()
										}
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
					super::push(out, 6003);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 14869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6004);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => super::t7::n25986(s, out),
								['f', 'u', 'a', '\u{1}', s @ ..] => super::t7::n25987(s, out),
								['n', 'a', s @ ..] => super::t7::n25988(s, out),
								['s', s @ ..] => super::t7::n25989(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t7::n1454(s, out),
		['r', s @ ..] => super::t7::n1455(s, out),
		['t', s @ ..] => super::t7::n1456(s, out),
		['w', s @ ..] => super::t7::n1457(s, out),
		_ => false,
	}
}

/// Words ending -gg.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1456);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 6015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1457);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['i', 'h', 'c', 's', 't', '\u{1}', s @ ..] => {
					super::push(out, 6016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ig.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1461);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6017);
					s.is_empty()
				}
				['d', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6019);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14882);
							s.is_empty()
						}
						['e', 'b', 'r', 'ü', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14883);
							s.is_empty()
						}
						['r', 'e', 't', 't', 'ä', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14884);
							s.is_empty()
						}
						['s', 'r', s @ ..] => {
							super::push(out, 14885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6020);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 25096);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 25097);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 33403);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 'p', s @ ..] => {
									super::push(out, 25098);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25099);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 39054);
													s.is_empty()
												}
												['i', 'n', 'i', s @ ..] => {
													super::push(out, 39055);
													s.is_empty()
												}
												['m', 'l', 'a', 'p', 'h', s @ ..] => {
													super::push(out, 39056);
													s.is_empty()
												}
												['n', 'n', 'a', 't', '\u{1}', s @ ..] => {
													super::push(out, 39057);
													s.is_empty()
												}
												['t', 'h', 'c', 'i', 'f', '\u{1}', s @ ..] => {
													super::push(out, 39058);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 39059);
													s.is_empty()
												}
												['f', 'e', 'i', 'k', '\u{1}', s @ ..] => {
													super::push(out, 39060);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 25100);
									s.is_empty()
								}
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
			super::push(out, 1464);
			match s {
				[] => true,
				['ä', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 6022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6025);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 14886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6026);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', 'p', s @ ..] => {
									match s {
										[] => true,
										['l', 'h', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 33404);
											s.is_empty()
										}
										['s', 'n', s @ ..] => {
											super::push(out, 33405);
											match s {
												[] => true,
												['o', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 39061);
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
										['\u{1}', s @ ..] => {
											super::push(out, 33406);
											s.is_empty()
										}
										['r', 'e', 'd', 'o', '\u{1}', s @ ..] => {
											super::push(out, 33407);
											s.is_empty()
										}
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
					super::push(out, 6027);
					s.is_empty()
				}
				['ö', s @ ..] => super::t7::n6088(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6029);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 25109);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 25110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6030);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14887);
							match s {
								[] => true,
								['n', 'i', 'e', 'w', 't', 'n', s @ ..] => super::t7::n26021(s, out),
								['r', s @ ..] => super::t7::n26022(s, out),
								['s', s @ ..] => super::t7::n26023(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t7::n15194(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 't', 'i', 'f', '\u{1}', s @ ..] => super::t7::n1473(s, out),
		['z', s @ ..] => super::t7::n1474(s, out),
		['ß', s @ ..] => super::t7::n1475(s, out),
		_ => false,
	}
}

/// Words ending -lg.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6036);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14892);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14893);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 33420);
											match s {
												[] => true,
												['r', 'o', '\u{1}', s @ ..] => {
													super::push(out, 39077);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 33421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['f', 'r', 'e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14895);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 25115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'e', s @ ..] => {
							super::push(out, 14896);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mg.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1476);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 6038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ng.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['a', s @ ..] => super::t7::n1480(s, out),
		['e', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6054);
					match s {
						[] => true,
						['o', 'g', 'i', 's', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6055);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t7::n1482(s, out),
		['o', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6078);
					match s {
						[] => true,
						['t', 'e', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 15081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6079);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15082);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6080);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 15084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6081);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 15085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6082);
					match s {
						[] => true,
						['-', 'r', 'e', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15086);
							s.is_empty()
						}
						['r', 'e', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6083);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 15088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6084);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15089);
							s.is_empty()
						}
						['l', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t7::n1484(s, out),
		['ä', s @ ..] => {
			super::push(out, 1483);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -og.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1484);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6104);
					match s {
						[] => true,
						['l', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 15170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6105);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1486);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6107);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 15171);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 15172);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 15173);
							match s {
								[] => true,
								['a', 'k', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 't', s @ ..] => {
											super::push(out, 33771);
											s.is_empty()
										}
										['t', 'k', s @ ..] => {
											super::push(out, 33772);
											s.is_empty()
										}
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
					super::push(out, 6108);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							super::push(out, 15174);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6110);
					match s {
						[] => true,
						['g', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 15177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6112);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15178);
							s.is_empty()
						}
						['n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15179);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6113);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 6118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1491);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6119);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6121);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 15183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1494);
			match s {
				[] => true,
				['r', 'e', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15184);
							s.is_empty()
						}
						['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15185);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15186);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 15187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rg.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6125);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6126);
					match s {
						[] => true,
						['k', 'n', 'u', s @ ..] => {
							super::push(out, 15188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6129);
					match s {
						[] => true,
						['d', 'n', 'a', 'r', s @ ..] => {
							super::push(out, 15190);
							s.is_empty()
						}
						['f', 'u', s @ ..] => {
							super::push(out, 15191);
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							super::push(out, 15192);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15193);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
									super::push(out, 25462);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 25463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', s @ ..] => {
							super::push(out, 15194);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15195);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25464);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 33773);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 33774);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 33775);
											s.is_empty()
										}
										['h', 'c', s @ ..] => {
											super::push(out, 33776);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 39327);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 33777);
											s.is_empty()
										}
										['l', 'l', 'e', s @ ..] => {
											super::push(out, 33778);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 33779);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 33780);
											match s {
												[] => true,
												['a', 'd', '\u{1}', s @ ..] => {
													super::push(out, 39328);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 39329);
													s.is_empty()
												}
												['l', 'a', 'k', '\u{1}', s @ ..] => {
													super::push(out, 39330);
													s.is_empty()
												}
												['o', 'r', '\u{1}', s @ ..] => {
													super::push(out, 39331);
													s.is_empty()
												}
												['t', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 39332);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'e', 'w', s @ ..] => {
									super::push(out, 25465);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 25466);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33781);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 33782);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15196);
							match s {
								[] => true,
								['e', 'm', 'm', 'o', s @ ..] => {
									super::push(out, 25467);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 25468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 25469);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 25470);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 33783);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 33784);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 25471);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 25472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', s @ ..] => {
							super::push(out, 15197);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6130);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6131);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 15199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1497);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15200);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 15201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6133);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15202);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15203);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 25473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t7::n15537(s, out),
						['r', s @ ..] => {
							super::push(out, 15205);
							s.is_empty()
						}
						['s', s @ ..] => super::t7::n15539(s, out),
						['t', s @ ..] => super::t7::n15540(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6134);
					s.is_empty()
				}
				['i', s @ ..] => super::t7::n6197(s, out),
				['l', s @ ..] => super::t7::n6198(s, out),
				['r', s @ ..] => super::t7::n6199(s, out),
				['t', s @ ..] => super::t7::n6200(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ug.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1499);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6140);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6141);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6142);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6143);
					match s {
						[] => true,
						['f', 'f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1501);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 6144);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15212);
							match s {
								[] => true,
								['i', 'e', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 25476);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 25477);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 25478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15213);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25479);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 25480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 15214);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25481);
									match s {
										[] => true,
										['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 33788);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'l', 'f', 'n', 'e', s @ ..] => {
									super::push(out, 25482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 15215);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 25483);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 25484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 15216);
							match s {
								[] => true,
								['c', 'e', s @ ..] => {
									super::push(out, 25485);
									match s {
										[] => true,
										['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 33789);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'w', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', s @ ..] => {
											super::push(out, 33790);
											match s {
												[] => true,
												['r', 'g', '\u{1}', s @ ..] => {
													super::push(out, 39335);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 33791);
											match s {
												[] => true,
												['a', 'i', 'z', 'e', 'p', 's', '\u{1}', s @ ..] => {
													super::push(out, 39336);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ß', 'i', s @ ..] => {
											super::push(out, 33792);
											match s {
												[] => true,
												['e', 'r', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 39337);
													s.is_empty()
												}
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
							super::push(out, 15217);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25486);
									s.is_empty()
								}
								['ö', s @ ..] => {
									super::push(out, 25487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 15218);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15219);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25489);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 25490);
									match s {
										[] => true,
										['e', 't', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39338);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 39339);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ü', s @ ..] => {
									super::push(out, 25491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 15220);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15221);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25492);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 33793);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 39340);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 33794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 25493);
									match s {
										[] => true,
										['a', 'f', s @ ..] => super::t7::n35583(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15222);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 25494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t7::n15556(s, out),
						['z', s @ ..] => {
							super::push(out, 15224);
							match s {
								[] => true,
								['t', 'u', 'p', '\u{1}', s @ ..] => super::t7::n26466(s, out),
								_ => false,
							}
						}
						['ß', s @ ..] => super::t7::n15558(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t7::n1504(s, out),
		['h', s @ ..] => super::t7::n1505(s, out),
		['j', s @ ..] => super::t7::n1506(s, out),
		['l', s @ ..] => super::t7::n1507(s, out),
		['r', s @ ..] => super::t7::n1508(s, out),
		['z', s @ ..] => super::t7::n1509(s, out),
		_ => false,
	}
}

/// Words ending -wg.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['-', 'n', 'e', 't', 'n', 'e', 'd', 'u', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 1509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ôg.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ög.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ch.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6176);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6177);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15259);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15260);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15261);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15262);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 33809);
											s.is_empty()
										}
										['t', 't', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 33810);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 33811);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 39352);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 25530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15263);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6178);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15265);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 25531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'l', s @ ..] => {
							super::push(out, 15266);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 15267);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15268);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15269);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 33812);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 33813);
											match s {
												[] => true,
												['l', 'e', s @ ..] => {
													super::push(out, 39353);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 25532);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 25533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15270);
							match s {
								[] => true,
								['e', 't', 's', s @ ..] => {
									super::push(out, 25534);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 25535);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 33814);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 33815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 'b', s @ ..] => {
							super::push(out, 15271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6179);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6180);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15273);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 25536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'h', 'a', s @ ..] => {
							super::push(out, 15274);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 25537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'm', s @ ..] => {
							super::push(out, 15275);
							s.is_empty()
						}
						['n', 'e', 'e', 'd', 'i', 'h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 15276);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 15277);
							s.is_empty()
						}
						['s', 't', 'h', 'c', 'i', 'r', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 15278);
							s.is_empty()
						}
						['t', 'h', 'c', 'i', 'l', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6181);
					match s {
						[] => true,
						['c', s @ ..] => super::t7::n15617(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6182);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15281);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15282);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25538);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 25539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => super::t7::n15620(s, out),
						['r', s @ ..] => super::t7::n15621(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t7::n6246(s, out),
				['n', s @ ..] => super::t7::n6247(s, out),
				['o', s @ ..] => super::t7::n6248(s, out),
				['r', s @ ..] => super::t7::n6249(s, out),
				['s', s @ ..] => super::t7::n6250(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t7::n1529(s, out),
		['i', s @ ..] => super::t7::n1530(s, out),
		['l', s @ ..] => super::t7::n1531(s, out),
		['n', s @ ..] => super::t7::n1532(s, out),
		['o', s @ ..] => super::t7::n1533(s, out),
		['r', s @ ..] => super::t7::n1534(s, out),
		['s', s @ ..] => super::t7::n1535(s, out),
		['t', s @ ..] => super::t7::n1536(s, out),
		['u', s @ ..] => super::t7::n1537(s, out),
		['ä', s @ ..] => super::t7::n1538(s, out),
		_ => false,
	}
}

/// Words ending -mvieh.
pub(crate) fn n15833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15489);
	s.is_empty()
}

/// Words ending -nvieh.
pub(crate) fn n15834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15490);
	match s {
		[] => true,
		['e', 't', 's', 'r', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 25799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rvieh.
pub(crate) fn n15835(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'd', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 25800);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 25801);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -svieh.
pub(crate) fn n15836(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'n', 'u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 25802);
			s.is_empty()
		}
		['k', 'c', 'e', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 25803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tvieh.
pub(crate) fn n15837(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 25804);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 34063);
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 34064);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zvieh.
pub(crate) fn n15838(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 25805);
			s.is_empty()
		}
		['t', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 25806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßvieh.
pub(crate) fn n15839(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 25807);
			s.is_empty()
		}
		['o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 25808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -keh.
pub(crate) fn n1542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1540);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6264);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 15491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leh.
pub(crate) fn n1543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1541);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6265);
			match s {
				[] => true,
				['g', 'r', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 15492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -peh.
pub(crate) fn n1544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1542);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 6266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reh.
pub(crate) fn n1545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1543);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 6267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weh.
pub(crate) fn n1546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1544);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6268);
			s.is_empty()
		}
		['h', 'c', 'u', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6269);
			s.is_empty()
		}
		['m', 'i', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 6270);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6271);
			match s {
				[] => true,
				['e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 15494);
					s.is_empty()
				}
				['r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 15495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6272);
			s.is_empty()
		}
		['z', 'r', s @ ..] => {
			super::push(out, 6273);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 15496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zeh.
pub(crate) fn n1547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1545);
	s.is_empty()
}

/// Words ending -gh.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1547);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1548);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ih.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6274);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6275);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 15498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6276);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15499);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34065);
											s.is_empty()
										}
										['h', 'c', s @ ..] => {
											match s {
												[] => true,
												['l', 'e', '\u{1}', s @ ..] => {
													super::push(out, 39537);
													s.is_empty()
												}
												['s', 'r', s @ ..] => {
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t7::n45365(s, out),
														['i', 'h', '\u{1}', s @ ..] => super::t7::n45366(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', 'e', 'i', 't', 'n', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 34066);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 15500);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 25809);
									s.is_empty()
								}
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

/// Words ending -kh.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1550);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1551);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lh.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	s.is_empty()
}

/// Words ending -oh.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6277);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6280);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25810);
									s.is_empty()
								}
								['n', 'e', 'n', 'h', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 25811);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 25812);
									s.is_empty()
								}
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

/// Words ending -ph.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6281);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15502);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 25813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 15503);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25814);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 39538);
													match s {
														[] => true,
														['p', s @ ..] => super::t7::n45367(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 39539);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 25815);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 34067);
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													super::push(out, 39540);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 25816);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 34068);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 39541);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 34069);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 39542);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 39543);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'o', s @ ..] => {
											super::push(out, 34070);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 39544);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 39545);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 34071);
											s.is_empty()
										}
										['p', 'a', s @ ..] => {
											super::push(out, 34072);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39546);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 34073);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 39547);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											super::push(out, 34074);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 39548);
													s.is_empty()
												}
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
					super::push(out, 6282);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 25817);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25818);
									s.is_empty()
								}
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
			super::push(out, 1555);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 6284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6285);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 't', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 34075);
											s.is_empty()
										}
										['t', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 34076);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 25819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6287);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 15505);
							match s {
								[] => true,
								['r', 't', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 25820);
									s.is_empty()
								}
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
			super::push(out, 1558);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6288);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 25821);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 25822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6289);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 15506);
							match s {
								[] => true,
								['o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 25823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 15507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rh.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['a', 't', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['-', 'm', 'r', 'a', 'd', '-', 'n', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15509);
							s.is_empty()
						}
						['m', 'r', 'a', 'd', 'k', 'c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 15510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sh.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6292);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15512);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 25824);
									match s {
										[] => true,
										['r', 't', 'c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
											super::push(out, 34077);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 15513);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 25825);
									match s {
										[] => true,
										['c', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 34078);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 25826);
									match s {
										[] => true,
										['s', 'e', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 34079);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 25827);
									match s {
										[] => true,
										['c', 'a', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 34080);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6293);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 15515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6294);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25829);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25830);
									match s {
										[] => true,
										['e', 's', 'r', 'ö', 'b', '\u{1}', s @ ..] => {
											super::push(out, 34081);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 15517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6295);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1561);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6297);
					match s {
						[] => true,
						['g', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 15518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6298);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -th.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['b', 'b', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6301);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6302);
					match s {
						[] => true,
						['l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6303);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6304);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6305);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 15520);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25831);
									s.is_empty()
								}
								['d', 'l', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15522);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15523);
							match s {
								[] => true,
								['d', 'n', 'ä', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 25833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 15524);
							s.is_empty()
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 25834);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 25835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'h', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15525);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 25836);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 25837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', 'n', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 25838);
									s.is_empty()
								}
								['h', 'c', 'i', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 25839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1565);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6307);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 15527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6308);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15528);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6309);
					match s {
						[] => true,
						['a', 'z', 'e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6311);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6312);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 15532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6313);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15533);
							match s {
								[] => true,
								['g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 25840);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 25841);
									match s {
										[] => true,
										['q', 'a', '\u{1}', s @ ..] => {
											super::push(out, 34082);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15534);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 25842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15535);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15536);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 25844);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'a', '\u{1}', s @ ..] => {
											super::push(out, 34083);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 34084);
											match s {
												[] => true,
												['s', 'a', 'g', '\u{1}', s @ ..] => {
													super::push(out, 39549);
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
										['o', s @ ..] => {
											super::push(out, 34085);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 34086);
											match s {
												[] => true,
												['h', 'p', '\u{1}', s @ ..] => {
													super::push(out, 39550);
													s.is_empty()
												}
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
					super::push(out, 6314);
					match s {
						[] => true,
						['v', 'i', '\u{1}', s @ ..] => {
							super::push(out, 15537);
							s.is_empty()
						}
						['w', 'i', '\u{1}', s @ ..] => {
							super::push(out, 15538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1567);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6315);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6316);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 15539);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15540);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1568);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6317);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6318);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6319);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 6320);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6321);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 6322);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6323);
					match s {
						[] => true,
						['t', 'e', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1569);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6325);
					s.is_empty()
				}
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6326);
					s.is_empty()
				}
				['u', 'b', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 6327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1570);
			match s {
				[] => true,
				['l', s @ ..] => super::t7::n6393(s, out),
				['m', s @ ..] => super::t7::n6394(s, out),
				['o', s @ ..] => {
					super::push(out, 6330);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => super::t7::n6396(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t7::n1573(s, out),
		['y', s @ ..] => super::t7::n1574(s, out),
		['ü', s @ ..] => super::t7::n1575(s, out),
		_ => false,
	}
}

/// Words ending -uh.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', s @ ..] => {
							super::push(out, 15550);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34087);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 34088);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 34089);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 34090);
											match s {
												[] => true,
												['a', 'm', 'n', 'i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 39551);
													s.is_empty()
												}
												['e', 's', 'e', 'i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 39552);
													s.is_empty()
												}
												['l', s @ ..] => {
													match s {
														[] => true,
														['a', 'b', 'e', 's', 'a', 'b', '\u{1}', s @ ..] => super::t7::n45368(s, out),
														['e', s @ ..] => super::t7::n45369(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 34091);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', 'e', 'l', '\u{1}', s @ ..] => {
													super::push(out, 39553);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 39554);
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 34092);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 34093);
											s.is_empty()
										}
										['é', 'c', 'a', 'l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 34094);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'u', 'a', 'l', 's', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 15551);
							s.is_empty()
						}
						['g', 'n', s @ ..] => {
							super::push(out, 15552);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15553);
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 15554);
							s.is_empty()
						}
						['s', 'g', s @ ..] => {
							super::push(out, 15555);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15556);
							match s {
								[] => true,
								['t', 'i', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 25845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', 'i', s @ ..] => {
							super::push(out, 15557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1578);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6338);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6340);
					s.is_empty()
				}
				['u', 'g', 'n', 'ä', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6341);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äh.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üh.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilbi.
pub(crate) fn n6448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6382);
	s.is_empty()
}

/// Words ending -mbi.
pub(crate) fn n1605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1603);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6383);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15578);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nbi.
pub(crate) fn n1606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1604);
	s.is_empty()
}

/// Words ending -obi.
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1605);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6385);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 6386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbi.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1606);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ubi.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1607);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ci.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -di.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6390);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6393);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6394);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 6395);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6396);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6397);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1611);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6400);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15582);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 25846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6401);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6402);
					match s {
						[] => true,
						['q', 'i', 'r', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6403);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6405);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15584);
							match s {
								[] => true,
								['o', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 25847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6406);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15586);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 15587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6407);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6408);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 15589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6409);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15590);
							s.is_empty()
						}
						['r', 'i', s @ ..] => {
							super::push(out, 15591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 15592);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 25848);
									match s {
										[] => true,
										['e', 't', 't', s @ ..] => {
											super::push(out, 34095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ei.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6411);
					s.is_empty()
				}
				['o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6413);
					s.is_empty()
				}
				['a', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6414);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6416);
					match s {
						[] => true,
						['k', 'c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 15594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6417);
					s.is_empty()
				}
				['o', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['e', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6419);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6420);
					match s {
						[] => true,
						['o', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 15595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6421);
					match s {
						[] => true,
						['e', 't', 'n', 'o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 25849);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 25850);
									s.is_empty()
								}
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
			super::push(out, 1624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6422);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6424);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6425);
					s.is_empty()
				}
				['p', 'o', 's', 'o', 'l', 'i', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6426);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['c', 'o', 'j', '\u{1}', s @ ..] => {
					super::push(out, 6428);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15596);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 15597);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 15598);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 25851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15599);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15600);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25852);
									s.is_empty()
								}
								['o', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 25853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 25854);
									s.is_empty()
								}
								['t', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 25855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6431);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15601);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 25856);
									match s {
										[] => true,
										['e', 'r', s @ ..] => {
											super::push(out, 34096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 15602);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25857);
									match s {
										[] => true,
										['i', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 34097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'v', s @ ..] => {
									super::push(out, 25858);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 15603);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 25859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 15604);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 25860);
									match s {
										[] => true,
										['c', 'a', s @ ..] => {
											super::push(out, 34099);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 25861);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 34100);
											s.is_empty()
										}
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
					super::push(out, 6432);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6434);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 15607);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6435);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15609);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 25863);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6436);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 15610);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1628);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6438);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 25866);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 25867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'k', 'c', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6439);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6441);
					s.is_empty()
				}
				['e', s @ ..] => super::t7::n6509(s, out),
				['h', s @ ..] => {
					super::push(out, 6443);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6444);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['k', s @ ..] => super::t7::n6513(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t7::n1633(s, out),
		['r', s @ ..] => super::t7::n1634(s, out),
		['s', s @ ..] => super::t7::n1635(s, out),
		['t', s @ ..] => super::t7::n1636(s, out),
		['z', s @ ..] => super::t7::n1637(s, out),
		['ß', s @ ..] => super::t7::n1638(s, out),
		_ => false,
	}
}

/// Words ending -fi.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1638);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6466);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6469);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6470);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gi.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6472);
					s.is_empty()
				}
				['o', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1645);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6474);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6476);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1647);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1648);
			match s {
				[] => true,
				['f', 'u', '\u{1}', s @ ..] => {
					super::push(out, 6478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hi.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1649);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1650);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 6479);
					match s {
						[] => true,
						['i', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 15648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 6480);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15649);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25902);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 25903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15650);
							match s {
								[] => true,
								['r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 25904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6481);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 15652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6482);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 15653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6483);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6484);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6485);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15656);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15657);
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 15658);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15659);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25906);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 25907);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 15660);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15661);
							match s {
								[] => true,
								['r', 'a', 'h', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 25909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15662);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15663);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 25910);
									match s {
										[] => true,
										['a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 34139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15664);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15665);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25911);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', 'g', 'h', 'c', 's', 't', 'e', 'w', 'z', '\u{1}', s @ ..] => {
													super::push(out, 39566);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 39567);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 34140);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 25912);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 25913);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 34141);
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													super::push(out, 39568);
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													super::push(out, 39569);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 34142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 25914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 15666);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 25915);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 25916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6487);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1651);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6489);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6490);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15667);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 25917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1652);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6491);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1654);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1655);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6493);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 15669);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15670);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6494);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6495);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6496);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1656);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6498);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 15672);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6499);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1657);
			s.is_empty()
		}
		['ī', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['r', 'v', 'u', 'h', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6501);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ii.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1659);
			match s {
				[] => true,
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 6502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ji.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1661);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1662);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ki.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1664);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6503);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6504);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6505);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6506);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15675);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 15676);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15678);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6508);
					match s {
						[] => true,
						['r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 15680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15681);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15682);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 25919);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 25920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1665);
			match s {
				[] => true,
				['o', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6511);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1666);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6513);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6514);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15683);
							match s {
								[] => true,
								['o', 'r', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6515);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6516);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6517);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6518);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6519);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1667);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6521);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6523);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6525);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 6526);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 6527);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6528);
					match s {
						[] => true,
						['i', 'l', 'a', 'k', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 15685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6529);
					match s {
						[] => true,
						['e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 15686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6530);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6531);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15687);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 25922);
									s.is_empty()
								}
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
			super::push(out, 1671);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6532);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 15688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6533);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6534);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 15689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -li.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1672);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6535);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 15690);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									super::push(out, 25923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15691);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 25924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 15692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6536);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15693);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15694);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6538);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6539);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6540);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							super::push(out, 15696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6541);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 15697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1673);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 6542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6543);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15698);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15699);
							match s {
								[] => true,
								['s', 'i', '\u{1}', s @ ..] => {
									super::push(out, 25925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6544);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6545);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15700);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15701);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 25926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 15702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6547);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6548);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6549);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 6550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1676);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 6551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1678);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6552);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15703);
							match s {
								[] => true,
								['w', 's', '\u{1}', s @ ..] => {
									super::push(out, 25927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 15704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6554);
					match s {
						[] => true,
						['c', 'o', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 15705);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15706);
							match s {
								[] => true,
								['i', 'l', 'l', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 25928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 15707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6555);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15708);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15709);
							match s {
								[] => true,
								['u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6556);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6557);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 6558);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 15710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'ä', 'p', 's', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1681);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6560);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6561);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6562);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6563);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15711);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 25930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6564);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 15713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6566);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['ü', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6569);
					match s {
						[] => true,
						['d', 'n', 'ä', 'l', 'i', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15715);
							s.is_empty()
						}
						['k', 'c', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 15716);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15717);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25932);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 34143);
											s.is_empty()
										}
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
					super::push(out, 6570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1685);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6571);
					match s {
						[] => true,
						['ü', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6572);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 15720);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 15721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 6573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1686);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6574);
					s.is_empty()
				}
				['i', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6575);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6576);
					s.is_empty()
				}
				['ö', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6578);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 6579);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6580);
					match s {
						[] => true,
						['i', s @ ..] => super::t7::n16086(s, out),
						['o', s @ ..] => super::t7::n16087(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t7::n6648(s, out),
				['z', s @ ..] => super::t7::n6649(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t7::n1690(s, out),
		['ö', s @ ..] => super::t7::n1691(s, out),
		['ü', s @ ..] => super::t7::n1692(s, out),
		_ => false,
	}
}

/// Words ending -mi.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6585);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6586);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6587);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15726);
							match s {
								[] => true,
								['r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 25934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6588);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6589);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15727);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6591);
					match s {
						[] => true,
						['u', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 15730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6592);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 15731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6593);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6594);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6595);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1692);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1693);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6598);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6599);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 15733);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15735);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1696);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6600);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6601);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6602);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15738);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15739);
							match s {
								[] => true,
								['h', 'e', 'i', 'z', 'n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 25935);
									s.is_empty()
								}
								['k', 'c', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 25936);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 25937);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 25938);
									match s {
										[] => true,
										['r', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 34144);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 25939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 6603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15741);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ni.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6607);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15743);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15744);
							match s {
								[] => true,
								['f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 25940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 15745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6608);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6609);
					match s {
						[] => true,
						['o', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 15746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6610);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 15747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6611);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6612);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6613);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15748);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 25941);
									s.is_empty()
								}
								['u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 25942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15749);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15751);
							match s {
								[] => true,
								['r', 'u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 25943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 15752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6614);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 15753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15754);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1700);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1702);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6617);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6618);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15756);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 15757);
							match s {
								[] => true,
								['c', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 25944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6619);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6620);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15758);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15759);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 34145);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 34146);
											s.is_empty()
										}
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
					super::push(out, 6621);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15760);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1704);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1705);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6624);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6625);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6626);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 15763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6627);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6628);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15764);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 25945);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 25946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15765);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 25947);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 25948);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 25949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 15766);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 25950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6629);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 6631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1706);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 6632);
					match s {
						[] => true,
						['r', 'f', 'g', 'e', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 15768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1707);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6634);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 15769);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 6635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oi.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1711);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pi.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6638);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15772);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 15773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6641);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1715);
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1716);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1717);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1718);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['t', 'n', s @ ..] => {
					super::push(out, 6645);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qi.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	s.is_empty()
}

/// Words ending -ri.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1720);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6646);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6647);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15775);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6649);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15777);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 25951);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 25952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 15778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6650);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6651);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6652);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15781);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15782);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 25953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6653);
					s.is_empty()
				}
				['n', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6654);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6655);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6656);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15783);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6657);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 25954);
									match s {
										[] => true,
										['a', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 34147);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 25955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1722);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1723);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6660);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6661);
					match s {
						[] => true,
						['v', 'z', '\u{1}', s @ ..] => {
							super::push(out, 15785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6662);
					match s {
						[] => true,
						['m', 'a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 15786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6663);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1725);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6666);
					match s {
						[] => true,
						['k', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 15787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6667);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6668);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6669);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15788);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15789);
							match s {
								[] => true,
								['s', 't', 'i', '\u{1}', s @ ..] => {
									super::push(out, 25956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6670);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1728);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6672);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15790);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6673);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6674);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6675);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6676);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6677);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15792);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1731);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6678);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 15794);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1732);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1733);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6680);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6681);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15796);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15797);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6682);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6683);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15799);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 15800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6685);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15801);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6688);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6689);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6690);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 6691);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6692);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6694);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6695);
					match s {
						[] => true,
						['-', 'i', 's', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 15803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1736);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1737);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1738);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1739);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6697);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15804);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6698);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15807);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 15808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6699);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15809);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15810);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15811);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 15812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15813);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 15814);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 15815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1740);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1741);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6703);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 15816);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15817);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 1742);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1743);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 6706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6707);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6708);
					match s {
						[] => true,
						['o', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 15818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6709);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15819);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6710);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15821);
							match s {
								[] => true,
								['r', 't', 'a', 'h', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 25960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6712);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15822);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6713);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1745);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1746);
			match s {
				[] => true,
				['y', '\u{1}', s @ ..] => {
					super::push(out, 6715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1747);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['m', 'ß', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15824);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1748);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['f', 'f', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 6716);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6717);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 15826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6718);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15827);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 15829);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 15831);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15832);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 25961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15834);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25962);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 25963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6723);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1754);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6725);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6727);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 6728);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 15836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6729);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 15838);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', 'c', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 25965);
									s.is_empty()
								}
								['g', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 25966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'o', 'r', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 15839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6730);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25967);
									s.is_empty()
								}
								['a', 'l', 'l', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 25968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6731);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6732);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 15840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 6733);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 15841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ui.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6736);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vi.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1763);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1764);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1765);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wi.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1767);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15842);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 15843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6740);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 6741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 6742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1769);
			s.is_empty()
		}
		['í', s @ ..] => {
			super::push(out, 1770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xi.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 15844);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15845);
							s.is_empty()
						}
						['r', 'e', 'h', s @ ..] => {
							super::push(out, 15846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yi.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	s.is_empty()
}

/// Words ending -zi.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1772);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 6743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1774);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6744);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6745);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6747);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6748);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6750);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 6751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6752);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ći.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ck.
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1805);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15863);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 15864);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 25973);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 25974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15865);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15866);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 25975);
									match s {
										[] => true,
										['d', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 34148);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 25976);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 25977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 15867);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 15868);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15869);
							match s {
								[] => true,
								['l', 'o', s @ ..] => {
									super::push(out, 25978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15870);
							match s {
								[] => true,
								['e', 'e', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 25979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15871);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									super::push(out, 25980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 15872);
							match s {
								[] => true,
								['u', 'o', s @ ..] => {
									super::push(out, 25981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34149);
											s.is_empty()
										}
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
					super::push(out, 6790);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15873);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15874);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6791);
					match s {
						[] => true,
						['b', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 6792);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 15877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6793);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15878);
							s.is_empty()
						}
						['b', 'r', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 15879);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15880);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15881);
							match s {
								[] => true,
								['m', 'ä', 'd', '\u{1}', s @ ..] => {
									super::push(out, 25982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 15882);
							match s {
								[] => true,
								['k', 'c', 'i', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 25983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15883);
							s.is_empty()
						}
						['m', 'ä', 'd', '\u{1}', s @ ..] => {
							super::push(out, 15884);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15885);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 25984);
									s.is_empty()
								}
								['t', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 25985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 15886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t7::n6868(s, out),
				['n', s @ ..] => super::t7::n6869(s, out),
				['p', s @ ..] => super::t7::n6870(s, out),
				['r', s @ ..] => super::t7::n6871(s, out),
				['s', s @ ..] => super::t7::n6872(s, out),
				['t', s @ ..] => {
					super::push(out, 6800);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15910);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => super::t7::n6874(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t7::n1808(s, out),
		['i', s @ ..] => super::t7::n1809(s, out),
		['l', s @ ..] => super::t7::n1810(s, out),
		['n', 'ä', 'r', 't', 'e', 'g', '\u{1}', s @ ..] => super::t7::n1811(s, out),
		['o', s @ ..] => super::t7::n1812(s, out),
		['r', 'e', 'w', '\u{1}', s @ ..] => super::t7::n1813(s, out),
		['u', s @ ..] => super::t7::n1814(s, out),
		['ä', s @ ..] => super::t7::n1815(s, out),
		['ü', s @ ..] => super::t7::n1816(s, out),
		_ => false,
	}
}

/// Words ending -ek.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1815);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1816);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6860);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6861);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6862);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 16033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6863);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 16034);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 26178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6864);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6865);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16036);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 26179);
									match s {
										[] => true,
										['h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 34271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1818);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6866);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6867);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 1819);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1821);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6870);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6872);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6873);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 6874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ik.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1827);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6875);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6876);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26180);
									s.is_empty()
								}
								['n', 'e', 'd', 'o', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34272);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 34273);
											s.is_empty()
										}
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
					super::push(out, 6877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6878);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6879);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1829);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6881);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16039);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 26181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6882);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6883);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 16040);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 16041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 6884);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6885);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26182);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 26183);
									s.is_empty()
								}
								['l', 'a', 'r', 'e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 26184);
									s.is_empty()
								}
								['n', 'r', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 26185);
									s.is_empty()
								}
								['r', 'e', 'g', 'n', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 26186);
									s.is_empty()
								}
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
			super::push(out, 1831);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6886);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'd', 'ä', 'p', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 26187);
									s.is_empty()
								}
								['r', 'e', 'd', s @ ..] => {
									super::push(out, 26188);
									s.is_empty()
								}
								['s', 'i', s @ ..] => {
									super::push(out, 26189);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 26190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 26191);
									s.is_empty()
								}
								['l', 'a', 'r', s @ ..] => {
									super::push(out, 26192);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 34274);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 34275);
											s.is_empty()
										}
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
			super::push(out, 1833);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6888);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 16042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6889);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6890);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 26193);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 34276);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 34277);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 34278);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 26194);
									s.is_empty()
								}
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
			super::push(out, 1834);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6892);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 16044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6893);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6894);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 16045);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 16046);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 26195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 16047);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 26196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6895);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6896);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6897);
					match s {
						[] => true,
						['n', 'y', 'd', 'o', s @ ..] => {
							super::push(out, 16049);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16050);
							match s {
								[] => true,
								['e', 'k', 'o', 'z', s @ ..] => {
									super::push(out, 26197);
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
						['h', s @ ..] => {
							super::push(out, 16051);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 16052);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 16054);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 16055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6898);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t7::n1838(s, out),
		['p', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t7::n6984(s, out),
				['e', s @ ..] => super::t7::n6985(s, out),
				['s', s @ ..] => super::t7::n6986(s, out),
				['u', s @ ..] => super::t7::n6987(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t7::n1840(s, out),
		['s', s @ ..] => super::t7::n1841(s, out),
		['t', s @ ..] => super::t7::n1842(s, out),
		['w', s @ ..] => super::t7::n1843(s, out),
		['x', s @ ..] => super::t7::n1844(s, out),
		_ => false,
	}
}

