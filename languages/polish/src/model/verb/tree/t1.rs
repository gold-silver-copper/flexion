use flexion::Path;

/// Words ending -wić.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['u', 'l', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5445);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3348);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3349);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1572);
					match s {
						[] => true,
						['e', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1573);
					match s {
						[] => true,
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						['ś', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1574);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5447);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5448);
									match s {
										[] => true,
										['o', 'p', s @ ..] => {
											super::push(out, 7206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5449);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5451);
									s.is_empty()
								}
								['o', 'n', 'r', 'a', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7207);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3356);
							match s {
								[] => true,
								['i', 'z', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7209);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7210);
											s.is_empty()
										}
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
					super::push(out, 1575);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3359);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 3360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5453);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3362);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5455);
									s.is_empty()
								}
								['o', 'g', 'o', 'ł', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['r', 'e', 'i', 'z', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5457);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5458);
									s.is_empty()
								}
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
			super::push(out, 488);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3366);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3368);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3369);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5459);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3370);
							match s {
								[] => true,
								['r', 'e', 'i', 'c', 'e', 'i', 'n', s @ ..] => super::t1::n6052(s, out),
								_ => false,
							}
						}
						['ś', s @ ..] => super::t1::n3710(s, out),
						['ż', s @ ..] => super::t1::n3711(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => super::t1::n1706(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t1::n502(s, out),
		['o', s @ ..] => super::t1::n503(s, out),
		['r', s @ ..] => super::t1::n504(s, out),
		['t', s @ ..] => super::t1::n505(s, out),
		['u', s @ ..] => super::t1::n506(s, out),
		['y', s @ ..] => super::t1::n507(s, out),
		['z', s @ ..] => super::t1::n508(s, out),
		['ó', s @ ..] => super::t1::n509(s, out),
		['ź', s @ ..] => super::t1::n510(s, out),
		_ => false,
	}
}

/// Words ending -zić.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1606);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n512(s, out),
		['g', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3466);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3468);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['e', 'i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3470);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5569);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zyć.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['c', s @ ..] => super::t1::n546(s, out),
		['r', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['d', s @ ..] => super::t1::n3890(s, out),
						['h', 'c', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						['j', s @ ..] => super::t1::n3892(s, out),
						['m', s @ ..] => super::t1::n3893(s, out),
						['p', s @ ..] => super::t1::n3894(s, out),
						['t', s @ ..] => super::t1::n3895(s, out),
						['w', s @ ..] => super::t1::n3896(s, out),
						['ż', s @ ..] => super::t1::n3897(s, out),
						_ => false,
					}
				}
				['b', s @ ..] => super::t1::n1800(s, out),
				['d', s @ ..] => super::t1::n1801(s, out),
				['e', s @ ..] => super::t1::n1802(s, out),
				['h', s @ ..] => super::t1::n1803(s, out),
				['i', 'w', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['k', 's', '\u{1}', s @ ..] => super::t1::n3910(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t1::n1805(s, out),
				['o', s @ ..] => super::t1::n1806(s, out),
				['p', s @ ..] => super::t1::n1807(s, out),
				['t', s @ ..] => super::t1::n1808(s, out),
				['u', s @ ..] => super::t1::n1809(s, out),
				['ó', s @ ..] => super::t1::n1810(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n548(s, out),
		_ => false,
	}
}

/// Words ending -rażyć.
pub(crate) fn n1833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1705);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5767);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ważyć.
pub(crate) fn n1835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1707);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3608);
			s.is_empty()
		}
		['e', 'c', 'k', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3609);
			s.is_empty()
		}
		['o', 'n', 'w', 'ó', 'r', '\u{1}', s @ ..] => {
			super::push(out, 3610);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dżyć.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['a', 'i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5769);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eżyć.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['n', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3613);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iżyć.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['l', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lżyć.
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1716);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ożyć.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1718);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5772);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1720);
			match s {
				[] => true,
				['ę', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5774);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5775);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1722);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1723);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5777);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3625);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3626);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5779);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5780);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5781);
							match s {
								[] => true,
								['d', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3627);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rżyć.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5783);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5784);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5785);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 5786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'i', 'z', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3628);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -użyć.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1728);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3630);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1731);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5787);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5788);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1732);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1733);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5790);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5791);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5792);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3634);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5794);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5795);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5796);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5797);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5798);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7433);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7434);
									s.is_empty()
								}
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

/// Words ending -yżyć.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3636);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3637);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zżyć.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	s.is_empty()
}

/// Words ending -óżyć.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	s.is_empty()
}

/// Words ending -ążyć.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3638);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3639);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3640);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1736);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5799);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5801);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3644);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5803);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ężyć.
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1737);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3645);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3646);
					match s {
						[] => true,
						['e', 'i', 'c', 'u', '\u{1}', s @ ..] => {
							super::push(out, 5805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1738);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3648);
					s.is_empty()
				}
				['t', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 1739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łżyć.
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	s.is_empty()
}

/// Words ending -nąć.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['t', 's', 'y', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3654);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3655);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 3656);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7435);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7436);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7437);
											s.is_empty()
										}
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
					super::push(out, 1782);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['i', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5806);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5807);
									s.is_empty()
								}
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
			super::push(out, 562);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3658);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3659);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7438);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 7439);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3661);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 5809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3662);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7440);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3663);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['a', 'i', 'm', 'ś', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						['e', 'i', 'm', 'ś', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5810);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5811);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['z', 'r', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5813);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5814);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 1796);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['i', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5816);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5817);
									s.is_empty()
								}
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
			super::push(out, 564);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n1929(s, out),
				['e', s @ ..] => super::t1::n1930(s, out),
				['i', s @ ..] => super::t1::n1931(s, out),
				['l', s @ ..] => super::t1::n1932(s, out),
				['r', s @ ..] => super::t1::n1933(s, out),
				['u', s @ ..] => super::t1::n1934(s, out),
				['y', s @ ..] => super::t1::n1935(s, out),
				['z', s @ ..] => super::t1::n1936(s, out),
				['ą', s @ ..] => super::t1::n1937(s, out),
				['ę', s @ ..] => super::t1::n1938(s, out),
				['ź', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t1::n586(s, out),
		['i', s @ ..] => super::t1::n587(s, out),
		['j', s @ ..] => super::t1::n588(s, out),
		['k', s @ ..] => super::t2::n589(s, out),
		['l', s @ ..] => super::t2::n590(s, out),
		['m', s @ ..] => super::t2::n591(s, out),
		['o', s @ ..] => super::t2::n592(s, out),
		['p', s @ ..] => super::t2::n593(s, out),
		['r', s @ ..] => super::t2::n594(s, out),
		['s', s @ ..] => super::t2::n595(s, out),
		['t', s @ ..] => super::t2::n596(s, out),
		['u', s @ ..] => super::t2::n597(s, out),
		['w', s @ ..] => super::t2::n598(s, out),
		['y', s @ ..] => super::t2::n599(s, out),
		['z', s @ ..] => super::t2::n600(s, out),
		['ż', s @ ..] => super::t2::n601(s, out),
		_ => false,
	}
}

