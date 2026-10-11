use flexion::Path;

/// Words ending -no.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 4967);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4969);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 9212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1576);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1577);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1578);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['ę', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1583);
			match s {
				[] => true,
				['a', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1584);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1585);
			match s {
				[] => true,
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4975);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 4976);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 4977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1586);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 4978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1589);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1591);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1592);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4980);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4981);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 9213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1593);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1594);
			s.is_empty()
		}
		['h', 'c', 'u', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1595);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1596);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1597);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4982);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4983);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9215);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4984);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 9216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1598);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1600);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1601);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1602);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4989);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4990);
					s.is_empty()
				}
				['o', 'k', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9219);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1605);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1607);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['ż', 'n', s @ ..] => {
					super::push(out, 4992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 1609);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wo.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1611);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 4994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 4995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4996);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12405);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 12406);
									s.is_empty()
								}
								['i', 'n', 'ś', 'e', 'z', s @ ..] => {
									super::push(out, 12407);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 12408);
									s.is_empty()
								}
								['n', 'y', s @ ..] => {
									super::push(out, 12409);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 12410);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12411);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 14279);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 14280);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 12412);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 12413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12414);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12415);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 12416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t3::n9716(s, out),
						['n', 'a', 't', 'e', s @ ..] => {
							super::push(out, 9221);
							s.is_empty()
						}
						['o', 'r', 'o', s @ ..] => {
							super::push(out, 9222);
							s.is_empty()
						}
						['u', 'r', s @ ..] => {
							super::push(out, 9223);
							s.is_empty()
						}
						['ę', s @ ..] => super::t3::n9720(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t3::n5186(s, out),
				['z', s @ ..] => super::t3::n5187(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t3::n1635(s, out),
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1616);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1617);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5000);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ło.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5002);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9236);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1621);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1622);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'i', s @ ..] => {
							super::push(out, 9238);
							s.is_empty()
						}
						['i', 'c', s @ ..] => {
							super::push(out, 9239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'i', s @ ..] => {
					super::push(out, 5004);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5005);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5006);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9240);
							s.is_empty()
						}
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 9241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1624);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['a', 'd', 'i', 'k', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 5008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1626);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1627);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1628);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['a', 'ż', s @ ..] => {
					super::push(out, 5010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['e', 'i', 'm', 'e', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5011);
					s.is_empty()
				}
				['o', 'i', 'm', s @ ..] => {
					super::push(out, 5012);
					match s {
						[] => true,
						['e', 'z', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5013);
					match s {
						[] => true,
						['i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 9243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1632);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5014);
					s.is_empty()
				}
				['e', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5015);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lop.
pub(crate) fn n1682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1660);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5043);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9257);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9259);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 9260);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 12441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'u', '\u{1}', s @ ..] => {
			super::push(out, 5045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nop.
pub(crate) fn n1684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1662);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pop.
pub(crate) fn n1685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5048);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rop.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1664);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5049);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5050);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -top.
pub(crate) fn n1687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5052);
			match s {
				[] => true,
				['m', 'o', 'r', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5053);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zop.
pub(crate) fn n1688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5055);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łop.
pub(crate) fn n1689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1667);
	match s {
		[] => true,
		['e', 'j', '\u{1}', s @ ..] => {
			super::push(out, 5056);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5057);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['-', 'a', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12442);
							s.is_empty()
						}
						['o', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rp.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9266);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 9268);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -up.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1672);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5059);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9270);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5061);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5062);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5063);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9273);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 12444);
									s.is_empty()
								}
								['z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yp.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1676);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąp.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1678);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęp.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 5068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1681);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 5069);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ojar.
pub(crate) fn n5290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5095);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 9286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kar.
pub(crate) fn n1715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1693);
	match s {
		[] => true,
		['s', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lar.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1694);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5097);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5098);
			s.is_empty()
		}
		['o', 'd', s @ ..] => {
			super::push(out, 5099);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5100);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9287);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mar.
pub(crate) fn n1717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1695);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5101);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5102);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nar.
pub(crate) fn n1718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1696);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 9288);
					s.is_empty()
				}
				['p', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5104);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 9290);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5105);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 9293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5106);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -par.
pub(crate) fn n1719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1697);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5107);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sar.
pub(crate) fn n1720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1698);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5109);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tar.
pub(crate) fn n1721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1699);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5111);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5112);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5113);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 5114);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5115);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5116);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 12454);
							s.is_empty()
						}
						['k', 'u', 'm', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uar.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1700);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5117);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5118);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5119);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5120);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bolivar.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1701);
	s.is_empty()
}

/// Words ending -war.
pub(crate) fn n1724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1702);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5121);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5122);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5123);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5124);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9302);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 12456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 9303);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 12457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9304);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zar.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1703);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5125);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9305);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 12459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'n', s @ ..] => {
			super::push(out, 5126);
			match s {
				[] => true,
				['a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 9306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5127);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5128);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 9308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 5129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żar.
pub(crate) fn n1726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1704);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5130);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -br.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['m', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 1706);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1707);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', 'z', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dr.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5133);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5134);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5135);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9311);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 9312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5136);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12460);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5137);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9313);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5138);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5139);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9316);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9317);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 12462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5140);
					match s {
						[] => true,
						['t', 'u', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 9319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5141);
					s.is_empty()
				}
				['ą', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5143);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5144);
					match s {
						[] => true,
						['m', 'i', 'f', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9320);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5145);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5146);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5147);
					match s {
						[] => true,
						['a', 'l', 'e', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9322);
							s.is_empty()
						}
						['e', 'u', 'l', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5148);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5149);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9325);
							s.is_empty()
						}
						['t', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 9326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5151);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 9327);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5152);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 9329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5153);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9330);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9331);
							s.is_empty()
						}
						['s', 't', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9332);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 9333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5154);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 12463);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 12464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5155);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9334);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12465);
									s.is_empty()
								}
								['f', 'a', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 12466);
									s.is_empty()
								}
								['l', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 9335);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 12468);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 14316);
											match s {
												[] => true,
												['i', 'c', '\u{1}', s @ ..] => {
													super::push(out, 15132);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 12469);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 14317);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 14318);
											match s {
												[] => true,
												['h', '\u{1}', s @ ..] => {
													super::push(out, 15133);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 12470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9336);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5156);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5157);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5158);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9339);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9340);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'g', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1716);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5159);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5160);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 5161);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							super::push(out, 9342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5162);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5163);
					s.is_empty()
				}
				['r', s @ ..] => super::t3::n5361(s, out),
				['s', s @ ..] => super::t3::n5362(s, out),
				['u', s @ ..] => super::t3::n5363(s, out),
				['y', s @ ..] => super::t3::n5364(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t3::n1739(s, out),
		['h', s @ ..] => super::t3::n1740(s, out),
		['i', s @ ..] => super::t3::n1741(s, out),
		['j', s @ ..] => super::t3::n1742(s, out),
		['k', s @ ..] => super::t3::n1743(s, out),
		['l', s @ ..] => super::t3::n1744(s, out),
		['m', s @ ..] => super::t3::n1745(s, out),
		['n', s @ ..] => super::t3::n1746(s, out),
		['p', s @ ..] => super::t3::n1747(s, out),
		['s', s @ ..] => super::t3::n1748(s, out),
		['t', s @ ..] => super::t3::n1749(s, out),
		['u', s @ ..] => super::t3::n1750(s, out),
		['v', s @ ..] => super::t3::n1751(s, out),
		['w', s @ ..] => super::t3::n1752(s, out),
		['x', s @ ..] => super::t3::n1753(s, out),
		['y', s @ ..] => super::t3::n1754(s, out),
		['z', s @ ..] => super::t3::n1755(s, out),
		['ż', s @ ..] => super::t3::n1756(s, out),
		_ => false,
	}
}

/// Words ending -fr.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1735);
			s.is_empty()
		}
		['y', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 1736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gr.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1737);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1738);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hr.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1741);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1742);
			match s {
				[] => true,
				['m', 'i', s @ ..] => {
					super::push(out, 5275);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5276);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1743);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 5278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5280);
					s.is_empty()
				}
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5281);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5282);
					match s {
						[] => true,
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1746);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1747);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'w', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1748);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 9521);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5284);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1750);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5286);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5287);
					s.is_empty()
				}
				['ś', '\u{1}', s @ ..] => {
					super::push(out, 5288);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jr.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kr.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['a', 'm', 'i', 'f', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nr.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	s.is_empty()
}

/// Words ending -or.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1754);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5289);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9524);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9525);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9526);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 12563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5290);
					match s {
						[] => true,
						['m', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9528);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9529);
							match s {
								[] => true,
								['l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 12564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9530);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', 'i', 'w', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 5294);
					s.is_empty()
				}
				['z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5296);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5297);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5298);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1758);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5300);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5301);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 5302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12565);
									s.is_empty()
								}
								['n', 'z', '\u{1}', s @ ..] => {
									super::push(out, 12566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9533);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5303);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9535);
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9536);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9537);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9538);
							s.is_empty()
						}
						['ę', 'z', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5304);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5305);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5306);
					match s {
						[] => true,
						['a', 'k', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 9540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5307);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9541);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 12568);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 12569);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 12570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ą', s @ ..] => {
							super::push(out, 9542);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 12571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5308);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 9543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5309);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12572);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 12573);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14351);
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
									super::push(out, 12574);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 14352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12575);
									s.is_empty()
								}
								['ę', s @ ..] => {
									super::push(out, 12576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9545);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1762);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5310);
					s.is_empty()
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1763);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5312);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5313);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 9547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5315);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5316);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5317);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9548);
							s.is_empty()
						}
						['a', 'r', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5319);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1766);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1767);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5321);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 9551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t3::n1790(s, out),
		['t', s @ ..] => super::t4::n1791(s, out),
		['u', s @ ..] => {
			super::push(out, 1770);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => super::t4::n1793(s, out),
		['z', s @ ..] => super::t4::n1794(s, out),
		_ => false,
	}
}

