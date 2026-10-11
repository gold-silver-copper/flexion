use flexion::Path;

/// Words ending -ito.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2644);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3290);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2648);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2649);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2651);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2655);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2661);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3292);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3293);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2663);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['l', 'c', 'e', 'd', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2666);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['g', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2667);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1731);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3296);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2673);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3299);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 3300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t1::n1790(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 755);
			s.is_empty()
		}
		['v', s @ ..] => super::t1::n767(s, out),
		_ => false,
	}
}

/// Words ending -lto.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['u', 's', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nto.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['r', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 761);
			match s {
				[] => true,
				['r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oto.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pto.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rto.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 770);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2694);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2695);
							match s {
								[] => true,
								['i', 'd', 'e', s @ ..] => {
									super::push(out, 3302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1762);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2697);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2698);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3303);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3304);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3305);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2701);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2703);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2705);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2707);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3309);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3310);
									s.is_empty()
								}
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

/// Words ending -sto.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2709);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2714);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3311);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2715);
							match s {
								[] => true,
								['i', s @ ..] => {
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
						['x', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 779);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1776);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2717);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 2718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tto.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2719);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3317);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3318);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3320);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uto.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 782);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['a', 's', 'n', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 785);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -æto.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erelinquo.
pub(crate) fn n3692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3576);
	s.is_empty()
}

/// Words ending -suo.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1806);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tuo.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2759);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2760);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2761);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3328);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['f', 'n', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viscor.
pub(crate) fn n2903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2807);
	s.is_empty()
}

/// Words ending -dor.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 844);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['a', 'r', 'p', 'e', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['u', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eor.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1858);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2810);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3353);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3354);
									s.is_empty()
								}
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
			super::push(out, 850);
			match s {
				[] => true,
				['i', 'f', 'n', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -for.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	s.is_empty()
}

/// Words ending -gor.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1861);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['u', 'f', 'e', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hor.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['c', 'c', 'a', 'b', 'e', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ior.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1865);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2814);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2815);
							match s {
								[] => true,
								['g', 'o', s @ ..] => {
									super::push(out, 3355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2816);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2817);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3533);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3577);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 3578);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 3579);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3580);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 3581);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 3534);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3535);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3536);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3537);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3538);
											s.is_empty()
										}
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
					super::push(out, 1867);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1868);
					match s {
						[] => true,
						['a', 'l', 'b', 'e', s @ ..] => {
							super::push(out, 2818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['r', 'a', 'l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2819);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1871);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1872);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2824);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3356);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3357);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3539);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 3540);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 3541);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3542);
											s.is_empty()
										}
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
			super::push(out, 860);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1873);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1874);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2829);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3358);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 3359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['i', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1877);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2831);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3360);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3544);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1879);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3362);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3363);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1880);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => super::t1::n880(s, out),
		_ => false,
	}
}

/// Words ending -lor.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', 'n', s @ ..] => {
									super::push(out, 3365);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1887);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2836);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 3367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2837);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3547);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 3548);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3549);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 3368);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2839);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1888);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2841);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2843);
							match s {
								[] => true,
								['m', 'e', s @ ..] => {
									super::push(out, 3369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1890);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2844);
							match s {
								[] => true,
								['r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2846);
							match s {
								[] => true,
								['p', 'e', s @ ..] => {
									super::push(out, 3371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1891);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2848);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3372);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 3373);
									s.is_empty()
								}
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

/// Words ending -mor.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 871);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nor.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 874);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['i', 'd', 'e', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1893);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2849);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2850);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3375);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2851);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2852);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1895);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1897);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3378);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3379);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1899);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1900);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2862);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 3381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1903);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3382);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1904);
					match s {
						[] => true,
						['r', 'e', 's', 'n', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -por.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ror.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1907);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3384);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['e', 'v', 'e', s @ ..] => {
							super::push(out, 2869);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2872);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 886);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['g', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sor.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tor.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 889);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2876);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3388);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2877);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3390);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3550);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3551);
											s.is_empty()
										}
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
					super::push(out, 1921);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 2878);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3391);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3392);
									s.is_empty()
								}
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
			super::push(out, 891);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1926);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2879);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2880);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3393);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 3394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1930);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3395);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2887);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3397);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3398);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1935);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3400);
									s.is_empty()
								}
								['n', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 1936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uor.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1937);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1938);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3403);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3404);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3552);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3553);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1939);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3406);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1940);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3408);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3554);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 3555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3410);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3411);
									s.is_empty()
								}
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
			super::push(out, 900);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1941);
					match s {
						[] => true,
						['m', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vor.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 902);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xor.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['i', 'n', s @ ..] => {
			super::push(out, 905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucesco.
pub(crate) fn n2100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2034);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3425);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3426);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -desco.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2035);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2036);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2037);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2959);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2960);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gesco.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3429);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3430);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2963);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2041);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lesco.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['v', 'e', s @ ..] => {
					super::push(out, 2966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2044);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2967);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2968);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2045);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2969);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mesco.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2047);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2048);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2971);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2972);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nesco.
pub(crate) fn n1016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2973);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2974);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2975);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3441);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3442);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2052);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2053);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2054);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pesco.
pub(crate) fn n1017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 993);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2056);
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2057);
			s.is_empty()
		}
		['s', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -resco.
pub(crate) fn n1018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 994);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2977);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3444);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3446);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2061);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2978);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 3448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2981);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2982);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2063);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2064);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2984);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2985);
					match s {
						[] => true,
						['f', 'e', s @ ..] => {
							super::push(out, 3450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2066);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2986);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['a', 'm', 'e', s @ ..] => {
							super::push(out, 3451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tesco.
pub(crate) fn n1019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2070);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2071);
			match s {
				[] => true,
				['b', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2993);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2072);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2995);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2996);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3452);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2998);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2073);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2075);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3001);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3455);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uesco.
pub(crate) fn n1020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['n', 'a', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['i', 'l', 'e', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3007);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3457);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3458);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vesco.
pub(crate) fn n1021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3008);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2080);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3460);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3461);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3462);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3563);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -isco.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3013);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3463);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2082);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1000);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['r', 'o', 'd', 'e', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1004);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1005);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2085);
					match s {
						[] => true,
						['l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osco.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2087);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 3017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2089);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3018);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptito.
pub(crate) fn n1790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1738);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2676);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2677);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vito.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1739);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1740);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1741);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vior.
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	s.is_empty()
}