/// Words ending -zwieść.
pub(crate) fn n4262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3866);
	s.is_empty()
}

/// Words ending -jeść.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1897);
			s.is_empty()
		}
		['o', 'd', 'a', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leść.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ześć.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iść.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jść.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1903);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3868);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5997);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 5998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1905);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3870);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3871);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 597);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 598);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -óść.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąść.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3873);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3874);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3875);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3876);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3877);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5999);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3878);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3879);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3880);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6001);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6003);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 6004);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7519);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 6006);
									s.is_empty()
								}
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

/// Words ending -ndać.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdać.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['e', 'm', 'a', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iudać.
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	s.is_empty()
}

/// Words ending -wdać.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	s.is_empty()
}

/// Words ending -ydać.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	s.is_empty()
}

/// Words ending -zdać.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['w', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3932);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3933);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ądać.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3935);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3936);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3937);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6021);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 3938);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3939);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zgać.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['ś', 'o', s @ ..] => {
							super::push(out, 3963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1988);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3964);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iągać.
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 709);
	match s {
		[] => true,
		['c', 'd', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3966);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3967);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rągać.
pub(crate) fn n741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 710);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3968);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęgać.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 711);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['s', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łgać.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 715);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3970);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3971);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3972);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3973);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3974);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źgać.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1998);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uchać.
pub(crate) fn n757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 725);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2018);
			s.is_empty()
		}
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2020);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 4005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2022);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4006);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6033);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6034);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6035);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6036);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6037);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 7523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6038);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6039);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7524);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7525);
									s.is_empty()
								}
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

/// Words ending -wdychać.
pub(crate) fn n4412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4013);
	s.is_empty()
}

/// Words ending -ydychać.
pub(crate) fn n4413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4014);
	s.is_empty()
}

/// Words ending -zdychać.
pub(crate) fn n4414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4015);
	s.is_empty()
}

/// Words ending -mychać.
pub(crate) fn n2186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2025);
	s.is_empty()
}

/// Words ending -pychać.
pub(crate) fn n2187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2026);
	match s {
		[] => true,
		['w', 'o', s @ ..] => {
			super::push(out, 4016);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 6040);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rychać.
pub(crate) fn n2188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2027);
	s.is_empty()
}

/// Words ending -sychać.
pub(crate) fn n2189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2028);
	s.is_empty()
}

/// Words ending -zchać.
pub(crate) fn n759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	s.is_empty()
}

/// Words ending -ąchać.
pub(crate) fn n760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 728);
	match s {
		[] => true,
		['w', '\u{1}', s @ ..] => {
			super::push(out, 2029);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yhać.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	s.is_empty()
}

/// Words ending -epiać.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 6047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6048);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -upiać.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	match s {
		[] => true,
		['r', 'k', s @ ..] => {
			super::push(out, 2059);
			s.is_empty()
		}
		['ł', 'g', 'y', s @ ..] => {
			super::push(out, 2060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ypiać.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4035);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['z', 'o', s @ ..] => {
							super::push(out, 6050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4037);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4038);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4040);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4041);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siać.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wiać.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 757);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2063);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2065);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4044);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4045);
							match s {
								[] => true,
								['z', 'a', s @ ..] => {
									super::push(out, 6051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 4046);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4047);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4048);
							match s {
								[] => true,
								['d', 'a', s @ ..] => {
									super::push(out, 6052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4049);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6053);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6054);
									match s {
										[] => true,
										['r', 'o', s @ ..] => {
											super::push(out, 7526);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 6055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2068);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 6056);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 7527);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7528);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 6057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6058);
									match s {
										[] => true,
										['o', 'p', s @ ..] => {
											super::push(out, 7529);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 6059);
									match s {
										[] => true,
										['z', 'o', s @ ..] => {
											super::push(out, 7530);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 6060);
									match s {
										[] => true,
										['o', 'r', 'o', s @ ..] => {
											super::push(out, 7531);
											s.is_empty()
										}
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
					super::push(out, 2070);
					match s {
						[] => true,
						['r', 'k', 'z', s @ ..] => {
							super::push(out, 4052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 4053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 759);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4054);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4055);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6061);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6062);
									s.is_empty()
								}
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
			super::push(out, 761);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4056);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4057);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2074);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4059);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 4060);
							s.is_empty()
						}
						['ż', s @ ..] => {
							super::push(out, 4061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2075);
					match s {
						[] => true,
						['d', 'y', s @ ..] => {
							super::push(out, 4062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 764);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2080);
					match s {
						[] => true,
						['a', 'm', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4064);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ziać.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['m', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bzikać.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	s.is_empty()
}

/// The word śpiulkać.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	s.is_empty()
}

/// Words ending -mkać.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	s.is_empty()
}

/// Words ending -okać.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['m', 'c', '\u{1}', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word kupkać.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	s.is_empty()
}

/// Words ending -rkać.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	match s {
		[] => true,
		['a', 'm', 's', 'y', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -skać.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2132);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6083);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['a', 'k', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2135);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6085);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6086);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7533);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6087);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 4100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2137);
					match s {
						[] => true,
						['ś', s @ ..] => {
							super::push(out, 4101);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 810);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2139);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2140);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4103);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6090);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6091);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6092);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6093);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6095);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tkać.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ukać.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 815);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 816);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['c', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 819);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 820);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 821);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2152);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 4105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4106);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4107);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ykać.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 827);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2156);
					match s {
						[] => true,
						['z', 'o', s @ ..] => {
							super::push(out, 4109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2157);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4110);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6097);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 6098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4111);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 2160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2163);
					match s {
						[] => true,
						['s', 'p', 'y', s @ ..] => {
							super::push(out, 4113);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 6099);
									s.is_empty()
								}
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
			super::push(out, 829);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2164);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4114);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2165);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4117);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		['ł', 'o', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zkać.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 834);
			s.is_empty()
		}
		['s', 'e', 'i', 'm', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2171);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąkać.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 835);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4119);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4120);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4121);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ćkać.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ękać.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['z', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 843);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word łkać.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	s.is_empty()
}

/// The word skulać.
pub(crate) fn n2384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	s.is_empty()
}

/// Words ending -lulać.
pub(crate) fn n916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 873);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mulać.
pub(crate) fn n917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 874);
	s.is_empty()
}

/// Words ending -pulać.
pub(crate) fn n918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 875);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tulać.
pub(crate) fn n919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 876);
	s.is_empty()
}

/// Words ending -zulać.
pub(crate) fn n920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	s.is_empty()
}