/// The word szypr.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	s.is_empty()
}

/// Words ending -rr.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	s.is_empty()
}

/// Words ending -tr.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['e', 't', 'y', s @ ..] => {
					super::push(out, 5346);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5347);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12642);
									s.is_empty()
								}
								['i', 'z', 'd', 'ę', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12643);
									s.is_empty()
								}
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
			super::push(out, 1774);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9596);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9597);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12644);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 14407);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 12645);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 14408);
											match s {
												[] => true,
												['k', '\u{1}', s @ ..] => {
													super::push(out, 15168);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'a', 'n', s @ ..] => {
									super::push(out, 12646);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'k', s @ ..] => {
									super::push(out, 12647);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 14410);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 9598);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 12648);
									s.is_empty()
								}
								['t', 'n', s @ ..] => {
									super::push(out, 12649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 5348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9599);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5349);
					s.is_empty()
				}
				['m', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5350);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 5351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', 'ą', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5353);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 5356);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5357);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5360);
					s.is_empty()
				}
				['z', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'j', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1783);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['j', 'u', '\u{1}', s @ ..] => {
					super::push(out, 5362);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5363);
					s.is_empty()
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1785);
			match s {
				[] => true,
				['c', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5366);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9603);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12650);
									s.is_empty()
								}
								['n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5368);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 9604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'r', s @ ..] => {
			super::push(out, 1787);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1788);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5370);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5371);
					s.is_empty()
				}
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5372);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5373);
					match s {
						[] => true,
						['z', 'c', 'z', 's', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1790);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5375);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5376);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5378);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5379);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5381);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 9607);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 9608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5382);
					match s {
						[] => true,
						['z', 's', 'a', 'r', s @ ..] => {
							super::push(out, 9609);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 12654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 5383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1793);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wr.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	s.is_empty()
}

/// Words ending -yr.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['d', 'o', 'w', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1794);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1795);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1797);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9610);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1798);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1799);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5387);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5388);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 9612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', '\u{1}', s @ ..] => {
			super::push(out, 1800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ór.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1801);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					super::push(out, 5390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1802);
			match s {
				[] => true,
				['o', 'w', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9613);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 12655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 9614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1803);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5391);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 5392);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 5393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1804);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1805);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5395);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5396);
					match s {
						[] => true,
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9615);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'c', 'ś', 'e', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 12656);
									s.is_empty()
								}
								['k', 's', 'u', 'ł', '\u{1}', s @ ..] => {
									super::push(out, 12657);
									s.is_empty()
								}
								['t', 'o', 'ł', 'z', '\u{1}', s @ ..] => {
									super::push(out, 12658);
									s.is_empty()
								}
								['ł', 'a', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 9616);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9617);
							s.is_empty()
						}
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5397);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5398);
					match s {
						[] => true,
						['e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9619);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5399);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1807);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					super::push(out, 5401);
					match s {
						[] => true,
						['o', 'ł', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1808);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5402);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['k', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5404);
					s.is_empty()
				}
				['w', 'o', s @ ..] => {
					super::push(out, 5405);
					match s {
						[] => true,
						['b', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 9623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5406);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5407);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5408);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5409);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 9624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1811);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5410);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5411);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9626);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 9627);
							match s {
								[] => true,
								['a', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5412);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word samopas.
pub(crate) fn n10169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9644);
	s.is_empty()
}

/// The word skotopas.
pub(crate) fn n10170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9645);
	s.is_empty()
}

/// The word buspas.
pub(crate) fn n5665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5450);
	s.is_empty()
}

/// Words ending -upas.
pub(crate) fn n5666(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9646);
			s.is_empty()
		}
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ras.
pub(crate) fn n1846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1824);
	match s {
		[] => true,
		['t', 'l', s @ ..] => {
			super::push(out, 5451);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 9648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5452);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 9649);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 9650);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sas.
pub(crate) fn n1847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1825);
	s.is_empty()
}

/// Words ending -tas.
pub(crate) fn n1848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1826);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5453);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5454);
			s.is_empty()
		}
		['p', 'u', 'ł', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5455);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5456);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5457);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 9651);
					s.is_empty()
				}
				['m', 's', '\u{1}', s @ ..] => {
					super::push(out, 9652);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -was.
pub(crate) fn n1849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1827);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 5458);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5459);
			match s {
				[] => true,
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 9653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zas.
pub(crate) fn n1850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1828);
	s.is_empty()
}

/// Words ending -łas.
pub(crate) fn n1851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1829);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 5460);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 9654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bs.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5461);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word trads.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	s.is_empty()
}

/// Words ending -es.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5463);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9655);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5466);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['n', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 5469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 't', 'a', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1836);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9658);
							s.is_empty()
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', 't', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 5471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5472);
					match s {
						[] => true,
						['g', 'y', 'r', 'o', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'i', 'b', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 9661);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9662);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12665);
									s.is_empty()
								}
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
			super::push(out, 1839);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1840);
			match s {
				[] => true,
				['g', 'e', s @ ..] => {
					super::push(out, 5474);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1841);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9663);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9664);
							match s {
								[] => true,
								['k', 'o', 'h', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 12666);
									s.is_empty()
								}
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
			super::push(out, 1843);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1844);
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5477);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hs.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 1846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5479);
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 5480);
					s.is_empty()
				}
				['r', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5483);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5484);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5485);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5486);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1850);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5488);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5489);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1852);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5491);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5492);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5493);
					match s {
						[] => true,
						['i', 'm', 'e', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 9669);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 9670);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12668);
									s.is_empty()
								}
								['e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 9671);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 12670);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 12671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9672);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9673);
							match s {
								[] => true,
								['o', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 12672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1853);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 5496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -js.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['e', 'p', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ks.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5498);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5499);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5500);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5502);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							super::push(out, 9676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12673);
									s.is_empty()
								}
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
			super::push(out, 1857);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5503);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9677);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 9678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5504);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 5505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5506);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5507);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5508);
					match s {
						[] => true,
						['y', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5509);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5510);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9682);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 12675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 9684);
							s.is_empty()
						}
						['f', 's', '\u{1}', s @ ..] => {
							super::push(out, 9685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5511);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12676);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 12677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 5513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5514);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5517);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5518);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9689);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1862);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5520);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5521);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ls.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['j', 'c', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 5523);
					s.is_empty()
				}
				['k', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5524);
					s.is_empty()
				}
				['m', 'r', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ms.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1864);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5528);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12678);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 12679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5529);
					match s {
						[] => true,
						['e', 'r', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 5530);
					s.is_empty()
				}
				['i', 'l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9692);
							s.is_empty()
						}
						['z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5531);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9694);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'e', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9695);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5532);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9697);
							match s {
								[] => true,
								['b', 'e', 'd', 'r', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9698);
							s.is_empty()
						}
						['u', 'b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5533);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5534);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9700);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 14411);
											s.is_empty()
										}
										['n', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 14412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 9701);
							match s {
								[] => true,
								['d', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 12682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5535);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5536);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9703);
							match s {
								[] => true,
								['e', 'l', 'e', 'c', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 9704);
							s.is_empty()
						}
						['e', 'w', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9705);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 9706);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9707);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5537);
					match s {
						[] => true,
						['e', 'n', 'e', s @ ..] => {
							super::push(out, 9709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5538);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5539);
					s.is_empty()
				}
				['w', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5540);
					s.is_empty()
				}
				['ż', 'i', 'l', 'y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1868);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5542);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5543);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9710);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9711);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5544);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9713);
							s.is_empty()
						}
						['s', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5546);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['f', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5548);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5549);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1869);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1870);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1871);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 5550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1872);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1873);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5552);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5553);
					match s {
						[] => true,
						['l', 'o', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'ł', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5555);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1876);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5557);
					match s {
						[] => true,
						['k', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5559);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5560);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9717);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5561);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5562);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1878);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 5563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5564);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5565);
					s.is_empty()
				}
				['i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5566);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5567);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5568);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 9723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1880);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5569);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5572);
					s.is_empty()
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9725);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5573);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9727);
							s.is_empty()
						}
						['z', 'e', s @ ..] => {
							super::push(out, 9728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9730);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', 'ę', 'z', '\u{1}', s @ ..] => {
									super::push(out, 12687);
									s.is_empty()
								}
								['t', 'o', 'ł', 'z', '\u{1}', s @ ..] => {
									super::push(out, 12688);
									s.is_empty()
								}
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

/// Words ending -ps.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1883);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 5576);
					s.is_empty()
				}
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 5577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1884);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5580);
					s.is_empty()
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5582);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5583);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5584);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['d', 'a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5585);
					s.is_empty()
				}
				['k', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5586);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5587);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5588);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5589);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9734);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12689);
									s.is_empty()
								}
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
			super::push(out, 1891);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5590);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 5591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ss.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5592);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ts.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	s.is_empty()
}