/// Words ending -wlać.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['a', 'p', 'e', 'i', 'n', 'a', 'j', 'd', 'o', '\u{1}', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylać.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zlać.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['a', 'k', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ślać.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6102);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 4138);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4139);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4140);
							match s {
								[] => true,
								['w', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7538);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7539);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4141);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6104);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6105);
									s.is_empty()
								}
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

/// Words ending -wierać.
pub(crate) fn n2459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2278);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4176);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4177);
			match s {
				[] => true,
				['i', 'n', 'o', 'p', 's', s @ ..] => {
					super::push(out, 6119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4178);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4179);
			match s {
				[] => true,
				['o', 'o', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4180);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4181);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -merać.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 963);
	s.is_empty()
}

/// Words ending -perać.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 964);
	match s {
		[] => true,
		['z', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -terać.
pub(crate) fn n1016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2280);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2281);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żerać.
pub(crate) fn n1017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4187);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -grać.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 970);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 973);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrać.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4190);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4191);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mrać.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	s.is_empty()
}

/// Words ending -orać.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 974);
			s.is_empty()
		}
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
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prać.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 983);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -srać.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 987);
			s.is_empty()
		}
		['a', 'n', s @ ..] => {
			super::push(out, 988);
			s.is_empty()
		}
		['e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 989);
			s.is_empty()
		}
		['o', 'p', s @ ..] => {
			super::push(out, 990);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trać.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'k', 's', s @ ..] => {
			super::push(out, 993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urać.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['d', 'z', 'b', 'u', '\u{1}', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrać.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -órać.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	s.is_empty()
}

/// Words ending -gotać.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1018);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2329);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					super::push(out, 4213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2330);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4215);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 6127);
							s.is_empty()
						}
						['g', 'a', s @ ..] => {
							super::push(out, 6128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2331);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2332);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 4216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2333);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4217);
					match s {
						[] => true,
						['j', 'a', s @ ..] => {
							super::push(out, 6129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['r', 'd', 'z', s @ ..] => {
							super::push(out, 6130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hotać.
pub(crate) fn n1075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4219);
					match s {
						[] => true,
						['ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7542);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4220);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7544);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7546);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4221);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7548);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7549);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7550);
									s.is_empty()
								}
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

/// Words ending -kotać.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2337);
			match s {
				[] => true,
				['l', 'k', 'z', s @ ..] => {
					super::push(out, 4224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'a', 'r', 't', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2338);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4225);
					s.is_empty()
				}
				['e', 't', 'a', s @ ..] => {
					super::push(out, 4226);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6132);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7551);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 6133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4227);
					match s {
						[] => true,
						['ł', 'o', s @ ..] => {
							super::push(out, 6134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'e', 'b', 'y', s @ ..] => {
			super::push(out, 2341);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 4229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -motać.
pub(crate) fn n1078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2342);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2343);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 4231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -potać.
pub(crate) fn n1079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1023);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2344);
			match s {
				[] => true,
				['z', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2345);
			match s {
				[] => true,
				['ł', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6138);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6139);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7552);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7553);
									s.is_empty()
								}
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
			super::push(out, 2346);
			match s {
				[] => true,
				['l', 'h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6141);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptać.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4232);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stać.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2355);
					match s {
						[] => true,
						['a', 'z', s @ ..] => {
							super::push(out, 4235);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6143);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6144);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 6145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4237);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['z', 'r', 'p', 'a', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['w', 'ś', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2358);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4241);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2360);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2362);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6146);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6147);
									s.is_empty()
								}
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
			super::push(out, 1034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2364);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['o', 'p', 'h', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['c', 'y', 'w', 't', 'r', 'a', 'm', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2368);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6148);
									match s {
										[] => true,
										['k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8265);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 8266);
													s.is_empty()
												}
												['y', s @ ..] => {
													super::push(out, 8267);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 6149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['ł', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utać.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'i', 'w', 'h', 'c', 'z', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytać.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2372);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2373);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2377);
					match s {
						[] => true,
						['g', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6150);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 6151);
									s.is_empty()
								}
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

/// Words ending -ztać.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['s', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ątać.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4249);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4251);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4252);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 6152);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7554);
											s.is_empty()
										}
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

/// Words ending -ętać.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['m', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2379);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2381);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łtać.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['e', 'b', 'z', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -śtać.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['u', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2383);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bolewać.
pub(crate) fn n2598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2407);
	s.is_empty()
}

/// Words ending -zewać.
pub(crate) fn n1116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1058);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4277);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iwać.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				['e', 'l', 'y', s @ ..] => {
					super::push(out, 2409);
					s.is_empty()
				}
				['i', 'm', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2410);
					s.is_empty()
				}
				['u', 'ł', 's', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4279);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4280);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 6165);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4283);
							match s {
								[] => true,
								['l', 'ś', 'b', s @ ..] => {
									super::push(out, 6167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['o', 'z', 'c', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6168);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6169);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6170);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 6171);
									s.is_empty()
								}
								['y', s @ ..] => {
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 7555);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7556);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'a', s @ ..] => {
							super::push(out, 4286);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4287);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 7557);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7558);
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
								['m', s @ ..] => {
									super::push(out, 6172);
									s.is_empty()
								}
								['ł', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7559);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 7560);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7561);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7562);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 7563);
											s.is_empty()
										}
										['y', 'z', s @ ..] => {
											super::push(out, 7564);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ą', s @ ..] => {
							super::push(out, 4288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['k', 's', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 6173);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6174);
									s.is_empty()
								}
								['e', 'z', 'r', s @ ..] => {
									super::push(out, 6175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['z', 'c', 'z', 's', 'o', s @ ..] => {
							super::push(out, 4290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['r', 'e', 't', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2418);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'k', 'y', s @ ..] => {
									super::push(out, 6176);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 6178);
									match s {
										[] => true,
										['d', 'o', '\u{1}', s @ ..] => {
											super::push(out, 7565);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', 'd', s @ ..] => {
											super::push(out, 7566);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 7567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6179);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6180);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 6181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2420);
					match s {
						[] => true,
						['r', s @ ..] => super::t2::n4717(s, out),
						['w', s @ ..] => {
							super::push(out, 4292);
							s.is_empty()
						}
						['z', s @ ..] => super::t2::n4719(s, out),
						_ => false,
					}
				}
				['z', 's', 'e', 'i', 'm', 'o', s @ ..] => super::t2::n2616(s, out),
				['ą', s @ ..] => super::t2::n2617(s, out),
				['ę', s @ ..] => super::t2::n2618(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n1120(s, out),
		['p', s @ ..] => super::t2::n1121(s, out),
		['w', s @ ..] => super::t2::n1122(s, out),
		_ => false,
	}
}