/// Words ending -us.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5594);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5595);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9735);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9736);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9737);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5596);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5597);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12690);
									s.is_empty()
								}
								['m', 'o', '\u{1}', s @ ..] => {
									super::push(out, 12691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 9739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5598);
					match s {
						[] => true,
						['a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 9740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5599);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9741);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5600);
					match s {
						[] => true,
						['i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9743);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 9744);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9745);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12692);
									s.is_empty()
								}
								['y', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 12693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9746);
							s.is_empty()
						}
						['t', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5601);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1897);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5602);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['l', 'k', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 5605);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5606);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5607);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5608);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 9751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5609);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12694);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 12695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 9752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5610);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5611);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'ł', 's', '\u{1}', s @ ..] => {
					super::push(out, 5612);
					s.is_empty()
				}
				['y', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1902);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1903);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5614);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9754);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9755);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5615);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9757);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9758);
							s.is_empty()
						}
						['s', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9759);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9760);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 12696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5616);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5617);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5618);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9762);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 9763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5619);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9764);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9765);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 12697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'r', 't', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 5620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['y', 't', 's', s @ ..] => {
					super::push(out, 5621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5623);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5624);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5625);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5626);
					s.is_empty()
				}
				['r', 'a', 'n', 'e', 't', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5627);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5628);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5629);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5630);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5631);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9768);
							s.is_empty()
						}
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9770);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9771);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 12698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5632);
					s.is_empty()
				}
				['y', 'd', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['a', 'l', 'h', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n1933(s, out),
		['s', s @ ..] => super::t4::n1934(s, out),
		['t', s @ ..] => super::t4::n1935(s, out),
		['v', 'i', 't', 'a', s @ ..] => super::t4::n1936(s, out),
		['w', s @ ..] => super::t4::n1937(s, out),
		['z', s @ ..] => super::t4::n1938(s, out),
		['ł', s @ ..] => super::t4::n1939(s, out),
		_ => false,
	}
}

/// Words ending -ws.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	s.is_empty()
}

/// Words ending -ys.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1914);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['n', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1917);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5662);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 9789);
							s.is_empty()
						}
						['k', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5663);
					match s {
						[] => true,
						['y', 't', '\u{1}', s @ ..] => {
							super::push(out, 9791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5665);
					s.is_empty()
				}
				['ł', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 5666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąs.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 1920);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5667);
					match s {
						[] => true,
						['ł', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1921);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęs.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -riat.
pub(crate) fn n5910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5684);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['t', 'e', 'l', s @ ..] => {
					super::push(out, 12705);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 12706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 9799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wiat.
pub(crate) fn n5911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5685);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 9800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12707);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9801);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 9802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12709);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12710);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12711);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 12712);
					match s {
						[] => true,
						['ż', 'e', 'i', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kat.
pub(crate) fn n1955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5686);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5687);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5688);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9803);
					s.is_empty()
				}
				['l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5689);
			match s {
				[] => true,
				['w', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lat.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['b', 'o', s @ ..] => {
			super::push(out, 5692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'b', s @ ..] => {
			super::push(out, 5693);
			match s {
				[] => true,
				['u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 9808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5694);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9809);
					s.is_empty()
				}
				['ł', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5695);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mat.
pub(crate) fn n1957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5696);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5697);
			match s {
				[] => true,
				['r', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5698);
			s.is_empty()
		}
		['g', 'o', s @ ..] => {
			super::push(out, 5699);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5700);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5701);
			match s {
				[] => true,
				['l', 'k', 'o', s @ ..] => {
					super::push(out, 9813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5702);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5703);
			match s {
				[] => true,
				['o', 'f', 'e', s @ ..] => {
					super::push(out, 9814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5704);
			s.is_empty()
		}
		['z', 's', s @ ..] => {
			super::push(out, 5705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nat.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5706);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12713);
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
					super::push(out, 9816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9818);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9819);
					s.is_empty()
				}
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9820);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 9821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5708);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['j', 's', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12715);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5710);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 9823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'c', s @ ..] => {
			super::push(out, 5711);
			match s {
				[] => true,
				['s', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pat.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5712);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9825);
					s.is_empty()
				}
				['k', 's', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rat.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9827);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9829);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9831);
					s.is_empty()
				}
				['u', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9832);
					s.is_empty()
				}
				['z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9833);
					s.is_empty()
				}
				['ł', 'ó', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['d', 'e', 'f', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9835);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9836);
					s.is_empty()
				}
				['n', 'e', 'g', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9837);
					s.is_empty()
				}
				['p', 's', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9838);
					s.is_empty()
				}
				['t', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 5718);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5719);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9840);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 9841);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5720);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5721);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5722);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12719);
							s.is_empty()
						}
						['u', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 12720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'p', 'r', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -asat.
pub(crate) fn n5951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5724);
	s.is_empty()
}

/// Words ending -esat.
pub(crate) fn n5952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5725);
	s.is_empty()
}

/// Words ending -ssat.
pub(crate) fn n5953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5726);
	s.is_empty()
}

/// Words ending -tat.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5727);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9843);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'e', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5729);
			match s {
				[] => true,
				['e', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5730);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5731);
			match s {
				[] => true,
				['p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vat.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wat.
pub(crate) fn n1964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1938);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5733);
			s.is_empty()
		}
		['a', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5734);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5735);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12722);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5736);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5737);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 9848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zat.
pub(crate) fn n1965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1939);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łat.
pub(crate) fn n1966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1940);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5740);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5741);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dt.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	s.is_empty()
}

/// Words ending -et.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5744);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1942);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5746);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9852);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5747);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1943);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5751);
					match s {
						[] => true,
						['z', 'r', 'g', 'o', s @ ..] => {
							super::push(out, 9855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5752);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9856);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9857);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 12724);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 9858);
							match s {
								[] => true,
								['r', 'k', s @ ..] => {
									super::push(out, 12726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 5753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1946);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5754);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 9859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5755);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5756);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5757);
					match s {
						[] => true,
						['i', 'f', s @ ..] => {
							super::push(out, 9860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5758);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 9861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5759);
					match s {
						[] => true,
						['e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5760);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5762);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5764);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 9865);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 12728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5765);
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5766);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5767);
					match s {
						[] => true,
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 9867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5768);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1950);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5769);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5771);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5772);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1952);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1953);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5775);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1954);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1955);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5777);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9874);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9875);
							match s {
								[] => true,
								['s', 'o', 'l', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 12729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ft.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	s.is_empty()
}

/// Words ending -ht.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5778);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5779);
					match s {
						[] => true,
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5780);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5781);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5782);
					match s {
						[] => true,
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5783);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5784);
					match s {
						[] => true,
						['l', 'y', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 9879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1958);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5786);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 5787);
					match s {
						[] => true,
						['p', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5789);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 5791);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1962);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5792);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5793);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9882);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', s @ ..] => {
					super::push(out, 5794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5795);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5796);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5797);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9885);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 12730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5798);
					match s {
						[] => true,
						['s', 'r', 'e', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1964);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9887);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9889);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 9890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', s @ ..] => {
					super::push(out, 5799);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5800);
					match s {
						[] => true,
						['e', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5801);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 9892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5802);
					s.is_empty()
				}
				['u', 't', s @ ..] => {
					super::push(out, 5803);
					s.is_empty()
				}
				['y', 't', s @ ..] => {
					super::push(out, 5804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1966);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1967);
			match s {
				[] => true,
				['r', 'f', 'e', 'p', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jt.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1969);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1970);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5808);
					s.is_empty()
				}
				['z', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 1971);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 5810);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kt.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1972);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5811);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5812);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5813);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 9894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5814);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5815);
					match s {
						[] => true,
						['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'n', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9897);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 9898);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1975);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lt.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5818);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5819);
					match s {
						[] => true,
						['s', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1977);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5820);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', 'r', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 5822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['k', 'e', 'd', '\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 1979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nt.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9902);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9903);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9904);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9905);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12732);
									s.is_empty()
								}
								['u', 't', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'y', s @ ..] => {
					super::push(out, 5826);
					match s {
						[] => true,
						['s', 'k', 'o', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9907);
							s.is_empty()
						}
						['i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9908);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5828);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9910);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5829);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9911);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9912);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 9913);
							s.is_empty()
						}
						['w', 'o', s @ ..] => {
							super::push(out, 9914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9915);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9916);
							match s {
								[] => true,
								['s', 'y', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9917);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9918);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 14416);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 14417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 9919);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12737);
									match s {
										[] => true,
										['b', 'u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 14418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 9920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5831);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9921);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 9922);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9923);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9924);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 12738);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 12739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 9925);
							match s {
								[] => true,
								['m', 'i', '\u{1}', s @ ..] => {
									super::push(out, 12740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9926);
							match s {
								[] => true,
								['m', 'y', 't', s @ ..] => {
									super::push(out, 12741);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 14419);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'r', s @ ..] => {
							super::push(out, 9927);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5832);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12743);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 12744);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 14420);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5833);
					match s {
						[] => true,
						['e', 'w', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9930);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t4::n6065(s, out),
				['s', s @ ..] => {
					super::push(out, 5835);
					match s {
						[] => true,
						['e', s @ ..] => super::t4::n10480(s, out),
						['r', s @ ..] => {
							super::push(out, 9942);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t4::n6067(s, out),
				['u', 'n', s @ ..] => {
					super::push(out, 5837);
					match s {
						[] => true,
						['y', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => super::t4::n10485(s, out),
						_ => false,
					}
				}
				['w', s @ ..] => super::t4::n6069(s, out),
				['ż', s @ ..] => super::t4::n6070(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n2009(s, out),
		['i', s @ ..] => super::t4::n2010(s, out),
		['o', s @ ..] => super::t4::n2011(s, out),
		['u', s @ ..] => super::t4::n2012(s, out),
		['y', s @ ..] => super::t4::n2013(s, out),
		_ => false,
	}
}

/// Words ending -ot.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1986);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5869);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5870);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 5871);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9997);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 9998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5872);
					match s {
						[] => true,
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 9999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1987);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5874);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5875);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5876);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 10000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5877);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5878);
					match s {
						[] => true,
						['r', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5879);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5880);
					match s {
						[] => true,
						['z', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5882);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10003);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 10004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1989);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5884);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10005);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5885);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 10007);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10008);
							match s {
								[] => true,
								['l', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'z', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 5886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5888);
					s.is_empty()
				}
				['e', 'z', s @ ..] => {
					super::push(out, 5889);
					match s {
						[] => true,
						['r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5890);
					match s {
						[] => true,
						['i', 'k', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 10011);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10012);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 12775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5891);
					match s {
						[] => true,
						['u', 'f', s @ ..] => {
							super::push(out, 10013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 5892);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5893);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12776);
									s.is_empty()
								}
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
			super::push(out, 1992);
			match s {
				[] => true,
				['a', 'z', s @ ..] => {
					super::push(out, 5894);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5895);
					match s {
						[] => true,
						['b', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10015);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10016);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10017);
							s.is_empty()
						}
						['z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5896);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12777);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5897);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10020);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 10021);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10022);
							s.is_empty()
						}
						['n', 'ł', s @ ..] => {
							super::push(out, 10023);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['a', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5898);
					s.is_empty()
				}
				['o', 'm', s @ ..] => {
					super::push(out, 5899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5900);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 10026);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 10027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'b', '-', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1995);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1996);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5902);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5903);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5904);
					s.is_empty()
				}
				['k', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 5905);
					s.is_empty()
				}
				['m', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5906);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5907);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 10029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5908);
					match s {
						[] => true,
						['s', 'k', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 10030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5909);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10031);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 10032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 5910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1998);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5911);
					s.is_empty()
				}
				['c', 'n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5912);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5913);
					s.is_empty()
				}
				['y', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 5914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5915);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5916);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10033);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10034);
							s.is_empty()
						}
						['z', 's', 'e', 'z', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5917);
					match s {
						[] => true,
						['i', 'k', 'n', 'o', 'd', '\u{1}', s @ ..] => super::t4::n10577(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => super::t4::n2028(s, out),
		_ => false,
	}
}

/// Words ending -pt.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 5921);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5922);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2003);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 5924);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5926);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5927);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10040);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 10041);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10042);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10043);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5929);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5930);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['b', 'm', 'e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5931);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5932);
					match s {
						[] => true,
						['s', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'i', s @ ..] => {
					super::push(out, 5933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2005);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5934);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5935);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5936);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10048);
							s.is_empty()
						}
						['s', 'k', s @ ..] => {
							super::push(out, 10049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5937);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12780);
									s.is_empty()
								}
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
			super::push(out, 2007);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5938);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5939);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5940);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5941);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 10052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['l', 'h', 'c', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2009);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5943);
					s.is_empty()
				}
				['d', 'b', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5945);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5946);
					match s {
						[] => true,
						['r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['e', 'g', 'r', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['e', 't', 'l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5948);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10055);
							match s {
								[] => true,
								['i', 'b', 'u', '\u{1}', s @ ..] => {
									super::push(out, 12781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12782);
									s.is_empty()
								}
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
			super::push(out, 2011);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5949);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 10057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5950);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5951);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10058);
							match s {
								[] => true,
								['u', 'ł', '\u{1}', s @ ..] => {
									super::push(out, 12783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5952);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5953);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5954);
					match s {
						[] => true,
						['z', 'w', s @ ..] => {
							super::push(out, 10060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['r', 'h', 'c', 'i', s @ ..] => {
					super::push(out, 5957);
					match s {
						[] => true,
						['p', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5959);
					match s {
						[] => true,
						['h', 'c', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2016);
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					super::push(out, 5961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5962);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5963);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10065);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 10066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5965);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10067);
							match s {
								[] => true,
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 5966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2018);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2019);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['b', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5968);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5970);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5971);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5972);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10069);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12785);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 12786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ń', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5974);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5975);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 10070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5976);
					match s {
						[] => true,
						['ł', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5977);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5978);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2024);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'ł', 'a', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 5981);
					s.is_empty()
				}
				['i', 'l', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10072);
							s.is_empty()
						}
						['i', 'p', 'r', 'a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 10073);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5984);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 5985);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 5987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5988);
					s.is_empty()
				}
				['y', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10076);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yt.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['b', 'z', s @ ..] => {
			super::push(out, 2029);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5989);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5990);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2031);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2032);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5992);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10078);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10079);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 12787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5993);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10080);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 10081);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 10082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'o', s @ ..] => {
					super::push(out, 5994);
					s.is_empty()
				}
				['k', 's', s @ ..] => {
					super::push(out, 5995);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5996);
					match s {
						[] => true,
						['w', 'a', s @ ..] => {
							super::push(out, 10083);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 12788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 's', s @ ..] => {
					super::push(out, 5997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2033);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5998);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5999);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6001);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 10084);
							match s {
								[] => true,
								['d', 'r', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6003);
					match s {
						[] => true,
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', s @ ..] => {
					super::push(out, 6004);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 10086);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 12790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zt.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6007);
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ót.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 2039);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6009);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąt.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6010);
					s.is_empty()
				}
				['j', 'ó', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6011);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10088);
							match s {
								[] => true,
								['c', 'ę', 'i', 'p', s @ ..] => {
									super::push(out, 12791);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14428);
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
								['o', 'w', 'z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 12792);
									s.is_empty()
								}
								['t', 's', 'o', '\u{1}', s @ ..] => {
									super::push(out, 12793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'a', 'n', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'd', 'e', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 12794);
									s.is_empty()
								}
								['t', 'ę', 'i', 'w', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12795);
									s.is_empty()
								}
								['y', 'z', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 12796);
									s.is_empty()
								}
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

/// Words ending -ęt.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2041);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6012);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10089);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10090);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12797);
									s.is_empty()
								}
								['o', 'p', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6014);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10092);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 12799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10093);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łt.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 321);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6015);
					match s {
						[] => true,
						['y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['s', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10096);
							s.is_empty()
						}
						['z', 's', 'k', 'o', s @ ..] => {
							super::push(out, 10097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńt.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2045);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word modrzew.
pub(crate) fn n16229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15170);
	s.is_empty()
}

/// Words ending -ałodrzew.
pub(crate) fn n16230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15171);
	match s {
		[] => true,
		['i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 15423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -arw.
pub(crate) fn n2104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2071);
	match s {
		[] => true,
		['k', '\u{1}', s @ ..] => {
			super::push(out, 6044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erw.
pub(crate) fn n2105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2072);
	match s {
		[] => true,
		['i', 'c', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 6045);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 6046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urw.
pub(crate) fn n2106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10111);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 10112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yw.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['r', 'g', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2074);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2075);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 6047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6048);
					match s {
						[] => true,
						['n', 'i', 'm', 'e', s @ ..] => {
							super::push(out, 10113);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 12802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6049);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6050);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 10114);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6051);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 2076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ów.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['d', 'z', 'i', 'w', 'g', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2077);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10116);
							s.is_empty()
						}
						['u', 'm', 'd', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2080);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6054);
					s.is_empty()
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6055);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6056);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 10118);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 12804);
									match s {
										[] => true,
										['ł', 'z', '\u{1}', s @ ..] => {
											super::push(out, 14433);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 12805);
									s.is_empty()
								}
								['s', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10119);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 10120);
							match s {
								[] => true,
								['a', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 10121);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10122);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6058);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6059);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 10124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łw.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['ó', s @ ..] => {
			super::push(out, 2082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nny.
pub(crate) fn n2160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2126);
	match s {
		[] => true,
		['e', 'i', 'm', 'e', s @ ..] => {
			super::push(out, 6089);
			match s {
				[] => true,
				['z', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 10137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ony.
pub(crate) fn n2161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2127);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6090);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6091);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6092);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10138);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12810);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10139);
					match s {
						[] => true,
						['i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 12812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6093);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6094);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6095);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10141);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 10142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6096);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 6097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pny.
pub(crate) fn n2162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	s.is_empty()
}

/// Words ending -rny.
pub(crate) fn n2163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	match s {
		[] => true,
		['a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 6098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tny.
pub(crate) fn n2164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2130);
	s.is_empty()
}

/// Words ending -wny.
pub(crate) fn n2165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2131);
	s.is_empty()
}