/// Words ending -ować.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2428);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2429);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'l', 'p', 'a', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4300);
							s.is_empty()
						}
						['r', 'p', 'a', 'a', s @ ..] => {
							super::push(out, 4301);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 6185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2434);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 6186);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7568);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7569);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7570);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7571);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7572);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 6187);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 6188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'w', 'z', s @ ..] => {
							super::push(out, 4302);
							s.is_empty()
						}
						['o', 's', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'a', s @ ..] => {
									super::push(out, 6189);
									s.is_empty()
								}
								['d', 'a', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7573);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 7574);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6190);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7575);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7576);
											s.is_empty()
										}
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
					super::push(out, 2435);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4303);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4304);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6191);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 6192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4305);
							match s {
								[] => true,
								['ś', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7577);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 7578);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4306);
							match s {
								[] => true,
								['n', 'i', 'a', s @ ..] => {
									super::push(out, 6193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4307);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 2437);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['u', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n2634(s, out),
				['b', s @ ..] => {
					super::push(out, 2440);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6195);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2441);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4315);
							s.is_empty()
						}
						['w', s @ ..] => super::t2::n4747(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n2637(s, out),
				['j', s @ ..] => super::t2::n2638(s, out),
				['l', s @ ..] => super::t2::n2639(s, out),
				['n', s @ ..] => super::t2::n2640(s, out),
				['o', s @ ..] => super::t2::n2641(s, out),
				['p', s @ ..] => super::t2::n2642(s, out),
				['r', s @ ..] => super::t2::n2643(s, out),
				['u', s @ ..] => super::t2::n2644(s, out),
				['y', s @ ..] => super::t2::n2645(s, out),
				['ń', 'a', 't', 'b', 'o', '\u{1}', s @ ..] => super::t2::n2646(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n1125(s, out),
		['e', 'r', 'k', 'y', s @ ..] => super::t2::n1126(s, out),
		['f', s @ ..] => super::t2::n1127(s, out),
		['g', s @ ..] => super::t2::n1128(s, out),
		['h', s @ ..] => super::t2::n1129(s, out),
		['i', s @ ..] => super::t2::n1130(s, out),
		['j', s @ ..] => super::t2::n1131(s, out),
		['k', s @ ..] => super::t2::n1132(s, out),
		['l', s @ ..] => super::t2::n1133(s, out),
		['m', s @ ..] => super::t2::n1134(s, out),
		['n', s @ ..] => super::t2::n1135(s, out),
		['p', s @ ..] => super::t2::n1136(s, out),
		['r', s @ ..] => super::t2::n1137(s, out),
		['s', s @ ..] => super::t2::n1138(s, out),
		['t', s @ ..] => super::t2::n1139(s, out),
		['u', s @ ..] => super::t2::n1140(s, out),
		['w', s @ ..] => super::t2::n1141(s, out),
		['z', s @ ..] => super::t2::n1142(s, out),
		['ł', s @ ..] => super::t2::n1143(s, out),
		['ż', s @ ..] => super::t2::n1144(s, out),
		_ => false,
	}
}

/// Words ending -rwać.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1087);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1089);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1090);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2651);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -twać.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['a', 'm', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2652);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uwać.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1092);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1093);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2657);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4875);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4877);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4878);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 6903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4879);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ywać.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2660);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4883);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['z', 'r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 6904);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2662);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4885);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4887);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2664);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 2666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				['e', 'i', 'b', 'o', 'a', s @ ..] => {
					super::push(out, 2668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2669);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4890);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 6906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4891);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 6907);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8073);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 6908);
									match s {
										[] => true,
										['a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 8074);
											s.is_empty()
										}
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
					super::push(out, 2670);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6909);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6910);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2671);
					match s {
						[] => true,
						['e', 'i', 'p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2672);
					match s {
						[] => true,
						['ł', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 6912);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6913);
									match s {
										[] => true,
										['w', 'o', s @ ..] => {
											super::push(out, 8075);
											s.is_empty()
										}
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
					super::push(out, 2673);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2674);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2675);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2677);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2678);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4893);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2679);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 4895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2680);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['ó', 'r', 'y', 'z', s @ ..] => {
							super::push(out, 4896);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2682);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4897);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4898);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4899);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8076);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 8077);
											s.is_empty()
										}
										['z', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8501);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 8502);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4900);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 4901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2683);
					match s {
						[] => true,
						['l', 'k', 'a', s @ ..] => {
							super::push(out, 4902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2684);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2686);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6915);
									s.is_empty()
								}
								['e', s @ ..] => super::t2::n7693(s, out),
								['o', s @ ..] => super::t2::n7694(s, out),
								['w', s @ ..] => super::t2::n7695(s, out),
								['y', s @ ..] => super::t2::n7696(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t2::n5431(s, out),
						['ł', s @ ..] => super::t2::n5432(s, out),
						_ => false,
					}
				}
				['y', s @ ..] => super::t2::n2895(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n1163(s, out),
		['s', s @ ..] => super::t2::n1164(s, out),
		['t', s @ ..] => super::t2::n1165(s, out),
		['w', s @ ..] => super::t2::n1166(s, out),
		['z', s @ ..] => super::t2::n1167(s, out),
		['ł', s @ ..] => super::t2::n1168(s, out),
		['ż', s @ ..] => super::t2::n1169(s, out),
		_ => false,
	}
}

/// Words ending -zwać.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1111);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 1113);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gęszczać.
pub(crate) fn n7753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6975);
	s.is_empty()
}

/// Words ending -zęszczać.
pub(crate) fn n7754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6976);
	s.is_empty()
}

/// Words ending -ączać.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1125);
	s.is_empty()
}

/// Words ending -ęczać.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2746);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńczać.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1126);
	s.is_empty()
}

/// Words ending -dzać.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2748);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4963);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6977);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4964);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2749);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2750);
					match s {
						[] => true,
						['z', 's', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6979);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 4967);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4968);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4970);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6981);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2753);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4972);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4974);
							match s {
								[] => true,
								['o', 'd', s @ ..] => {
									super::push(out, 6983);
									s.is_empty()
								}
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
			super::push(out, 1128);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2755);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2756);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1129);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['y', 'z', 'r', 'b', 'y', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2760);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4975);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4976);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4977);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 6984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4978);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ezać.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['z', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2761);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2762);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -izać.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2763);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2764);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rzać.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['d', 'y', s @ ..] => {
					super::push(out, 2765);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2766);
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4980);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4981);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4982);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4983);
							match s {
								[] => true,
								['k', 'e', 'z', 'r', 'p', 'o', s @ ..] => {
									super::push(out, 6986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4984);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 6987);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2767);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2768);
					match s {
						[] => true,
						['w', 'y', s @ ..] => {
							super::push(out, 4986);
							match s {
								[] => true,
								['z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6989);
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
						['t', 'y', s @ ..] => {
							super::push(out, 4987);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1136);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2769);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6990);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 6991);
									match s {
										[] => true,
										['z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 6992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 6993);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2770);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4989);
							match s {
								[] => true,
								['z', 's', 'y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 6995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2772);
					s.is_empty()
				}
				['y', 'z', 'r', 'p', 'a', s @ ..] => {
					super::push(out, 2773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['e', 'i', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4991);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1141);
			match s {
				[] => true,
				['e', 'i', 'w', 'o', 'p', 'd', s @ ..] => {
					super::push(out, 2774);
					s.is_empty()
				}
				['s', 'o', 'b', s @ ..] => {
					super::push(out, 2775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 2776);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 4993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2777);
					match s {
						[] => true,
						['d', 'o', 'o', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -szać.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2778);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4995);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2779);
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 4997);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6996);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6997);
									s.is_empty()
								}
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
			super::push(out, 1144);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6998);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 6999);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7000);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7001);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7002);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7003);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7004);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 7005);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8102);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4999);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1145);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1146);
			match s {
				[] => true,
				['e', 'i', 'n', 'm', 'z', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1147);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2782);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2783);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 5002);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5003);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7006);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7007);
									s.is_empty()
								}
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

/// Words ending -uzać.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['t', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yzać.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	s.is_empty()
}

/// Words ending -ązać.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['i', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2787);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 5004);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łzać.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	s.is_empty()
}

/// Words ending -mieć.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1190);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1191);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5033);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1193);
			match s {
				[] => true,
				['h', 'c', 'u', 'r', 'e', 'i', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5035);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5037);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2835);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5039);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8105);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8106);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 8515);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 8516);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 8107);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5040);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7029);
									s.is_empty()
								}
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
			super::push(out, 1195);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7030);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7031);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8109);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8110);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 7032);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7034);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 1197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nieć.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['c', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2839);
					s.is_empty()
				}
				['m', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				['r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5042);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5043);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2842);
					match s {
						[] => true,
						['ł', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7036);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7037);
									s.is_empty()
								}
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
			super::push(out, 1199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2843);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2844);
					match s {
						[] => true,
						['ł', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7038);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2845);
					match s {
						[] => true,
						['i', 'b', 'z', s @ ..] => {
							super::push(out, 5045);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['r', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7040);
									s.is_empty()
								}
								['s', 'w', 'o', 'p', 's', s @ ..] => {
									super::push(out, 7041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2846);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2847);
					match s {
						[] => true,
						['g', 'a', 'ł', 'z', s @ ..] => {
							super::push(out, 5046);
							s.is_empty()
						}
						['r', 'y', 'w', 'z', s @ ..] => {
							super::push(out, 5047);
							s.is_empty()
						}
						['ł', 'g', 'z', s @ ..] => {
							super::push(out, 5048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'w', 't', 's', s @ ..] => {
					super::push(out, 2848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7042);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 7043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8111);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8112);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'r', 'e', 'z', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7045);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7046);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7047);
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
						['e', 'i', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7048);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5050);
							s.is_empty()
						}
						['l', 'u', 'p', 's', s @ ..] => {
							super::push(out, 5051);
							s.is_empty()
						}
						['ó', 'r', 'p', 's', s @ ..] => {
							super::push(out, 5052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5053);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5055);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5056);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', 'u', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2849);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['z', 'r', 'o', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5058);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'i', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5060);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1203);
			match s {
				[] => true,
				['a', 'm', 'r', 'o', 'n', 'z', s @ ..] => {
					super::push(out, 2851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7050);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7051);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 7052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'c', 'k', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['p', s @ ..] => super::t2::n5613(s, out),
						['t', s @ ..] => super::t2::n5614(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n1276(s, out),
		['o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1205);
			s.is_empty()
		}
		['p', s @ ..] => super::t2::n1278(s, out),
		['r', s @ ..] => super::t2::n1279(s, out),
		['t', s @ ..] => super::t2::n1280(s, out),
		['w', s @ ..] => super::t2::n1281(s, out),
		['y', 't', 'e', 'r', 'k', '\u{1}', s @ ..] => super::t2::n1282(s, out),
		['z', s @ ..] => super::t2::n1283(s, out),
		['ś', s @ ..] => super::t2::n1284(s, out),
		['ż', s @ ..] => super::t2::n1285(s, out),
		_ => false,
	}
}

/// The word kipieć.
pub(crate) fn n5660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5098);
	s.is_empty()
}

/// Words ending -ykipieć.
pub(crate) fn n5661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5099);
	s.is_empty()
}

/// Words ending -opieć.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1215);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5100);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rpieć.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1216);
	match s {
		[] => true,
		['e', 'i', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5102);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5103);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5104);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5105);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5106);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 5107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upieć.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1217);
	match s {
		[] => true,
		['r', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5108);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 5109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ypieć.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1218);
	match s {
		[] => true,
		['r', 'h', 'c', 'a', s @ ..] => {
			super::push(out, 2868);
			s.is_empty()
		}
		['z', 'r', 'k', 's', 'a', s @ ..] => {
			super::push(out, 2869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -skąpieć.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2870);
			s.is_empty()
		}
		['e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ępieć.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1219);
	match s {
		[] => true,
		['t', '\u{1}', s @ ..] => {
			super::push(out, 2872);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word uśpieć.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1220);
	s.is_empty()
}

/// Words ending -sieć.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1221);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2873);
					s.is_empty()
				}
				['t', 'a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5110);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['ł', 'y', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wieć.
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1224);
			match s {
				[] => true,
				['l', 'ś', 'o', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'z', 'd', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2876);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5114);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5116);
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['r', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5117);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7090);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7091);
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
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						['d', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7092);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7093);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7094);
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
						['\u{1}', s @ ..] => {
							super::push(out, 5119);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5121);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'ą', 'r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5123);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['a', 'j', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7095);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7097);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7098);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', 'ó', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5125);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5126);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5127);
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
				['r', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5128);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'c', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2879);
					s.is_empty()
				}
				['u', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5130);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'r', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5132);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5133);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'b', 'o', 'r', 'z', s @ ..] => {
							super::push(out, 5135);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5136);
							match s {
								[] => true,
								['r', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 7100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ź', 'e', 'z', 'r', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2880);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2881);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zieć.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2883);
					match s {
						[] => true,
						['i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5137);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2884);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7101);
									match s {
										[] => true,
										['ś', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8517);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 8518);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8120);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8121);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 8122);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8123);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8124);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8125);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8126);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 7103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8127);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8128);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 8519);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 8520);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 8129);
											match s {
												[] => true,
												['w', '\u{1}', s @ ..] => {
													super::push(out, 8521);
													s.is_empty()
												}
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
					super::push(out, 2885);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7104);
									s.is_empty()
								}
								['a', 'n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8130);
											s.is_empty()
										}
										['e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 8131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7105);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7106);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7107);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7108);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2886);
					match s {
						[] => true,
						['z', 'r', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7110);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2887);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5139);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8132);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8133);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 7112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2888);
					match s {
						[] => true,
						['r', 'z', s @ ..] => {
							super::push(out, 5141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2889);
					match s {
						[] => true,
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 5142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 2890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['p', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nakrzyczeć.
pub(crate) fn n9462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8522);
	s.is_empty()
}

/// Words ending -zakrzyczeć.
pub(crate) fn n9463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8523);
	s.is_empty()
}