/// Words ending -yny.
pub(crate) fn n2166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2132);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 6099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zny.
pub(crate) fn n2167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2133);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6100);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 10143);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'r', '\u{1}', s @ ..] => {
					super::push(out, 10144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6101);
			s.is_empty()
		}
		['s', 'e', 'i', 'p', 's', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oy.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -py.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 347);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2136);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 6105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ry.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['k', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6106);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10145);
							s.is_empty()
						}
						['-', 'y', 'r', 'a', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 6108);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 6109);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6110);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2140);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6114);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 6115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2142);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2143);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6116);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2144);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 6118);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 6119);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 2145);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sy.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2146);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2147);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2148);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2149);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2150);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6123);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 6124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ty.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2152);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6125);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6126);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6127);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10148);
							s.is_empty()
						}
						['o', 'ż', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'r', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6129);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6130);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6131);
					match s {
						[] => true,
						['o', 'i', s @ ..] => {
							super::push(out, 10151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2154);
			s.is_empty()
		}
		['h', 'c', 'a', 'z', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2155);
			s.is_empty()
		}
		['i', 'm', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2156);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2157);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6133);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2158);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6135);
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 10152);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 10153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 6136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'n', 'e', 'k', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10154);
							s.is_empty()
						}
						['w', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2160);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2161);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 2162);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					super::push(out, 6138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 2163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wy.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2164);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 6139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2166);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6140);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10156);
							s.is_empty()
						}
						['o', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6141);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 10158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', s @ ..] => {
					super::push(out, 6142);
					match s {
						[] => true,
						['p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6143);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10160);
							match s {
								[] => true,
								['e', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 12814);
									s.is_empty()
								}
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
			super::push(out, 2167);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6144);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zy.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2168);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 6145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2169);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6146);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12815);
									s.is_empty()
								}
								['o', 'd', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', 'e', 'l', 'd', 'a', s @ ..] => {
							super::push(out, 10161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2170);
			match s {
				[] => true,
				['w', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2171);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6148);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6149);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6151);
					match s {
						[] => true,
						['r', 'e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['t', 'j', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ły.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6156);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6157);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2177);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2178);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2179);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2180);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2182);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ży.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['y', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2184);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 2185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cz.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['a', s @ ..] => super::t4::n2227(s, out),
		['e', s @ ..] => {
			super::push(out, 2194);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6188);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6190);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12905);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10248);
							s.is_empty()
						}
						['u', 'w', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6192);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6193);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2195);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2196);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10251);
							s.is_empty()
						}
						['a', 'r', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6197);
					match s {
						[] => true,
						['d', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6198);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10255);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 10256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 6199);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6201);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10258);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10259);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10260);
							match s {
								[] => true,
								['d', 'ę', 'w', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 12908);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 12909);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 12910);
									match s {
										[] => true,
										['d', 'n', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 14477);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12911);
									s.is_empty()
								}
								['m', 'y', 'z', 'r', 'g', 'l', 'e', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12912);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 12913);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'z', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 14478);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['f', 'a', '\u{1}', s @ ..] => {
													super::push(out, 15186);
													s.is_empty()
												}
												['i', 'r', 'a', 'k', '\u{1}', s @ ..] => {
													super::push(out, 15187);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'e', 'i', 'k', 'n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12914);
									s.is_empty()
								}
								['z', 'a', 'o', '\u{1}', s @ ..] => {
									super::push(out, 12915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6202);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2197);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6203);
					s.is_empty()
				}
				['o', 'd', 's', 'k', 'l', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2198);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2199);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6206);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12916);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 12917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2200);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6208);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6209);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6210);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 10262);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6211);
					match s {
						[] => true,
						['o', 'ł', 'a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10264);
							s.is_empty()
						}
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 6212);
					match s {
						[] => true,
						['o', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 10266);
							s.is_empty()
						}
						['p', 'y', 'z', 'r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6213);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 10268);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6214);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6215);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12918);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12919);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 12920);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 12921);
									s.is_empty()
								}
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
			super::push(out, 2202);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6216);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6217);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6218);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t4::n2237(s, out),
		['z', s @ ..] => super::t4::n2238(s, out),
		['ę', s @ ..] => super::t4::n2239(s, out),
		['ń', 'a', 'r', 'a', 'm', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uoz.
pub(crate) fn n2268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	match s {
		[] => true,
		['t', 'r', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 6237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łoz.
pub(crate) fn n2269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2235);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 6238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rz.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6239);
					match s {
						[] => true,
						['a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6240);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6241);
					match s {
						[] => true,
						['n', 'e', 'l', s @ ..] => {
							super::push(out, 10290);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6242);
					match s {
						[] => true,
						['a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 10292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6243);
					match s {
						[] => true,
						['a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 10293);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6244);
					match s {
						[] => true,
						['b', 'ę', s @ ..] => {
							super::push(out, 10295);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10296);
							match s {
								[] => true,
								['a', 'm', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 12947);
									s.is_empty()
								}
								['e', 'i', s @ ..] => {
									super::push(out, 12948);
									match s {
										[] => true,
										['b', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 14483);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'e', 'z', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 12949);
									s.is_empty()
								}
								['n', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 12950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 10297);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 10298);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 14484);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 15190);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 14485);
											match s {
												[] => true,
												['o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 15191);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'ę', s @ ..] => {
							super::push(out, 10299);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10300);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 12952);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 14486);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 12953);
									match s {
										[] => true,
										['i', 'l', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 14487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'a', s @ ..] => {
					super::push(out, 6245);
					match s {
						[] => true,
						['w', 'm', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 10301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6246);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12954);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 12955);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 14488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'ó', s @ ..] => {
							super::push(out, 10302);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 12956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10303);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10304);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12957);
									match s {
										[] => true,
										['p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14489);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 10305);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 12958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ż', 'o', s @ ..] => {
							super::push(out, 10306);
							match s {
								[] => true,
								['r', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n6498(s, out),
				['m', 'a', s @ ..] => {
					super::push(out, 6248);
					match s {
						[] => true,
						['ł', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6249);
					match s {
						[] => true,
						['i', s @ ..] => super::t4::n10882(s, out),
						['o', s @ ..] => super::t4::n10883(s, out),
						['y', s @ ..] => {
							super::push(out, 10320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t4::n6501(s, out),
				['t', s @ ..] => super::t4::n6502(s, out),
				['w', s @ ..] => super::t4::n6503(s, out),
				['z', s @ ..] => {
					super::push(out, 6253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n2271(s, out),
		['h', s @ ..] => super::t4::n2272(s, out),
		['o', s @ ..] => super::t4::n2273(s, out),
		['p', s @ ..] => super::t4::n2274(s, out),
		['t', s @ ..] => super::t4::n2275(s, out),
		['u', s @ ..] => super::t4::n2276(s, out),
		['ó', s @ ..] => super::t4::n2277(s, out),
		_ => false,
	}
}

/// Words ending -sz.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6270);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10361);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 10362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6272);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 6273);
					match s {
						[] => true,
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6274);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10365);
							s.is_empty()
						}
						['u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6275);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10367);
							s.is_empty()
						}
						['r', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10368);
							s.is_empty()
						}
						['z', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6276);
					match s {
						[] => true,
						['o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 10370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6277);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6278);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6279);
					match s {
						[] => true,
						['n', 'o', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10371);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10372);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 13006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6280);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10373);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10374);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10375);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10376);
							match s {
								[] => true,
								['z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 6281);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13008);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 13009);
									s.is_empty()
								}
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
			super::push(out, 2245);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6282);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6283);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10377);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10379);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 10380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6285);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2246);
			match s {
				[] => true,
				['c', 'o', 'ł', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6287);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6288);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10383);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6289);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6290);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6291);
					match s {
						[] => true,
						['r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6292);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6293);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13010);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 13011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6294);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2247);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2248);
			match s {
				[] => true,
				['y', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2249);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6296);
					match s {
						[] => true,
						['a', s @ ..] => super::t4::n10961(s, out),
						['e', 'd', '\u{1}', s @ ..] => super::t4::n10962(s, out),
						['i', 'b', s @ ..] => super::t4::n10963(s, out),
						['m', s @ ..] => super::t4::n10964(s, out),
						['o', s @ ..] => super::t4::n10965(s, out),
						['u', s @ ..] => super::t4::n10966(s, out),
						['ę', s @ ..] => super::t4::n10967(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t4::n6549(s, out),
				['g', s @ ..] => super::t4::n6550(s, out),
				['k', s @ ..] => super::t4::n6551(s, out),
				['l', s @ ..] => super::t4::n6552(s, out),
				['m', s @ ..] => super::t4::n6553(s, out),
				['n', s @ ..] => super::t4::n6554(s, out),
				['p', s @ ..] => super::t4::n6555(s, out),
				['r', s @ ..] => super::t4::n6556(s, out),
				['t', s @ ..] => super::t4::n6557(s, out),
				['w', s @ ..] => super::t4::n6558(s, out),
				['ł', 'p', 'y', 'w', '\u{1}', s @ ..] => super::t4::n6559(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n2284(s, out),
		['u', s @ ..] => super::t4::n2285(s, out),
		['y', s @ ..] => super::t4::n2286(s, out),
		['ó', s @ ..] => super::t4::n2287(s, out),
		['ł', s @ ..] => super::t4::n2288(s, out),
		['ż', s @ ..] => super::t4::n2289(s, out),
		_ => false,
	}
}

/// Words ending -uz.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2257);
			match s {
				[] => true,
				['a', 'k', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6335);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6336);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6337);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 10453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2259);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 2260);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'r', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2262);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 6342);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 6343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['h', 'c', 'a', 'l', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 6344);
					s.is_empty()
				}
				['k', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6345);
					s.is_empty()
				}
				['m', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6346);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6347);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yz.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['r', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2266);
			s.is_empty()
		}
		['m', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2267);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 6349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zz.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -óz.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 2271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąz.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2272);
			s.is_empty()
		}
		['w', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęz.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 2274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ść.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2298);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 10466);
							s.is_empty()
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2299);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6372);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 6373);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', 'ę', 'r', '\u{1}', s @ ..] => {
									super::push(out, 13036);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 13037);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 13038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6375);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2300);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6376);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6377);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n2336(s, out),
		['r', s @ ..] => {
			super::push(out, 2302);
			match s {
				[] => true,
				['a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 6390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', 'e', 'z', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2303);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2304);
			match s {
				[] => true,
				['p', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6391);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 2305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ił.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['s', 'ć', 'ę', 'i', 'w', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2332);
			s.is_empty()
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 2333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oł.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['d', 'j', 'a', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2334);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2335);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6426);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 6427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2336);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13121);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 13122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10530);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2337);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6428);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10532);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 10533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6429);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6430);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10534);
							match s {
								[] => true,
								['h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 13124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6431);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 10535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6432);
					match s {
						[] => true,
						['y', s @ ..] => {
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 13125);
									s.is_empty()
								}
								['ż', s @ ..] => {
									super::push(out, 13126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6433);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10536);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13127);
									s.is_empty()
								}
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
			super::push(out, 2338);
			match s {
				[] => true,
				['n', 'a', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2339);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2341);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2342);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10537);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sł.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['y', 'm', 'a', s @ ..] => {
			super::push(out, 2343);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uł.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2344);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6438);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 10539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'a', s @ ..] => {
			super::push(out, 2345);
			match s {
				[] => true,
				['r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2346);
			match s {
				[] => true,
				['o', 'ł', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ół.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2347);
			match s {
				[] => true,
				['i', 'p', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2348);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6442);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2349);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6444);
					match s {
						[] => true,
						['ś', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6445);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2350);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10541);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10542);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13128);
									s.is_empty()
								}
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
			super::push(out, 2351);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 2352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6447);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6448);
					s.is_empty()
				}
				['o', 'm', 'ż', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6449);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ipień.