/// Words ending -pokrzyczeć.
pub(crate) fn n9464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8524);
	s.is_empty()
}

/// Words ending -szczeć.
pub(crate) fn n3215(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7113);
					match s {
						[] => true,
						['b', 'e', 'i', 'n', 'z', s @ ..] => {
							super::push(out, 8139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 7114);
					match s {
						[] => true,
						['r', 't', 'a', s @ ..] => {
							super::push(out, 8140);
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
				['n', s @ ..] => {
					super::push(out, 7115);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 8141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7116);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5162);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			super::push(out, 5163);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 7119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5164);
			match s {
				[] => true,
				['ł', 'b', 'a', s @ ..] => {
					super::push(out, 7120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', 'z', 'r', 'h', 'c', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 5165);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ślęczeć.
pub(crate) fn n5737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5166);
	s.is_empty()
}

/// Words ending -abrzęczeć.
pub(crate) fn n3217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2938);
	s.is_empty()
}

/// Words ending -łczeć.
pub(crate) fn n1346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1259);
	match s {
		[] => true,
		['e', 'j', 'z', s @ ..] => {
			super::push(out, 2939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rzeć.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5167);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2942);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2943);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2944);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5169);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5170);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7121);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 5171);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5172);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2945);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2946);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 2947);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7124);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2948);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2949);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5173);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5174);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5175);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5176);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7126);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7127);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 8143);
											s.is_empty()
										}
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
					super::push(out, 2950);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5177);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2951);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2953);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2954);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5179);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2955);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2956);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5182);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5184);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2960);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2961);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5186);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2962);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5188);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7128);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7129);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2963);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2967);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2968);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5190);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7131);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7132);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7133);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 8144);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 7134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2969);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5193);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5194);
							s.is_empty()
						}
						['i', s @ ..] => super::t2::n5767(s, out),
						['w', s @ ..] => {
							super::push(out, 5196);
							s.is_empty()
						}
						['z', s @ ..] => super::t2::n5769(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2971);
					s.is_empty()
				}
				['u', s @ ..] => super::t2::n3253(s, out),
				['y', s @ ..] => super::t2::n3254(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t2::n1355(s, out),
		_ => false,
	}
}

/// Words ending -szeć.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['i', 'p', 'u', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['t', 's', 'u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5205);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5207);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['r', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5209);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2980);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2981);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7139);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7140);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7141);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7142);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8525);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 8526);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7143);
									s.is_empty()
								}
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

/// Words ending -rocić.
pub(crate) fn n1432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1332);
	s.is_empty()
}

/// Words ending -socić.
pub(crate) fn n1433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1333);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 3059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łocić.
pub(crate) fn n1434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1334);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5256);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3060);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 3061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pcić.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3062);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcić.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3064);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucić.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3067);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['a', 'ł', 'a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5260);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3070);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5262);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8151);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 8152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5263);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5264);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7157);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7158);
									s.is_empty()
								}
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

/// Words ending -ycić.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1343);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['c', 'z', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5265);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5266);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zcić.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5268);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ócić.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 3075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ącić.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3080);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęcić.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['i', 'w', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1352);
			match s {
				[] => true,
				['h', 'c', 'a', 'h', 'c', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1354);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5271);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5272);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7159);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7160);
									s.is_empty()
								}
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

/// Words ending -łcić.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['t', 'z', 's', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5273);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5274);
							s.is_empty()
						}
						['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 1355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ścić.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5276);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				['ł', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7162);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['e', 'z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7164);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['c', 'z', 'e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7166);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7167);
									s.is_empty()
								}
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
			super::push(out, 1358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1359);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3092);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3093);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5280);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5281);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5282);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3097);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5284);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5285);
							match s {
								[] => true,
								['z', 'a', 'z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8153);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8154);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5286);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7169);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7170);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7171);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7172);
									s.is_empty()
								}
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
			super::push(out, 1362);
			match s {
				[] => true,
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3105);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5290);
							match s {
								[] => true,
								['z', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8155);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8156);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5291);
							match s {
								[] => true,
								['h', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8157);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8158);
											s.is_empty()
										}
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

/// Words ending -zelić.
pub(crate) fn n1495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	match s {
		[] => true,
		['r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 3139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -glić.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1392);
			s.is_empty()
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilić.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1395);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -klić.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['z', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olić.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3148);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['c', 'r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3159);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3160);
					match s {
						[] => true,
						['r', 'g', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3162);
					match s {
						[] => true,
						['y', 'r', 'g', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['ę', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'g', 'z', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['f', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5322);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 7174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3169);
					match s {
						[] => true,
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 5324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3172);
					match s {
						[] => true,
						['o', 'd', s @ ..] => {
							super::push(out, 5326);
							s.is_empty()
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plić.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	s.is_empty()
}

/// Words ending -rlić.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tlić.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1414);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['w', 'ś', 'a', s @ ..] => {
							super::push(out, 5328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 1416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulić.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1417);
			match s {
				[] => true,
				['a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 1418);
			s.is_empty()
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1419);
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 1420);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1421);
			s.is_empty()
		}
		['ż', '\u{1}', s @ ..] => {
			super::push(out, 1422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylić.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5329);
							s.is_empty()
						}
						['a', 'z', s @ ..] => {
							super::push(out, 5330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'o', 'p', s @ ..] => {
							super::push(out, 5331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5333);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3182);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ślić.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5335);
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7177);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 5336);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5337);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5338);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7179);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7180);
									s.is_empty()
								}
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

/// Words ending -źlić.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 1429);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word jesienić.
pub(crate) fn n3528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3210);
	s.is_empty()
}

/// The word czerwienić.
pub(crate) fn n3529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3211);
	s.is_empty()
}

/// Words ending -lenić.
pub(crate) fn n1567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1460);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3212);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['i', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7190);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3214);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 5357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', s @ ..] => {
			super::push(out, 3215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zenić.
pub(crate) fn n1568(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 3216);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żenić.
pub(crate) fn n1569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1461);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3218);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3219);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 3220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gnić.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1462);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3223);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hnić.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	s.is_empty()
}

/// Words ending -inić.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['l', 'ś', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1467);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3230);
					match s {
						[] => true,
						['z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', '\u{1}', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jnić.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['a', 't', 's', s @ ..] => {
			super::push(out, 1469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -knić.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 436);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['ę', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5359);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5360);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 1471);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5362);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lnić.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1472);
			s.is_empty()
		}
		['e', 't', s @ ..] => {
			super::push(out, 1473);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'w', 'o', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnić.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	s.is_empty()
}

/// Words ending -nnić.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	s.is_empty()
}

/// Words ending -onić.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1476);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3241);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 5365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3242);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7192);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7193);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7194);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5366);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5367);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5368);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5369);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pnić.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	s.is_empty()
}