pub(crate) fn n11148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10554);
	match s {
		[] => true,
		['l', '\u{1}', s @ ..] => {
			super::push(out, 13136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opień.
pub(crate) fn n11149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10555);
	match s {
		[] => true,
		['r', 'k', s @ ..] => {
			super::push(out, 13137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rpień.
pub(crate) fn n11150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10556);
	s.is_empty()
}

/// Words ending -upień.
pub(crate) fn n11151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10557);
	match s {
		[] => true,
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 13138);
			s.is_empty()
		}
		['ł', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14594);
					s.is_empty()
				}
				['r', 'a', 'k', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 14595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ępień.
pub(crate) fn n11152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10558);
	match s {
		[] => true,
		['r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 13139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sień.
pub(crate) fn n6719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6459);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10559);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10560);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 13140);
					s.is_empty()
				}
				['z', 'r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10561);
			match s {
				[] => true,
				['ł', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10562);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 10563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wień.
pub(crate) fn n6720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6460);
	match s {
		[] => true,
		['e', 'z', 'r', '\u{1}', s @ ..] => {
			super::push(out, 10564);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10565);
			match s {
				[] => true,
				['e', 'z', 'c', 'd', s @ ..] => {
					super::push(out, 13143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'r', '\u{1}', s @ ..] => {
			super::push(out, 10566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zień.
pub(crate) fn n6721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6461);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13144);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 13145);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13146);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14596);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 14597);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13147);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 10568);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13149);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leń.
pub(crate) fn n2391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2355);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6462);
			s.is_empty()
		}
		['a', 'k', s @ ..] => {
			super::push(out, 6463);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6464);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10569);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 13150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'g', s @ ..] => {
			super::push(out, 6465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6466);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 6467);
			match s {
				[] => true,
				['c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10571);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reń.
pub(crate) fn n2392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2356);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 6468);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 10572);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 10573);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seń.
pub(crate) fn n2393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2357);
	s.is_empty()
}

/// Words ending -teń.
pub(crate) fn n2394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2358);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 6469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zeń.
pub(crate) fn n2395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2359);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6470);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10574);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10575);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13151);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 13152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 10577);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 13154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 10578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6471);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6472);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 10579);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 13155);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 14599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10580);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14600);
									match s {
										[] => true,
										['z', 'r', 'p', 'd', s @ ..] => {
											super::push(out, 15228);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14601);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14602);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 15229);
											s.is_empty()
										}
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
					super::push(out, 10581);
					s.is_empty()
				}
				['ó', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 10582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6473);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 10583);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10584);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10585);
					match s {
						[] => true,
						['e', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 13156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żeń.
pub(crate) fn n2396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2360);
	match s {
		[] => true,
		['r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 6474);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ciemń.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	s.is_empty()
}

/// Words ending -oń.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2361);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6475);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6476);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 10586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6477);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2363);
			match s {
				[] => true,
				['-', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6479);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6481);
					s.is_empty()
				}
				['ł', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2365);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2366);
			match s {
				[] => true,
				['c', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2367);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6484);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 10588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6485);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'y', 'ł', '\u{1}', s @ ..] => {
			super::push(out, 2368);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6487);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 2370);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2371);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 2372);
			match s {
				[] => true,
				['b', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 6489);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6490);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 6491);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 2373);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pń.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2374);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rń.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['e', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uń.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2376);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2377);
			s.is_empty()
		}
		['z', 'e', 'z', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yń.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['d', 'u', s @ ..] => {
			super::push(out, 2379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -śń.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2380);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6493);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źń.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2383);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dopowiedź.
pub(crate) fn n16293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15230);
	s.is_empty()
}

/// The word niedźwiedź.
pub(crate) fn n16294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15231);
	s.is_empty()
}

/// Words ending -yż.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2469);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2471);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2472);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 6569);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10619);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 10620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -óż.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2473);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6571);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6572);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6573);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ząż.
pub(crate) fn n2518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2479);
	s.is_empty()
}

/// Words ending -ęż.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2480);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 2481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łż.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -błuda.
pub(crate) fn n2602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2558);
	s.is_empty()
}

/// Words ending -wda.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	s.is_empty()
}

/// Words ending -yda.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6645);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 10653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', s @ ..] => {
			super::push(out, 2559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zda.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2561);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęda.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['z', 'r', 'z', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ografia.
pub(crate) fn n14048(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 15232);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 15233);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 15234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'd', s @ ..] => {
			super::push(out, 14606);
			s.is_empty()
		}
		['k', 's', s @ ..] => {
			super::push(out, 14607);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14608);
			s.is_empty()
		}
		['m', 'l', s @ ..] => {
			super::push(out, 14609);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15235);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15236);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 't', s @ ..] => {
			super::push(out, 14610);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14611);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15238);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15239);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lfia.
pub(crate) fn n2671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2622);
	s.is_empty()
}

/// Words ending -ofia.
pub(crate) fn n2672(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 't', 's', s @ ..] => {
			super::push(out, 6698);
			s.is_empty()
		}
		['z', 'o', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10678);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gia.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2623);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2624);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2625);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2626);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2627);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2628);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10680);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', s @ ..] => {
									super::push(out, 13183);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 13184);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 14612);
											s.is_empty()
										}
										['g', '\u{1}', s @ ..] => {
											super::push(out, 14613);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 13185);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 13186);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 14614);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 15240);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['j', 's', s @ ..] => {
									super::push(out, 13187);
									s.is_empty()
								}
								['k', 'y', 's', s @ ..] => {
									super::push(out, 13188);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 13189);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 14615);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'y', 't', 'e', s @ ..] => {
									super::push(out, 13190);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', 'm', s @ ..] => {
											super::push(out, 14616);
											s.is_empty()
										}
										['i', 'm', 'r', s @ ..] => {
											super::push(out, 14617);
											s.is_empty()
										}
										['o', 'f', s @ ..] => {
											super::push(out, 14618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 13191);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 14619);
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
												['m', s @ ..] => {
													super::push(out, 15241);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 15242);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'm', s @ ..] => {
											super::push(out, 14620);
											s.is_empty()
										}
										['n', 'a', s @ ..] => {
											super::push(out, 14621);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 14622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 10681);
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
					super::push(out, 6700);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6701);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hia.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'o', 't', s @ ..] => {
					super::push(out, 6703);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 10682);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -makia.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	s.is_empty()
}

/// Words ending -lia.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6704);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 6705);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6706);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'm', s @ ..] => {
					super::push(out, 6707);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6708);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6709);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							super::push(out, 10685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6710);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6712);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6713);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'n', s @ ..] => {
					super::push(out, 6714);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2630);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6717);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6718);
					match s {
						[] => true,
						['o', 'r', 'p', s @ ..] => {
							super::push(out, 10689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6719);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6720);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6722);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6723);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6724);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10690);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2633);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6727);
					s.is_empty()
				}
				['s', 'n', 'e', 't', 'u', '\u{1}', s @ ..] => {
					super::push(out, 6728);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mia.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2635);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6730);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6731);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2636);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6733);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 10692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6734);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6735);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 10693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'o', 't', 'e', s @ ..] => {
			super::push(out, 2637);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 2638);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2639);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6738);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 13193);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 13194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 10694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6739);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 6740);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10696);
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
				['a', s @ ..] => {
					super::push(out, 6741);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 10697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2640);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2641);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nia.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2643);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6744);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6746);
					match s {
						[] => true,
						['o', 'z', s @ ..] => {
							super::push(out, 10701);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6747);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6748);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 6749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2645);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6750);
					match s {
						[] => true,
						['r', 'ś', s @ ..] => {
							super::push(out, 10704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13195);
									s.is_empty()
								}
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
			super::push(out, 2646);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6751);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10705);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', 'o', 'l', s @ ..] => {
					super::push(out, 6752);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6753);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 6754);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2647);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2648);
			match s {
				[] => true,
				['c', 'u', 'k', 'r', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 6756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2649);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6757);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 's', s @ ..] => {
					super::push(out, 6758);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6759);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6760);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2650);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2652);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 6762);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'u', s @ ..] => {
			super::push(out, 2653);
			match s {
				[] => true,
				['l', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2654);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6764);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6765);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6766);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10709);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10710);
							match s {
								[] => true,
								['z', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 13196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6767);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6768);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10711);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13197);
									s.is_empty()
								}
								['h', 's', 'y', s @ ..] => {
									super::push(out, 13198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'h', 'c', 'n', 'y', 's', 'a', s @ ..] => {
					super::push(out, 6769);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2655);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10712);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 13199);
									match s {
										[] => true,
										['r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 14623);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'c', 'e', s @ ..] => {
							super::push(out, 10713);
							s.is_empty()
						}
						['k', 'u', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['u', 'm', s @ ..] => {
					super::push(out, 6771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6772);
					s.is_empty()
				}
				['b', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6773);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 6774);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6775);
					match s {
						[] => true,
						['c', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10715);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 10716);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 13200);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 13201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6776);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10717);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6777);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6778);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10719);
							s.is_empty()
						}
						['o', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6779);
					s.is_empty()
				}
				['ł', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10721);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 13202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', s @ ..] => {
							super::push(out, 10722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2659);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6781);
					s.is_empty()
				}
				['d', 'o', 'r', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 6782);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6783);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6784);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6785);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 6786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2660);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 2661);
			s.is_empty()
		}
		['ź', s @ ..] => {
			super::push(out, 2662);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 2663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pia.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 503);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6787);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10723);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									match s {
										[] => true,
										['g', 'o', s @ ..] => {
											super::push(out, 14624);
											s.is_empty()
										}
										['i', 'r', 'k', s @ ..] => {
											super::push(out, 14625);
											s.is_empty()
										}
										['k', 'a', s @ ..] => {
											super::push(out, 14626);
											s.is_empty()
										}
										['n', 'u', s @ ..] => {
											super::push(out, 14627);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 14628);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 's', s @ ..] => {
			super::push(out, 2664);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2665);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10725);
							s.is_empty()
						}
						['s', 'o', 'r', 'k', s @ ..] => {
							super::push(out, 10726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6788);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6789);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6790);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2666);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2667);
			match s {
				[] => true,
				['ą', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', 'o', s @ ..] => {
			super::push(out, 2668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ria.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2669);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6792);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6793);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13203);
									s.is_empty()
								}
								['m', 'i', s @ ..] => {
									super::push(out, 13204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6794);
					match s {
						[] => true,
						['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10730);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2670);
			match s {
				[] => true,
				['d', 'u', s @ ..] => {
					super::push(out, 6796);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 6797);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 10732);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6798);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 6799);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6800);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10734);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 13205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'i', 'k', 'r', s @ ..] => {
							super::push(out, 10735);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10736);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 10737);
							s.is_empty()
						}
						['n', 'a', 'd', s @ ..] => {
							super::push(out, 10738);
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 10739);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'u', s @ ..] => {
					super::push(out, 6801);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 6802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2671);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6803);
					s.is_empty()
				}
				['k', 'o', s @ ..] => {
					super::push(out, 6804);
					s.is_empty()
				}
				['l', 'f', s @ ..] => {
					super::push(out, 6805);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6806);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6807);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2672);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10741);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10743);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 13206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 's', s @ ..] => {
							super::push(out, 10744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6809);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6810);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', s @ ..] => {
			super::push(out, 2673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sia.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 505);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2674);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10745);
							s.is_empty()
						}
						['j', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 10746);
							s.is_empty()
						}
						['m', 'o', 'r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2675);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2676);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 6813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6814);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 6815);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 10749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tia.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 506);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'y', s @ ..] => {
							super::push(out, 10750);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 13207);
									s.is_empty()
								}
								['r', 'u', s @ ..] => {
									super::push(out, 13208);
									s.is_empty()
								}
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
			super::push(out, 2679);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6816);
					s.is_empty()
				}
				['y', 'r', 'a', s @ ..] => {
					super::push(out, 6817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wia.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6818);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6819);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zia.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2682);
			match s {
				[] => true,
				['i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6820);
					s.is_empty()
				}
				['m', 'u', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6821);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6822);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 6823);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2683);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 6824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cja.
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 511);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2689);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6826);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6827);
					match s {
						[] => true,
						['a', 'r', 'g', 'e', 'd', 'o', s @ ..] => {
							super::push(out, 10752);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10753);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 13209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6828);
					s.is_empty()
				}
				['g', s @ ..] => super::t4::n7118(s, out),
				['i', 'd', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10757);
							s.is_empty()
						}
						['e', 'm', 'r', s @ ..] => {
							super::push(out, 10758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => super::t4::n7120(s, out),
				['k', s @ ..] => super::t4::n7121(s, out),
				['l', s @ ..] => super::t4::n7122(s, out),
				['m', s @ ..] => super::t4::n7123(s, out),
				['n', s @ ..] => super::t4::n7124(s, out),
				['p', s @ ..] => super::t4::n7125(s, out),
				['r', s @ ..] => super::t4::n7126(s, out),
				['s', s @ ..] => super::t4::n7127(s, out),
				['t', s @ ..] => super::t4::n7128(s, out),
				['u', s @ ..] => super::t4::n7129(s, out),
				['w', s @ ..] => super::t4::n7130(s, out),
				['z', s @ ..] => super::t4::n7131(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2690);
			s.is_empty()
		}
		['i', s @ ..] => super::t4::n2747(s, out),
		['k', s @ ..] => super::t4::n2748(s, out),
		['n', s @ ..] => super::t4::n2749(s, out),
		['o', 'm', 'o', 'k', s @ ..] => super::t4::n2750(s, out),
		['p', s @ ..] => super::t4::n2751(s, out),
		['r', s @ ..] => super::t4::n2752(s, out),
		['u', s @ ..] => super::t4::n2753(s, out),
		['y', s @ ..] => super::t4::n2754(s, out),
		_ => false,
	}
}

/// Words ending -cyja.
pub(crate) fn n2781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2717);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6873);
			match s {
				[] => true,
				['b', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10825);
					s.is_empty()
				}
				['l', 'u', 'b', 'a', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10826);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13294);
							s.is_empty()
						}
						['t', 's', 'i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10827);
					match s {
						[] => true,
						['n', 'e', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13296);
							s.is_empty()
						}
						['s', 'e', 't', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'i', 'n', 'a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 10828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6874);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6875);
			match s {
				[] => true,
				['n', 'o', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 6876);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 10830);
					s.is_empty()
				}
				['e', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10831);
					s.is_empty()
				}
				['u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6877);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 's', 'e', 'r', 'o', 'h', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13298);
							s.is_empty()
						}
						['n', 'y', 't', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13299);
							s.is_empty()
						}
						['r', 'o', 'h', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13300);
							s.is_empty()
						}
						['t', 'r', 'e', 'w', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13301);
							s.is_empty()
						}
						['w', 'n', 'i', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'l', 'o', 's', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -maryja.
pub(crate) fn n11474(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 13303);
			s.is_empty()
		}
		['r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 13304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oryja.
pub(crate) fn n7190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6880);
	match s {
		[] => true,
		['s', 'e', 'c', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10833);
			s.is_empty()
		}
		['t', 's', 'i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 10834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -syja.
pub(crate) fn n2784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2720);
	match s {
		[] => true,
		['e', 'c', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6881);
			s.is_empty()
		}
		['s', 'i', 'm', 'm', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 6882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zyja.
pub(crate) fn n2786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2722);
	match s {
		[] => true,
		['a', 'k', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zja.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6884);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 10835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6886);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2723);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2724);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6888);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6889);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', s @ ..] => {
									super::push(out, 13305);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6890);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10836);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10837);
							match s {
								[] => true,
								['k', 'o', s @ ..] => {
									super::push(out, 13307);
									s.is_empty()
								}
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
			super::push(out, 2725);
			match s {
				[] => true,
				['f', 'y', s @ ..] => {
					super::push(out, 6891);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6892);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6893);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6894);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -anka.