/// Words ending -rnić.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5373);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tnić.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1482);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 5375);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1483);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3247);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1485);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 1486);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3249);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7196);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3250);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5377);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unić.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['t', 'ł', 'o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3252);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wnić.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3254);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1489);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 5379);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ynić.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['z', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3258);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3259);
					match s {
						[] => true,
						['ę', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['ć', s @ ..] => {
							super::push(out, 5382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -znić.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1491);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1492);
			match s {
				[] => true,
				['t', 'ę', 'n', 'w', 'e', s @ ..] => {
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 5383);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łnić.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['e', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3261);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3262);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3263);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3264);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -śnić.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1494);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1495);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1496);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3268);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1498);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1499);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 3272);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źnić.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['j', 'y', 'z', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5386);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5388);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5389);
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
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1502);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5390);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'l', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5392);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5393);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3275);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1503);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 1504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żnić.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1505);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3277);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 5395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 1506);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tąpić.
pub(crate) fn n1660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1543);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5421);
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7202);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 5422);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5423);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5424);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5425);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5426);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5427);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5428);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tępić.
pub(crate) fn n1662(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3311);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3312);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łpić.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -niecierpliwić.
pub(crate) fn n6052(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7211);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 7212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -śliwić.
pub(crate) fn n3710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3371);
	s.is_empty()
}

/// Words ending -żliwić.
pub(crate) fn n3711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3372);
	s.is_empty()
}

/// Words ending -ziwić.
pub(crate) fn n1706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1584);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5461);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5462);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7213);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kwić.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3373);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -owić.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1585);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3375);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1586);
			match s {
				[] => true,
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 3378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1588);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1589);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3381);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7215);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 5465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1590);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1591);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5467);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1592);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3384);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3385);
					match s {
						[] => true,
						['j', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7217);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5469);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3387);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 5471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 3391);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7219);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7220);
									s.is_empty()
								}
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

/// Words ending -rwić.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1595);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3393);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1597);
			match s {
				[] => true,
				['n', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5472);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5473);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5474);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5475);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5476);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -twić.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1599);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 3396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1600);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5478);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1601);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5480);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5481);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5482);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5483);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1602);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3398);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5485);
							s.is_empty()
						}
						['c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3399);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 3400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uwić.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	s.is_empty()
}

/// Words ending -ywić.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['z', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1603);
			s.is_empty()
		}
		['ż', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zwić.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['r', 'e', 'i', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3403);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ówić.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3406);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3407);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3408);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3409);
					s.is_empty()
				}
				['w', 'i', s @ ..] => {
					super::push(out, 3410);
					match s {
						[] => true,
						['c', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3411);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3412);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5488);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źwić.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['e', 'z', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3414);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3415);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5490);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dzić.
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3421);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3422);
					match s {
						[] => true,
						['o', 'r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7221);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7222);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5493);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5494);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7224);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5495);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5496);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7226);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5499);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5500);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5501);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3426);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7229);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5502);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7231);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7232);
									s.is_empty()
								}
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
			super::push(out, 1608);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5503);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3429);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5505);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3431);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3432);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7233);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7234);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8162);
											s.is_empty()
										}
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
			super::push(out, 1609);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1610);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['b', s @ ..] => super::t2::n3777(s, out),
				['g', s @ ..] => super::t2::n3778(s, out),
				['h', s @ ..] => super::t2::n3779(s, out),
				['k', s @ ..] => super::t2::n3780(s, out),
				['l', s @ ..] => {
					super::push(out, 3437);
					s.is_empty()
				}
				['r', s @ ..] => super::t2::n3782(s, out),
				['w', s @ ..] => super::t2::n3783(s, out),
				['ł', s @ ..] => super::t2::n3784(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n1739(s, out),
		['u', s @ ..] => super::t2::n1740(s, out),
		['w', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['y', 'z', 'r', 'k', '\u{1}', s @ ..] => super::t2::n3796(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t2::n1742(s, out),
		['ą', s @ ..] => super::t2::n1743(s, out),
		['ę', s @ ..] => super::t2::n1744(s, out),
		['ź', s @ ..] => super::t2::n1745(s, out),
		_ => false,
	}
}

/// Words ending -czyć.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1650);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5571);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5572);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5573);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5574);
							match s {
								[] => true,
								['u', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8184);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5575);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7301);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8186);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5576);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 7303);
									s.is_empty()
								}
								['z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3476);
					match s {
						[] => true,
						['j', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 5577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5579);
							match s {
								[] => true,
								['w', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5580);
							s.is_empty()
						}
						['ż', s @ ..] => {
							super::push(out, 5581);
							match s {
								[] => true,
								['ó', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3478);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5582);
							s.is_empty()
						}
						['i', 'p', 'z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5584);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5585);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5586);
							match s {
								[] => true,
								['ł', 't', '\u{1}', s @ ..] => {
									super::push(out, 7307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3479);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5588);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5590);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3482);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5592);
							s.is_empty()
						}
						['r', 'a', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7308);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3483);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5593);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1651);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['a', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3485);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5595);
							match s {
								[] => true,
								['z', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 5596);
							match s {
								[] => true,
								['o', 'ł', 'z', 'c', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5597);
							s.is_empty()
						}
						['a', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7312);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7313);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8188);
											s.is_empty()
										}
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
					super::push(out, 3487);
					match s {
						[] => true,
						['i', 'n', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3488);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7314);
									s.is_empty()
								}
								['p', 'a', s @ ..] => {
									super::push(out, 7315);
									s.is_empty()
								}
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
			super::push(out, 1654);
			match s {
				[] => true,
				['l', s @ ..] => super::t2::n3839(s, out),
				['n', s @ ..] => super::t2::n3840(s, out),
				['w', s @ ..] => {
					super::push(out, 3491);
					match s {
						[] => true,
						['t', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5609);
							s.is_empty()
						}
						['ć', '\u{1}', s @ ..] => super::t2::n6223(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => super::t2::n3842(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t2::n1783(s, out),
		['k', s @ ..] => super::t2::n1784(s, out),
		['l', s @ ..] => super::t2::n1785(s, out),
		['m', s @ ..] => super::t2::n1786(s, out),
		['o', s @ ..] => super::t2::n1787(s, out),
		['p', s @ ..] => super::t2::n1788(s, out),
		['r', s @ ..] => super::t2::n1789(s, out),
		['t', s @ ..] => super::t2::n1790(s, out),
		['u', s @ ..] => super::t2::n1791(s, out),
		['y', s @ ..] => super::t2::n1792(s, out),
		['z', s @ ..] => super::t2::n1793(s, out),
		['ó', s @ ..] => super::t2::n1794(s, out),
		['ą', s @ ..] => super::t2::n1795(s, out),
		['ę', s @ ..] => super::t2::n1796(s, out),
		['ł', s @ ..] => super::t2::n1797(s, out),
		['ń', s @ ..] => super::t2::n1798(s, out),
		_ => false,
	}
}

/// Words ending -darzyć.
pub(crate) fn n3890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5669);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5670);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5671);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5672);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5673);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jarzyć.
pub(crate) fn n3892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3534);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5675);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5676);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5677);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -marzyć.
pub(crate) fn n3893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3535);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5678);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5679);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5680);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5681);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -parzyć.
pub(crate) fn n3894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3536);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5683);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5684);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tarzyć.
pub(crate) fn n3895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3537);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5686);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7369);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -warzyć.
pub(crate) fn n3896(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5688);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5689);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7371);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 5691);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 5692);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5693);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5694);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7373);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żarzyć.
pub(crate) fn n3897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3538);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5696);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -brzyć.
pub(crate) fn n1800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1672);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3539);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3540);
			match s {
				[] => true,
				['r', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7375);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -drzyć.
pub(crate) fn n1801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1673);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3542);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3543);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 3544);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 5699);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erzyć.
pub(crate) fn n1802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1674);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3545);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7377);
							s.is_empty()
						}
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 7378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7379);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7381);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7382);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7383);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7384);
							s.is_empty()
						}
						['z', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3546);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5703);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3548);
			match s {
				[] => true,
				['c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 5705);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7386);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7387);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hrzyć.
pub(crate) fn n1803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1675);
	match s {
		[] => true,
		['c', 'i', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5706);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5707);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word skwirzyć.
pub(crate) fn n3910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3549);
	s.is_empty()
}

/// Words ending -krzyć.
pub(crate) fn n1805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1677);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3550);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5708);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7390);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7391);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3552);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3553);
			match s {
				[] => true,
				['z', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7393);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7394);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7395);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orzyć.
pub(crate) fn n1806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1678);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3554);
			match s {
				[] => true,
				['z', 'a', 'i', 'w', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3555);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5710);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7397);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3556);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5711);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5712);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5713);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3558);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7399);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3559);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 3560);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5715);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8211);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7401);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7402);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7403);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7405);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7406);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7407);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7408);
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 7409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5717);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 7410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -przyć.
pub(crate) fn n1807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1679);
	match s {
		[] => true,
		['e', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3561);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trzyć.
pub(crate) fn n1808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1680);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3562);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7411);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7412);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 8215);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8530);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 7413);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 8216);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8217);
									s.is_empty()
								}
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
			super::push(out, 3563);
			match s {
				[] => true,
				['i', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7414);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7415);
							match s {
								[] => true,
								['p', 'a', 'n', s @ ..] => {
									super::push(out, 8218);
									s.is_empty()
								}
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
			super::push(out, 3564);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7416);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 5718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3565);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 3566);
			match s {
				[] => true,
				['j', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7418);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3567);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urzyć.
pub(crate) fn n1809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1681);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3568);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3569);
			match s {
				[] => true,
				['a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 5722);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 5723);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5724);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5725);
					s.is_empty()
				}
				['z', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 3570);
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 3571);
			s.is_empty()
		}
		['m', 'h', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5727);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 3572);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3573);
			match s {
				[] => true,
				['u', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3574);
			match s {
				[] => true,
				['n', 'a', 'w', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5730);
					s.is_empty()
				}
				['ł', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3575);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 3576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5732);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -órzyć.
pub(crate) fn n1810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1682);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3577);
			match s {
				[] => true,
				['c', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7420);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5733);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -szyć.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1683);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3578);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3580);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 5735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1685);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1686);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['b', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7422);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7423);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5736);
							match s {
								[] => true,
								['ś', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8219);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8220);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5737);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7425);
									s.is_empty()
								}
								['ś', '\u{1}', s @ ..] => {
									super::push(out, 7426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5739);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5740);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7427);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8222);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8223);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8224);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 8531);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 7428);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7429);
									s.is_empty()
								}
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
			super::push(out, 1688);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5741);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['e', 'i', 'n', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1690);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1691);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3592);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3593);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5744);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8225);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8226);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8227);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', 'u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5745);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 5747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1693);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3596);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5748);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3598);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5750);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5751);
							match s {
								[] => true,
								['u', 'b', 'a', s @ ..] => {
									super::push(out, 7430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5752);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3600);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 5754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5756);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5757);
							s.is_empty()
						}
						['y', s @ ..] => super::t2::n6393(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t2::n3967(s, out),
				['z', s @ ..] => super::t2::n3968(s, out),
				['ł', s @ ..] => super::t2::n3969(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t2::n1824(s, out),
		['y', s @ ..] => super::t2::n1825(s, out),
		['z', s @ ..] => super::t2::n1826(s, out),
		['ó', s @ ..] => super::t2::n1827(s, out),
		['ę', s @ ..] => super::t2::n1828(s, out),
		['ż', s @ ..] => super::t2::n1829(s, out),
		_ => false,
	}
}

/// Words ending -agnąć.
pub(crate) fn n1929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1798);
	match s {
		[] => true,
		['r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5818);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5819);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -egnąć.
pub(crate) fn n1930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1799);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3667);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3668);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 3669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ignąć.
pub(crate) fn n1931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1800);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3670);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3671);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 3672);
			match s {
				[] => true,
				['ź', 'd', 'u', s @ ..] => {
					super::push(out, 5822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lgnąć.
pub(crate) fn n1932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1801);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3673);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7442);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rgnąć.
pub(crate) fn n1933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1802);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5823);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3676);
			s.is_empty()
		}
		['y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugnąć.
pub(crate) fn n1934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1803);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5825);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ygnąć.
pub(crate) fn n1935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1804);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3678);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3679);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 5827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3680);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 5828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3681);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 5829);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zgnąć.
pub(crate) fn n1936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1805);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3682);
			match s {
				[] => true,
				['l', 'ś', 'b', s @ ..] => {
					super::push(out, 5830);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 7444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3683);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 7445);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3684);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ągnąć.
pub(crate) fn n1937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1806);
	match s {
		[] => true,
		['i', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5832);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5833);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7447);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5834);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęgnąć.
pub(crate) fn n1938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1807);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 3686);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3687);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5835);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hnąć.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1809);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3689);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 5837);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 5839);
							s.is_empty()
						}
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 5840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3690);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5841);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3691);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5843);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5844);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 5845);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3693);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5847);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5848);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5849);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7452);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7453);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8233);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8234);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 8235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5850);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5851);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5852);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7455);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5853);
							s.is_empty()
						}
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5855);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7457);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5856);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5857);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5858);
							s.is_empty()
						}
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5860);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7460);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5862);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 5863);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8236);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8237);
											s.is_empty()
										}
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
					super::push(out, 3697);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5864);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 7463);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['r', 'e', 'i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 7465);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8238);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8239);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 7466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8240);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7467);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7468);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7469);
									s.is_empty()
								}
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

/// Words ending -inąć.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3702);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1813);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jnąć.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 568);
	s.is_empty()
}