pub(crate) fn n2820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2753);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 6921);
			match s {
				[] => true,
				['ż', s @ ..] => {
					super::push(out, 10850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6922);
			match s {
				[] => true,
				['m', 'o', s @ ..] => {
					super::push(out, 10851);
					match s {
						[] => true,
						['ł', s @ ..] => {
							super::push(out, 13311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10852);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'o', 'r', s @ ..] => {
					super::push(out, 10853);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6923);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 10854);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onka.
pub(crate) fn n2822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2755);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6924);
			match s {
				[] => true,
				['r', 'a', 'k', 's', '-', s @ ..] => {
					super::push(out, 10855);
					match s {
						[] => true,
						['a', 'k', 'n', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 13315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'm', s @ ..] => {
			super::push(out, 6925);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6926);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'h', s @ ..] => {
			super::push(out, 6927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ynka.
pub(crate) fn n2823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2756);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 6928);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 10858);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'k', s @ ..] => {
			super::push(out, 6929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oka.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2757);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2758);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 2759);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 10859);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 13317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 10860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rka.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['a', 'i', 's', s @ ..] => {
			super::push(out, 2760);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					super::push(out, 6931);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', s @ ..] => {
					super::push(out, 6932);
					s.is_empty()
				}
				['t', 'y', s @ ..] => {
					super::push(out, 6933);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 6934);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 10862);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 13318);
									s.is_empty()
								}
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
			super::push(out, 2761);
			match s {
				[] => true,
				['m', 'o', s @ ..] => {
					super::push(out, 6935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 't', s @ ..] => {
			super::push(out, 2762);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6936);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ska.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2763);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 6937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'z', s @ ..] => {
			super::push(out, 2764);
			match s {
				[] => true,
				['d', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2765);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'z', 'c', 'z', 's', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2766);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2767);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6940);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 10864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2768);
			s.is_empty()
		}
		['w', 'a', 'k', 'e', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2769);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2770);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 6941);
					match s {
						[] => true,
						['r', 'g', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 2771);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6942);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tka.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2772);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 6943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2773);
			match s {
				[] => true,
				['a', 'r', 'o', 'd', s @ ..] => {
					super::push(out, 6944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2774);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 6945);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10866);
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
					super::push(out, 6946);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2775);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6948);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uka.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2776);
			match s {
				[] => true,
				['n', 'o', s @ ..] => {
					super::push(out, 6949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wka.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['a', 'z', 's', s @ ..] => {
			super::push(out, 2777);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2778);
			s.is_empty()
		}
		['ó', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6950);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6951);
					match s {
						[] => true,
						['o', 'r', 'd', s @ ..] => {
							super::push(out, 10869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'a', s @ ..] => {
					super::push(out, 6952);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 10870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'y', s @ ..] => {
					super::push(out, 6953);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 10871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', s @ ..] => {
					super::push(out, 6954);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 10872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yka.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2779);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6955);
					match s {
						[] => true,
						['ł', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2780);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6957);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10874);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'a', s @ ..] => {
			super::push(out, 2781);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2782);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10876);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13319);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['j', s @ ..] => {
											super::push(out, 14663);
											s.is_empty()
										}
										['t', 'u', s @ ..] => {
											super::push(out, 14664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6958);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 10878);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'l', 'k', s @ ..] => {
					super::push(out, 6959);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6960);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13321);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 13322);
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
						['a', 'm', 'e', s @ ..] => {
							super::push(out, 10879);
							s.is_empty()
						}
						['e', 'm', s @ ..] => {
							super::push(out, 10880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'n', s @ ..] => {
					super::push(out, 6961);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 10881);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10882);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13323);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14665);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14666);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 14667);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'z', s @ ..] => {
									super::push(out, 13324);
									s.is_empty()
								}
								['w', 'a', 'l', s @ ..] => {
									super::push(out, 13325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'g', s @ ..] => {
											super::push(out, 14668);
											s.is_empty()
										}
										['e', 't', s @ ..] => {
											super::push(out, 14669);
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 15266);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'l', s @ ..] => {
											super::push(out, 14670);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									super::push(out, 13326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6962);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10883);
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							super::push(out, 10884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2783);
			s.is_empty()
		}
		['z', 'i', 'f', 'o', 'r', 'g', s @ ..] => {
			super::push(out, 2784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zka.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6964);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 10885);
							match s {
								[] => true,
								['ł', s @ ..] => {
									super::push(out, 13327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 10886);
							match s {
								[] => true,
								['a', 'i', 'b', s @ ..] => {
									super::push(out, 13328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6965);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 10887);
							match s {
								[] => true,
								['s', 'u', s @ ..] => {
									super::push(out, 13329);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14671);
											s.is_empty()
										}
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
					super::push(out, 6966);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10888);
							match s {
								[] => true,
								['b', 'u', s @ ..] => {
									super::push(out, 13330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 't', 'e', s @ ..] => {
					super::push(out, 6967);
					match s {
						[] => true,
						['m', 't', s @ ..] => {
							super::push(out, 10889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 6968);
					match s {
						[] => true,
						['ż', s @ ..] => {
							super::push(out, 10890);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 13331);
									s.is_empty()
								}
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
			super::push(out, 2785);
			match s {
				[] => true,
				['i', 'k', s @ ..] => {
					super::push(out, 6969);
					match s {
						[] => true,
						['i', 'z', 'd', 'ę', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10892);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							super::push(out, 10893);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13332);
									s.is_empty()
								}
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

/// Words ending -óka.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['ł', s @ ..] => {
			super::push(out, 2786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąka.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	s.is_empty()
}

/// Words ending -ćka.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2787);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęka.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2789);
			match s {
				[] => true,
				['z', 'd', 'o', s @ ..] => {
					super::push(out, 6970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2790);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łka.
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 552);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2792);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					super::push(out, 6972);
					s.is_empty()
				}
				['k', 'y', 'm', s @ ..] => {
					super::push(out, 6973);
					s.is_empty()
				}
				['p', 'y', s @ ..] => {
					super::push(out, 6974);
					match s {
						[] => true,
						['z', 'c', 'r', 'u', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 10894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', s @ ..] => {
					super::push(out, 6975);
					match s {
						[] => true,
						['m', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2793);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6976);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2794);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2795);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńka.
pub(crate) fn n555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2796);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					super::push(out, 6978);
					match s {
						[] => true,
						['a', 'h', 'c', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -śka.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2798);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źka.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	s.is_empty()
}

/// Words ending -żka.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 556);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6979);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'l', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6980);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word studencina.
pub(crate) fn n7374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7051);
	s.is_empty()
}

/// Words ending -ucina.
pub(crate) fn n7375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7052);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 10911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęcina.
pub(crate) fn n7376(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'n', s @ ..] => {
			super::push(out, 10912);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gościna.
pub(crate) fn n7377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7053);
	s.is_empty()
}

/// Words ending -eina.
pub(crate) fn n2966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2885);
	s.is_empty()
}

/// Words ending -fina.
pub(crate) fn n2967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2886);
	s.is_empty()
}

/// Words ending -ragina.
pub(crate) fn n2968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2887);
	s.is_empty()
}

/// Words ending -lina.
pub(crate) fn n2969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2888);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'g', s @ ..] => {
					super::push(out, 10914);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10915);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['z', 'a', s @ ..] => {
					super::push(out, 10917);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 10918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7054);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7055);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 10919);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'c', s @ ..] => {
			super::push(out, 7056);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10921);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mina.
pub(crate) fn n2970(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7057);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13334);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 10922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nina.
pub(crate) fn n2971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10923);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10924);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 10925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7059);
			match s {
				[] => true,
				['l', 'e', 'j', s @ ..] => {
					super::push(out, 10926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7060);
			s.is_empty()
		}
		['o', 't', s @ ..] => {
			super::push(out, 7061);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7062);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 7063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oina.
pub(crate) fn n2972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	s.is_empty()
}

/// Words ending -pina.
pub(crate) fn n2973(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			super::push(out, 7064);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7065);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sina.
pub(crate) fn n2974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	s.is_empty()
}

/// Words ending -wina.
pub(crate) fn n2975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2892);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 10927);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 10928);
					s.is_empty()
				}
				['ł', 'o', s @ ..] => {
					super::push(out, 10929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zina.
pub(crate) fn n2976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2893);
	match s {
		[] => true,
		['d', 'u', s @ ..] => {
			super::push(out, 7067);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 10930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7068);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 10931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jna.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2894);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lna.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	s.is_empty()
}

/// Words ending -mna.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2896);
			match s {
				[] => true,
				['d', 'z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2897);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7070);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nna.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2898);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10932);
							s.is_empty()
						}
						['ć', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 10933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 7072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2899);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7073);
					match s {
						[] => true,
						['m', 'z', '\u{1}', s @ ..] => {
							super::push(out, 10934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2900);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7074);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ona.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2901);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2902);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2903);
			match s {
				[] => true,
				['ó', 'r', 'w', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'w', s @ ..] => {
			super::push(out, 2904);
			match s {
				[] => true,
				['a', 'ł', 's', 'o', 'g', 'o', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2907);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 2908);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rna.
pub(crate) fn n608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2909);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 7082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10936);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 10937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2910);
			s.is_empty()
		}
		['u', 'ż', s @ ..] => {
			super::push(out, 2911);
			match s {
				[] => true,
				['y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sna.
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2912);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 7084);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tna.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 607);
	s.is_empty()
}

/// Words ending -una.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2913);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2914);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wna.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2915);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7087);
					s.is_empty()
				}
				['z', 'r', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 2916);
			s.is_empty()
		}
		_ => false,
	}
}

