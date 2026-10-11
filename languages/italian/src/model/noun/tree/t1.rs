use flexion::Path;

/// Words ending -ola (3 syllables).
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'f', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 's', 'a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'm', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 2623);
							match s {
								[] => true,
								['l', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3130);
									s.is_empty()
								}
								['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3131);
									s.is_empty()
								}
								['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'n', 'e', s @ ..] => {
			match s {
				[] => true,
				['p', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['v', 'o', 'b', 'r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'c', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['s', 'o', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2625);
							s.is_empty()
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2627);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3133);
									s.is_empty()
								}
								['r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', 'v', 'a', 'z', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'n', 'o', 'g', 'r', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ula (3 syllables).
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2628);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2629);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'i', 'd', 'n', 'e', 'f', 'f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['l', 'l', 'e', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ma (3 syllables).
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2633);
									s.is_empty()
								}
								['l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2635);
									s.is_empty()
								}
								['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
								['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'n', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2638);
									s.is_empty()
								}
								['t', 't', 'i', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'g', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['k', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						['s', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2640);
									s.is_empty()
								}
								['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2641);
									s.is_empty()
								}
								['n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['d', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['s', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['o', 'r', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'e', 'o', 'l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 1751);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2643);
									s.is_empty()
								}
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
			super::push(out, 364);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['l', 'c', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							match s {
								[] => true,
								['m', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2644);
									s.is_empty()
								}
								['t', 'n', 'a', 's', 'a', 'm', 'm', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1752);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3135);
											s.is_empty()
										}
										['o', 'c', 'l', 'i', 't', 'e', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3136);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'b', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1754);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1755);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['a', 'i', 'r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3137);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
													super::push(out, 3321);
													s.is_empty()
												}
												['r', 'o', '\u{1}', s @ ..] => {
													super::push(out, 3322);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t1::n2232(s, out),
						['r', s @ ..] => super::t1::n2233(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n1124(s, out),
				['i', 'z', 'z', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				['o', 's', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n439(s, out),
		['r', s @ ..] => super::t1::n440(s, out),
		['s', s @ ..] => super::t1::n441(s, out),
		['u', s @ ..] => super::t1::n442(s, out),
		['è', 'n', 'i', 'c', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => super::t1::n443(s, out),
		['ì', 'r', 'c', 's', 'i', 'a', '\u{1}', s @ ..] => super::t1::n444(s, out),
		_ => false,
	}
}

/// The word malaguegna (3 syllables).
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1796);
	s.is_empty()
}

/// Words ending -ogna (3 syllables).
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'r', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1797);
			s.is_empty()
		}
		['t', 'o', 'c', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ina (3 syllables).
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', 'c', 'o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 934);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['a', 'r', 't', 'i', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1799);
					s.is_empty()
				}
				['e', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1800);
					s.is_empty()
				}
				['i', 'f', 'f', 'o', s @ ..] => {
					match s {
						[] => true,
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2670);
							s.is_empty()
						}
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 't', 'o', 'r', 'p', 'o', 'c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 937);
			s.is_empty()
		}
		['f', 'r', 'o', 'm', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 938);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 939);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2674);
							s.is_empty()
						}
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['n', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'm', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				['o', 'r', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['u', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'i', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 2676);
							s.is_empty()
						}
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2677);
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
				['a', 'm', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['a', 'i', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'e', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['r', 'a', 'l', 'i', 't', 'e', 'm', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['s', 'e', 'p', 'o', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 1812);
					match s {
						[] => true,
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', 'o', 'c', '-', 'a', 'z', 'z', 'a', 'g', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				['s', 'e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				['i', 'r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nna (3 syllables).
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['p', 'a', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['s', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', 'm', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				['s', 'o', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						['m', 'i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						['i', 'd', 'n', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '-', 'e', 'r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ona (3 syllables).
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['r', 'd', 'n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 942);
			s.is_empty()
		}
		['t', 'o', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['z', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word pincerna (3 syllables).
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	s.is_empty()
}

/// Words ending -una (3 syllables).
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['l', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 945);
			s.is_empty()
		}
		['t', 'r', 'o', 'f', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1823);
					s.is_empty()
				}
				['s', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tiera (3 syllables).
pub(crate) fn n2332(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'm', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2685);
			s.is_empty()
		}
		['s', 'a', 't', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2686);
			s.is_empty()
		}
		['t', 'e', 'h', 'c', 's', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lera (3 syllables).
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1839);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nera (3 syllables).
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 976);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				['v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pera (3 syllables).
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 977);
	match s {
		[] => true,
		['o', 'd', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word portatessera (3 syllables).
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 978);
	s.is_empty()
}

/// Words ending -ettera (3 syllables).
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 979);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['i', 'd', 'u', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ira (3 syllables).
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', 's', 'r', 'e', 'p', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 980);
			s.is_empty()
		}
		['v', 'e', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ora (3 syllables).
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['n', 'e', 'm', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['p', 'm', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						['t', 'l', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['v', 'o', 'l', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2697);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3145);
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 3375);
													s.is_empty()
												}
												['i', 'g', '\u{1}', s @ ..] => {
													super::push(out, 3376);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 3325);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'l', 'i', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3326);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 3327);
											s.is_empty()
										}
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

/// Words ending -pra (3 syllables).
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	s.is_empty()
}

/// Words ending -rra (3 syllables).
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				['u', 'g', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tra (3 syllables).
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['b', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				['l', 'o', 'n', 'o', 'c', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2698);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'm', 'o', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 987);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2700);
							s.is_empty()
						}
						['h', 'c', 'r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2701);
							s.is_empty()
						}
						['n', 'i', 'f', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'i', 's', 'o', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1845);
					s.is_empty()
				}
				['o', 'm', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ura (3 syllables).
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['r', 'e', 'v', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 990);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2703);
							match s {
								[] => true,
								['b', 'b', 'e', 'd', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'z', 'a', 'p', 's', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 2704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['o', 'c', 'i', 'g', 'r', 'a', 'p', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1850);
					match s {
						[] => true,
						['e', 'v', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['e', 'v', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						['o', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vra (3 syllables).
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['o', 'n', 'a', 'm', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 992);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ta (3 syllables).
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['b', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['r', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['z', 'n', 'e', 'r', 'e', 'f', 'f', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'p', 'o', 'm', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['m', 'l', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1865);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3149);
											s.is_empty()
										}
										['t', 's', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3150);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'u', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['c', 'o', 't', 'n', 'o', 'r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3151);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 3328);
													s.is_empty()
												}
												['r', 'g', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3329);
													s.is_empty()
												}
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
					super::push(out, 1024);
					match s {
						[] => true,
						['o', 'p', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', s @ ..] => {
					match s {
						[] => true,
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						['t', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['n', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['z', 'a', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['b', 'a', 'f', 'l', 'a', 'n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['d', 'o', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				['f', 'o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2713);
									s.is_empty()
								}
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', 'o', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['t', 'a', 't', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2715);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['s', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3152);
											s.is_empty()
										}
										['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 's', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2716);
									s.is_empty()
								}
								['t', 'a', 'r', 'a', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3154);
											s.is_empty()
										}
										['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'p', 'o', 's', 's', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						['s', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 3156);
											s.is_empty()
										}
										['s', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 3157);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 't', 'o', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2718);
									s.is_empty()
								}
								['o', 't', 'a', 'n', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n508(s, out),
		['l', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['n', s @ ..] => super::t1::n510(s, out),
		['o', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['l', s @ ..] => super::t1::n1310(s, out),
				['n', s @ ..] => super::t1::n1311(s, out),
				['r', s @ ..] => super::t1::n1312(s, out),
				['u', s @ ..] => super::t1::n1313(s, out),
				['v', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n512(s, out),
		['s', s @ ..] => super::t1::n513(s, out),
		['t', s @ ..] => super::t1::n514(s, out),
		['u', s @ ..] => super::t1::n515(s, out),
		_ => false,
	}
}

/// Words ending -obile (3 syllables).
pub(crate) fn n2550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2020);
	match s {
		[] => true,
		['m', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2797);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word segnafile (3 syllables).
pub(crate) fn n1420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1135);
	s.is_empty()
}

/// Words ending -nile (3 syllables).
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1136);
	s.is_empty()
}

/// Words ending -oile (3 syllables).
pub(crate) fn n1422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1137);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 2798);
					s.is_empty()
				}
				['é', '\u{1}', s @ ..] => {
					super::push(out, 2799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word baciapile (3 syllables).
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1138);
	s.is_empty()
}

/// Words ending -rile (3 syllables).
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1139);
	match s {
		[] => true,
		['a', 'b', 'a', 'c', 'i', 'r', 'a', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2021);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word etilesile (3 syllables).
pub(crate) fn n1425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1140);
	s.is_empty()
}

/// Words ending -tile (3 syllables).
pub(crate) fn n1426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1141);
	s.is_empty()
}

/// Words ending -lle (3 syllables).
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				['p', 'a', 't', 'o', 'u', 'v', 's', '\u{1}', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						['o', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2801);
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
				['b', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'l', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2025);
					match s {
						[] => true,
						['a', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2803);
							s.is_empty()
						}
						['i', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['m', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								['n', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3196);
									s.is_empty()
								}
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
			super::push(out, 1143);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						['r', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'b', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				['h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['t', 'n', 'i', 'c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ole (3 syllables).
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['b', 'u', 'b', 'i', 'd', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['c', 'a', 'i', 'f', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				['s', 'u', 'i', 'a', 'm', 'a', 's', 's', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'e', 'c', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 1146);
			s.is_empty()
		}
		['g', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'r', 'b', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'i', 'p', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						['i', 'l', 'g', 'o', 'c', 'c', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'g', 'e', 's', 'a', 'd', 'l', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', 'i', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1148);
			s.is_empty()
		}
		['m', '-', 'o', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1149);
			s.is_empty()
		}
		['p', 'a', 'u', 'q', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1150);
			s.is_empty()
		}
		['r', 'e', 't', 'i', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1151);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						['g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['a', 'c', 's', 'i', s @ ..] => {
					match s {
						[] => true,
						['p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						['r', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'p', 'a', 'c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2035);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2815);
							s.is_empty()
						}
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								['a', 'r', 't', s @ ..] => {
									match s {
										[] => true,
										['s', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3336);
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3337);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'o', 'r', 'f', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2816);
							s.is_empty()
						}
						['i', 'd', 'n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', 'a', 't', 'n', 'o', 'c', 'c', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				['l', 'a', 'v', 'a', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word infilaperle (3 syllables).
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	s.is_empty()
}

/// Words ending -ule (3 syllables).
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['j', 'a', 'g', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2039);
					s.is_empty()
				}
				['p', '-', 'e', 'd', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'p', 'a', 'c', 'a', 'v', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ine (3 syllables).
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2063);
					match s {
						[] => true,
						['a', 'b', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						['o', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['t', 'i', 'h', 'g', 'n', 'i', 'l', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1176);
			match s {
				[] => true,
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['a', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['a', 'm', 'a', 'r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'i', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						['p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'g', 'a', 'b', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'f', 'a', 'p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1181);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1182);
			match s {
				[] => true,
				['t', 'a', 'l', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['u', 'o', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2831);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1183);
			match s {
				[] => true,
				['g', 'n', 'i', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1184);
			match s {
				[] => true,
				['a', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -penne (3 syllables).
pub(crate) fn n2620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2075);
	s.is_empty()
}

/// Words ending -one (3 syllables).
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['r', 'a', 'c', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1187);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1190);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1191);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						['t', 'r', 'a', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 2834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1193);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2083);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2087);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2089);
					match s {
						[] => true,
						['r', 'a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2090);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2091);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['i', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 't', 'a', s @ ..] => {
							super::push(out, 2839);
							match s {
								[] => true,
								['i', 'b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'm', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2840);
							s.is_empty()
						}
						['u', 'f', 'o', 'r', 't', s @ ..] => {
							super::push(out, 2841);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2092);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						['s', 'e', 'g', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2093);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2844);
							match s {
								[] => true,
								['c', 'i', 'f', 'i', s @ ..] => {
									match s {
										[] => true,
										['n', 'g', s @ ..] => {
											super::push(out, 3338);
											s.is_empty()
										}
										['t', 's', 'i', s @ ..] => {
											super::push(out, 3339);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 3202);
									match s {
										[] => true,
										['e', 'l', 'e', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3340);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3203);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 3341);
											s.is_empty()
										}
										['u', 'p', 'a', 'n', 'a', 'c', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 3342);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3204);
									match s {
										[] => true,
										['i', 's', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
										['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3344);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'z', 'i', 'l', 'a', 'c', 'i', 'd', 'e', s @ ..] => {
									super::push(out, 3205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2845);
							match s {
								[] => true,
								['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['a', 's', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['m', 'a', 'm', 'o', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1196);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1197);
			match s {
				[] => true,
				['a', 's', 'a', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1198);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2097);
					match s {
						[] => true,
						['e', 'p', 'a', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'e', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3345);
											s.is_empty()
										}
										['i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3346);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'a', 's', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['s', 'a', 'b', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				['t', 'e', 'z', 'z', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1201);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rne (3 syllables).
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2101);
					s.is_empty()
				}
				['i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ere (3 syllables).
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1214);
			s.is_empty()
		}
		['c', 'o', 'u', 's', 'a', 't', 'n', 'e', 'm', 'r', 'o', 'd', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1215);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['e', 'c', 'o', 'r', 'p', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2116);
					s.is_empty()
				}
				['n', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'r', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 1216);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1217);
			match s {
				[] => true,
				['b', 'b', s @ ..] => {
					super::push(out, 2118);
					match s {
						[] => true,
						['a', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'g', 'i', 's', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2120);
					match s {
						[] => true,
						['i', 'd', 'e', 'p', 'r', 'o', 't', 'a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						['n', 'o', 'p', 'u', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						['o', 'm', 'i', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'e', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2121);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['e', 'c', 'r', 'a', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						['r', 'a', 'b', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						['u', 't', 'n', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['a', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1218);
			match s {
				[] => true,
				['o', 'v', 'n', 'o', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 'a', 't', 't', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				['h', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					super::push(out, 2127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1220);
			match s {
				[] => true,
				['a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2128);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3208);
									s.is_empty()
								}
								['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 's', 'e', s @ ..] => {
			match s {
				[] => true,
				['n', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2130);
					s.is_empty()
				}
				['t', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['s', 'i', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2132);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['e', 'l', 'a', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
								['s', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3211);
									s.is_empty()
								}
								['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
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
		['v', 'l', 'o', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				['p', 'a', 'r', 'i', 'p', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ardire (3 syllables).
pub(crate) fn n1534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1222);
	s.is_empty()
}

/// Words ending -sire (3 syllables).
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2136);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2866);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tire (3 syllables).
pub(crate) fn n1536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1223);
	s.is_empty()
}

/// Words ending -ore (3 syllables).
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['c', 'e', 'p', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1225);
			s.is_empty()
		}
		['l', 'o', 'c', 'o', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['t', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', 'v', 'o', 'l', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['t', 't', 'a', 'u', 'q', 'i', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['i', 'g', 'g', 'e', 's', 'r', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						['r', 'i', 'p', 's', 'e', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2870);
							match s {
								[] => true,
								['t', 'e', 'c', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2140);
					match s {
						[] => true,
						['l', 'g', 'o', 'c', 'c', 'a', 'r', '-', 'e', 'r', 'o', 't', 'a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2141);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['-', 'o', 'n', 'e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3214);
									s.is_empty()
								}
								['a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2143);
					match s {
						[] => true,
						['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						['r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['o', 't', 'n', 'a', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'l', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						['i', 'g', 'g', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word succiacapre (3 syllables).
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	s.is_empty()
}

/// Words ending -rre (3 syllables).
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['a', 'g', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tre (3 syllables).
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['e', 'i', 'p', 'a', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						['l', 'g', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ure (3 syllables).
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['c', 'i', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				['n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'r', 'e', 'v', 'a', 't', 't', 'e', 'f', 'f', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1232);
			s.is_empty()
		}
		['h', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		['r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1234);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', 'v', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['t', 'a', 'f', 'a', 'i', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ère (3 syllables).
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	s.is_empty()
}

/// Words ending -lite (3 syllables).
pub(crate) fn n1586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'a', 't', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2884);
					s.is_empty()
				}
				['t', 'o', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2171);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2886);
					s.is_empty()
				}
				['d', 'o', s @ ..] => {
					super::push(out, 2887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', '\u{1}', s @ ..] => {
			super::push(out, 2172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -limite (3 syllables).
pub(crate) fn n1587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1266);
	match s {
		[] => true,
		['a', 'n', 'g', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 2173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nite (3 syllables).
pub(crate) fn n1588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1267);
	match s {
		[] => true,
		['a', 'i', 'v', 'i', 'l', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2174);
			s.is_empty()
		}
		['n', 'a', 'm', 'k', 'c', 'o', 'l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2175);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rite (3 syllables).
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'n', 'a', 's', 's', 'e', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2177);
			s.is_empty()
		}
		['e', 't', 'n', 'a', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 2178);
			s.is_empty()
		}
		['o', 'e', s @ ..] => {
			super::push(out, 2179);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word andalusite (3 syllables).
pub(crate) fn n1591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	s.is_empty()
}

/// Words ending -tite (3 syllables).
pub(crate) fn n1592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1270);
	match s {
		[] => true,
		['a', 'm', 'a', s @ ..] => {
			super::push(out, 2180);
			s.is_empty()
		}
		['r', 'o', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -avite (3 syllables).
pub(crate) fn n1593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1271);
	s.is_empty()
}

/// Words ending -nte (3 syllables).
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['f', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 'g', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						['o', 's', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', s @ ..] => {
					super::push(out, 2186);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['d', 'n', 'e', 'p', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2893);
							match s {
								[] => true,
								['l', 'l', 'o', 'f', 's', '\u{1}', s @ ..] => {
									super::push(out, 3220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['o', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['a', 'v', 'i', 'u', 'q', 'e', '-', 'o', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2898);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2899);
							match s {
								[] => true,
								['o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3221);
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
						['t', 'i', s @ ..] => {
							super::push(out, 2900);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['p', 'o', 'r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'p', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ote (3 syllables).
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 556);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1275);
			s.is_empty()
		}
		['n', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1276);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'c', 'a', 't', 'n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1278);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rte (3 syllables).
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 557);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['a', 'p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 3222);
									s.is_empty()
								}
								['t', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						['t', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1281);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2906);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								['s', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						['z', 'z', 'a', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				['t', 'r', 'e', 'h', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ste (3 syllables).
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2201);
					s.is_empty()
				}
				['f', 'a', 't', 's', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'u', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2203);
					s.is_empty()
				}
				['v', 'i', 'r', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tte (3 syllables).
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1284);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'l', 'b', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['l', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', 't', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						['t', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['u', 'q', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				['v', 'o', 'r', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['m', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								['a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
								['r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 's', 'i', 'h', 'c', 'n', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['l', 'u', s @ ..] => {
					match s {
						[] => true,
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2211);
					match s {
						[] => true,
						['a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ute (3 syllables).
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['e', 'v', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'v', 'a', 'i', 'b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yte (3 syllables).
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	s.is_empty()
}

/// The word ombellico (3 syllables).
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2233);
	s.is_empty()
}

/// The word dolico (3 syllables).
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	s.is_empty()
}

/// Words ending -nico (3 syllables).
pub(crate) fn n1664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1329);
	match s {
		[] => true,
		['c', 'e', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2235);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rico (3 syllables).
pub(crate) fn n1665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1330);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3237);
							s.is_empty()
						}
						['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				['g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2931);
					s.is_empty()
				}
				['l', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'm', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2237);
			s.is_empty()
		}
		['t', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 2238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tico (3 syllables).
pub(crate) fn n1666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1331);
	match s {
		[] => true,
		['e', 'l', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 2239);
			s.is_empty()
		}
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2240);
			s.is_empty()
		}
		['s', 'a', 'm', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vico (3 syllables).
pub(crate) fn n1667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1332);
	s.is_empty()
}

/// Words ending -zico (3 syllables).
pub(crate) fn n1668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1333);
	s.is_empty()
}

/// Words ending -lco (3 syllables).
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', 'e', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				['p', 'i', 'r', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				['t', 'i', 'g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nco (3 syllables).
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', 'b', 'o', 't', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				['r', 'b', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'e', 'i', 't', 'n', 'a', 'v', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1334);
			s.is_empty()
		}
		['o', 'r', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1335);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oco (3 syllables).
pub(crate) fn n709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	match s {
		[] => true,
		['i', 'g', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1336);
			s.is_empty()
		}
		['l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['u', 'f', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['-', 'a', 'c', 'c', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['t', 't', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						['z', 'z', 'i', 't', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'i', 'u', 'q', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rco (3 syllables).
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['a', 'p', 'a', s @ ..] => {
			match s {
				[] => true,
				['d', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['i', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sco (3 syllables).
pub(crate) fn n711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['a', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1341);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'g', 'n', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2252);
					s.is_empty()
				}
				['l', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'b', 'a', 'i', 'c', 'c', 'a', 'r', 'b', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1342);
			s.is_empty()
		}
		['u', 'r', 't', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uco (3 syllables).
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['b', 'o', 's', 's', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -logo (3 syllables).
pub(crate) fn n1728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2284);
			match s {
				[] => true,
				['m', 'm', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2285);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2953);
					s.is_empty()
				}
				['m', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2954);
					s.is_empty()
				}
				['n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2955);
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2956);
					s.is_empty()
				}
				['r', 'p', s @ ..] => {
					super::push(out, 2957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgo (3 syllables).
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['u', 't', 'a', 'm', 'u', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 1378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugo (3 syllables).
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	s.is_empty()
}

/// Words ending -agnio (2 syllables).
pub(crate) fn n1754(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2302);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 2303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pio (2 syllables).
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['m', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 1396);
			s.is_empty()
		}
		['s', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1397);
			s.is_empty()
		}
		['u', 'i', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio (2 syllables).
pub(crate) fn n766(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		['i', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sio (2 syllables).
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1401);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1402);
			s.is_empty()
		}
		['u', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tio (2 syllables).
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 647);
	match s {
		[] => true,
		['i', 'a', 'r', 'b', 's', '\u{1}', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['o', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1405);
			s.is_empty()
		}
		['s', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 2964);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sciacquio (2 syllables).
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	s.is_empty()
}

/// Words ending -vio (2 syllables).
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		['n', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zio (2 syllables).
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1409);
			s.is_empty()
		}
		['z', 'i', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -io (3 syllables).
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['-', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 650);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['n', 'i', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'm', 'o', 'b', 'm', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['c', 'a', 'i', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['h', 'g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'r', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3243);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['i', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 3347);
													s.is_empty()
												}
												['r', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3348);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 't', 't', 'a', 'b', 's', '\u{1}', s @ ..] => {
									super::push(out, 2966);
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
						['f', 'f', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['u', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'i', 's', 'a', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2312);
							s.is_empty()
						}
						['n', 'r', 'e', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'c', 'o', 't', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2314);
							s.is_empty()
						}
						['g', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						['r', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						['z', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'u', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						['i', 'd', 'a', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2968);
									s.is_empty()
								}
								['i', 'r', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3244);
											match s {
												[] => true,
												['s', 's', 'u', '\u{1}', s @ ..] => {
													super::push(out, 3349);
													s.is_empty()
												}
												['x', 'u', '\u{1}', s @ ..] => {
													super::push(out, 3350);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3245);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['m', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						['r', 'a', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['v', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'p', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				['r', 'o', 'b', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'f', 'u', 'r', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2969);
									s.is_empty()
								}
								['r', 'o', 'f', 'a', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 2971);
									s.is_empty()
								}
								['p', 'p', 'e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'l', 'o', 'r', 'o', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['e', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
								['o', 'n', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								['u', 'i', 'c', 'a', 'b', 's', '\u{1}', s @ ..] => {
									super::push(out, 2975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['r', 'r', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2976);
									s.is_empty()
								}
								['t', 's', 'i', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'i', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						['s', 'u', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n780(s, out),
		['n', s @ ..] => {
			super::push(out, 658);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'a', 'b', 's', '\u{1}', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2338);
							s.is_empty()
						}
						['m', 'i', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2998);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'p', 'o', 'n', 'c', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['i', s @ ..] => super::t1::n1799(s, out),
				['n', s @ ..] => super::t1::n1800(s, out),
				['o', s @ ..] => super::t1::n1801(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n782(s, out),
		['p', s @ ..] => super::t1::n783(s, out),
		['r', s @ ..] => super::t1::n784(s, out),
		['s', 's', 'a', 't', 'a', 'p', '\u{1}', s @ ..] => super::t1::n785(s, out),
		['t', s @ ..] => super::t1::n786(s, out),
		['u', s @ ..] => super::t1::n787(s, out),
		['z', s @ ..] => super::t1::n788(s, out),
		_ => false,
	}
}

/// The word torcicollo (3 syllables).
pub(crate) fn n4108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3351);
	s.is_empty()
}

/// The word storcicollo (3 syllables).
pub(crate) fn n4109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3352);
	s.is_empty()
}

/// The word originalismo (3 syllables).
pub(crate) fn n4110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3353);
	s.is_empty()
}

/// The word antinazionalismo (3 syllables).
pub(crate) fn n4111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3354);
	s.is_empty()
}

/// Words ending -vanismo (3 syllables).
pub(crate) fn n3741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3030);
	s.is_empty()
}

/// Words ending -inismo (3 syllables).
pub(crate) fn n3742(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'l', 'e', 'b', 'i', 'h', 'g', 'o', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 3263);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3264);
			match s {
				[] => true,
				['e', 'l', '-', 'o', 'm', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word clintonismo (3 syllables).
pub(crate) fn n3743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3031);
	s.is_empty()
}

/// Words ending -ocomunismo (3 syllables).
pub(crate) fn n3744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3032);
	match s {
		[] => true,
		['g', 'o', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3265);
			s.is_empty()
		}
		['r', 'e', 't', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 3266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rismo (3 syllables).
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2390);
	match s {
		[] => true,
		['i', 'a', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3033);
			s.is_empty()
		}
		['o', 'r', 'r', 'e', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3034);
			s.is_empty()
		}
		['u', 't', 'n', s @ ..] => {
			super::push(out, 3035);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word doppiopesismo (3 syllables).
pub(crate) fn n3011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2391);
	s.is_empty()
}

/// Words ending -tismo (3 syllables).
pub(crate) fn n3012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2392);
	match s {
		[] => true,
		['i', 'c', 's', 'u', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3036);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['e', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word schiavismo (3 syllables).
pub(crate) fn n3013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2393);
	s.is_empty()
}

/// Words ending -umo (3 syllables).
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'c', 'a', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3270);
									s.is_empty()
								}
								['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word portaspazzolino (3 syllables).
pub(crate) fn n1868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1481);
	s.is_empty()
}

/// Words ending -rino (3 syllables).
pub(crate) fn n1869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['o', 't', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2408);
			s.is_empty()
		}
		['t', 'e', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2409);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word scaricalasino (3 syllables).
pub(crate) fn n1870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1483);
	s.is_empty()
}

/// Words ending -istino (3 syllables).
pub(crate) fn n1871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1484);
	match s {
		[] => true,
		['l', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2410);
			s.is_empty()
		}
		['r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -avino (3 syllables).
pub(crate) fn n1872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1485);
	match s {
		[] => true,
		['i', 'g', 'g', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2412);
			s.is_empty()
		}
		['t', 't', 'o', 'b', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 2413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nno (3 syllables).
pub(crate) fn n815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				['e', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 's', 'a', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['t', 'u', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ono (3 syllables).
pub(crate) fn n816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	match s {
		[] => true,
		['u', 'b', 'o', 'v', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rno (3 syllables).
pub(crate) fn n817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['f', 'o', 't', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['i', 'g', 'i', 'd', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 't', s @ ..] => {
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word novantuno (3 syllables).
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	s.is_empty()
}

/// Words ending -nto (3 syllables).
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 721);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['c', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['i', 'p', 'm', 'i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				['s', 'o', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2464);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3060);
							match s {
								[] => true,
								['n', 'o', 'i', 'z', 'e', 'f', 'r', s @ ..] => {
									super::push(out, 3282);
									match s {
										[] => true,
										['e', 'p', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3356);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'i', s @ ..] => {
									super::push(out, 3283);
									match s {
										[] => true,
										['l', 'g', 'i', 'm', s @ ..] => {
											match s {
												[] => true,
												['-', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3378);
													s.is_empty()
												}
												['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3379);
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
										['e', 'l', s @ ..] => {
											super::push(out, 3357);
											match s {
												[] => true,
												['p', 'm', 'o', 'c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 3380);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 3358);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'c', '-', 'o', 'i', 'a', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3062);
							match s {
								[] => true,
								['r', 'a', 'i', s @ ..] => {
									super::push(out, 3284);
									match s {
										[] => true,
										['h', 'c', 'n', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3359);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'r', 'a', 'p', 'i', s @ ..] => {
									super::push(out, 3285);
									match s {
										[] => true,
										['d', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3360);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'o', 'm', 'o', s @ ..] => {
									super::push(out, 3286);
									match s {
										[] => true,
										['p', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3361);
											s.is_empty()
										}
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
					super::push(out, 2466);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3362);
											s.is_empty()
										}
										['l', 'g', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 3363);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3287);
									s.is_empty()
								}
								['r', 't', 's', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3288);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3364);
											s.is_empty()
										}
										['u', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3365);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'a', 'i', 'c', 'c', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3063);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3289);
									s.is_empty()
								}
								['t', 't', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1533);
			s.is_empty()
		}
		['o', 'c', 'a', 'd', 'l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1534);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3064);
							s.is_empty()
						}
						['o', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -moto (3 syllables).
pub(crate) fn n1943(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 3066);
					s.is_empty()
				}
				['p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rto (3 syllables).
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 723);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['a', 'c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3068);
							s.is_empty()
						}
						['o', 'b', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['t', 'o', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sto (3 syllables).
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 724);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'l', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				['n', 'n', 'i', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'f', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1537);
			s.is_empty()
		}
		['o', 'p', 'o', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				['p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'i', 'g', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tto (3 syllables).
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 725);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['g', 'a', 'z', 'z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				['i', 'p', 'p', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				['r', 'f', 's', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1541);
			match s {
				[] => true,
				['c', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2482);
					s.is_empty()
				}
				['g', 'i', 'p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				['i', 'a', 'r', 'b', 'b', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['-', 'o', 'n', 'a', 'v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						['i', 'd', 'n', 'e', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2486);
					match s {
						[] => true,
						['i', 'b', 'a', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3073);
							s.is_empty()
						}
						['n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2488);
					match s {
						[] => true,
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'o', 'r', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2490);
					match s {
						[] => true,
						['a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', s @ ..] => {
			match s {
				[] => true,
				['f', 'a', 'l', 'o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				['i', 'v', 'i', 'r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'a', 'n', 'e', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 2495);
					match s {
						[] => true,
						['c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3077);
							s.is_empty()
						}
						['i', 'c', 's', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2496);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uto (3 syllables).
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 726);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1543);
			s.is_empty()
		}
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1544);
			s.is_empty()
		}
		['n', 'e', 'v', 'n', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1545);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word biocinetica (3 syllables).
pub(crate) fn n4141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3381);
	s.is_empty()
}

/// The word termocinetica (3 syllables).
pub(crate) fn n4142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3382);
	s.is_empty()
}

/// The word sottotraccia (3 syllables).
pub(crate) fn n4126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3367);
	s.is_empty()
}

/// The word autotraccia (3 syllables).
pub(crate) fn n4127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3368);
	s.is_empty()
}

/// The word camiscia (3 syllables).
pub(crate) fn n2098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1652);
	s.is_empty()
}

/// The word dietrocoscia (3 syllables).
pub(crate) fn n2099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1653);
	s.is_empty()
}

/// Words ending -atiuscia (3 syllables).
pub(crate) fn n2100(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 2567);
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 2568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fiducia (3 syllables).
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1654);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 1655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dia (3 syllables).
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['e', 'm', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				['r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 'd', 'o', 'c', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['r', 'a', 'u', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fia (3 syllables).
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', 'o', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						['s', 's', 'o', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3106);
									s.is_empty()
								}
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

/// Words ending -gia (3 syllables).
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3107);
									s.is_empty()
								}
								['l', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3108);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3109);
									match s {
										[] => true,
										['b', 'e', 'r', 'e', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3306);
											s.is_empty()
										}
										['e', 't', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3307);
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 3308);
											s.is_empty()
										}
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
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						['p', 'o', 'i', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'f', 'n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3110);
									s.is_empty()
								}
								['t', 'a', 'm', 'o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3111);
									s.is_empty()
								}
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
				['i', 'l', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['l', 'p', 'o', 'r', 'f', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['e', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['o', 'i', 'p', 'a', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						['u', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['a', 'i', 'g', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'n', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['a', 'r', 'f', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				['i', 'r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2578);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3112);
									match s {
										[] => true,
										['a', 'r', 't', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3309);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 3310);
											s.is_empty()
										}
										['p', 'o', 'c', 'a', 'm', 'r', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3311);
											s.is_empty()
										}
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
			super::push(out, 828);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2579);
							s.is_empty()
						}
						['l', 'l', 'a', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						['n', 'i', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						['g', 'a', 'p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						['s', 's', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						['l', 'l', 'a', 't', 'e', 'm', 'o', 'm', 'r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						['m', 'i', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						['t', 'i', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'l', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hia (3 syllables).
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'm', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 1669);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3312);
											s.is_empty()
										}
										['g', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3313);
											s.is_empty()
										}
										['s', 's', 'a', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 3314);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'i', 'c', 'i', 'd', 'n', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kia (3 syllables).
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['n', 'i', 'm', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 'm', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				['o', 'b', 'm', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lia (3 syllables).
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['a', 'b', '-', 'o', 't', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['i', 'c', 'n', 'i', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 3315);
											s.is_empty()
										}
										['r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3316);
											s.is_empty()
										}
										['s', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3317);
											s.is_empty()
										}
										['t', 'i', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 3318);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', 'o', 't', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'v', 'i', 'm', 'r', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'g', 'e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						['h', 'c', 'a', 's', 'l', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						['m', 'a', 'f', 'o', s @ ..] => {
							super::push(out, 2594);
							match s {
								[] => true,
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'd', s @ ..] => {
							super::push(out, 2595);
							match s {
								[] => true,
								['a', 'u', 'q', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'c', 'n', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						['s', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['t', 'a', 'p', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mia (3 syllables).
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', 'c', s @ ..] => {
					super::push(out, 1674);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3116);
									s.is_empty()
								}
								['r', 't', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'o', 't', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ì', 'h', 'c', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nia (3 syllables).
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['t', 's', 'i', 'r', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3118);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 3119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'o', 'f', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'g', 'r', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pia (3 syllables).
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['o', 'c', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ria (3 syllables).
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['e', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						['o', 'n', 'g', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'i', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'b', 'a', 't', '-', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2602);
							match s {
								[] => true,
								['a', 'i', 'b', s @ ..] => {
									match s {
										[] => true,
										['a', 'g', 'u', 'i', 'c', 's', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3319);
											s.is_empty()
										}
										['i', 'd', 'n', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 3320);
											s.is_empty()
										}
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
						['r', 'o', s @ ..] => {
							super::push(out, 2603);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 3120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'b', 'a', 'c', 'i', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['l', 'g', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				['m', 'e', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['a', 'p', 'a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['e', 'm', 'o', 'n', 'o', 'r', 'c', 'o', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', 'i', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['c', 'a', 'o', 'n', 'i', 'm', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sia (3 syllables).
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['o', 'c', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'r', 'o', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tia (3 syllables).
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['i', 'l', 'o', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 842);
			s.is_empty()
		}
		['s', 'e', 'b', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['b', 'a', 'k', 'n', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word alleluia (3 syllables).
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	s.is_empty()
}

/// Words ending -via (3 syllables).
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['p', 'p', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'c', 'o', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 844);
			s.is_empty()
		}
		['o', 'r', 'r', 'e', 'f', 'a', 'c', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zia (3 syllables).
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['a', 'b', 'b', 'a', 'i', 'c', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 846);
			s.is_empty()
		}
		['i', 'l', 'l', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['o', 'e', 'b', 'n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mamma (3 syllables).
pub(crate) fn n2232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1756);
	match s {
		[] => true,
		['i', 't', 't', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2646);
			s.is_empty()
		}
		['o', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 2647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ramma (3 syllables).
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1757);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3323);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'f', 'e', 'c', 'n', 'e', 'o', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3369);
									s.is_empty()
								}
								['u', 'c', 'o', 'o', 'r', 't', 't', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3324);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3371);
									s.is_empty()
								}
								['p', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3373);
									s.is_empty()
								}
								['o', 'f', 'o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3374);
									s.is_empty()
								}
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

/// Words ending -emma (3 syllables).
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['l', 'i', 'r', 'v', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1759);
			s.is_empty()
		}
		['r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		['t', 'f', 'o', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oma (3 syllables).
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['e', 'd', 'i', '\u{1}', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		['g', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['i', 'g', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['o', 'f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['o', 'c', 'r', 'e', 'b', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				['u', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'd', 'a', 'o', 'f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				['i', 'n', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		['r', 'c', s @ ..] => {
			super::push(out, 912);
			s.is_empty()
		}
		['s', 'o', s @ ..] => {
			match s {
				[] => true,
				['l', 'c', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				['n', 'i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3139);
									s.is_empty()
								}
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2648);
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
				['a', 'r', 'e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2649);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'o', 'r', 't', 's', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2651);
							s.is_empty()
						}
						['o', 'g', 'i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'x', 'o', 'r', 'b', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'b', 'o', s @ ..] => {
							match s {
								[] => true,
								['l', 'u', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3141);
									s.is_empty()
								}
								['r', 'd', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'a', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rma (3 syllables).
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['a', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 914);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				['p', 's', 'o', s @ ..] => {
					match s {
						[] => true,
						['i', 'g', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2655);
							s.is_empty()
						}
						['n', 'm', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'x', '\u{1}', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						['s', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					match s {
						[] => true,
						['s', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['t', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'i', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sma (3 syllables).
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['a', 'l', 'p', 'o', 'l', 'a', 'i', '\u{1}', s @ ..] => {
			super::push(out, 918);
			s.is_empty()
		}
		['i', 's', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 919);
			s.is_empty()
		}
		['u', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uma (3 syllables).
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['a', 'r', 't', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'c', 'c', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2661);
							s.is_empty()
						}
						['o', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 3143);
									s.is_empty()
								}
								['g', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'm', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 922);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word proscinèma (3 syllables).
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	s.is_empty()
}

/// The word aiscrìma (3 syllables).
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	s.is_empty()
}

/// Words ending -ita (3 syllables).
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['a', 'l', 'o', 'c', 'i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						['n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['c', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1884);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				['n', 'i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								['x', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['i', 'v', 'o', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						['m', 'u', 'e', 'n', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', 'e', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3160);
									s.is_empty()
								}
								['e', 'l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 3161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						['t', 'a', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3162);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 3163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'f', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['u', 'q', 'c', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						['v', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				['o', 'l', 'g', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				['r', 'a', 'f', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['u', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['a', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						['r', 'p', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						['t', 'a', 'm', 'r', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', 'o', 'e', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				['u', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1897);
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
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						['t', 'f', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'o', 'r', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['e', 'a', s @ ..] => {
					match s {
						[] => true,
						['m', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2735);
							s.is_empty()
						}
						['r', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 2737);
							s.is_empty()
						}
						['z', 'a', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'a', 'f', 'f', 'a', 'r', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1899);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', 't', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3164);
									s.is_empty()
								}
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								['r', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 't', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						['t', 'a', 'm', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['a', 'l', 'a', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				['r', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						['m', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['i', 'e', 'r', 'h', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2746);
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				['c', 'o', 'p', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				['e', 'd', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				['i', 'g', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1909);
					s.is_empty()
				}
				['o', 'i', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				['t', 's', 'e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['u', 'q', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'r', 'a', 'p', 'o', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['a', 'm', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				['r', 'a', 'p', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['n', 's', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['s', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => super::t1::n1302(s, out),
		['z', s @ ..] => super::t1::n1303(s, out),
		_ => false,
	}
}

/// Words ending -nta (3 syllables).
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', 'o', 'c', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				['p', 'o', 'c', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				['r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				['l', 'o', 'p', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['m', 'l', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				['p', 's', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2754);
							s.is_empty()
						}
						['o', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', 'p', 'm', 'i', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['u', 'p', 'a', s @ ..] => {
			match s {
				[] => true,
				['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lota (3 syllables).
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'p', 'o', s @ ..] => {
			super::push(out, 1932);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nota (3 syllables).
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1934);
			s.is_empty()
		}
		['o', 'c', 'n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rota (3 syllables).
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1936);
			s.is_empty()
		}
		['r', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uota (3 syllables).
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['q', 'i', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1938);
			s.is_empty()
		}
		['r', 'o', 't', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rta (3 syllables).
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['a', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1941);
					match s {
						[] => true,
						['a', 'b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2757);
							s.is_empty()
						}
						['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1943);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3168);
									s.is_empty()
								}
								['r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2759);
							s.is_empty()
						}
						['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						['p', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3170);
									s.is_empty()
								}
								['v', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3171);
									s.is_empty()
								}
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

/// Words ending -sta (3 syllables).
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1944);
					match s {
						[] => true,
						['a', 'p', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['d', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['l', 'a', 'b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						['i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						['n', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['c', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['d', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				['g', 'g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 3172);
									s.is_empty()
								}
								['n', 'a', 'g', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3174);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3175);
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3330);
											s.is_empty()
										}
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
					super::push(out, 1953);
					match s {
						[] => true,
						['e', 'c', 's', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['n', 'e', 'l', '-', 'a', 't', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['a', 'r', 'e', 't', 'o', 't', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2771);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['o', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3331);
											s.is_empty()
										}
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2772);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3333);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3377);
													s.is_empty()
												}
												['o', 'r', s @ ..] => {
													match s {
														[] => true,
														['c', 'i', 'm', '\u{1}', s @ ..] => super::t1::n4143(s, out),
														['t', 't', 'e', 'p', 's', '\u{1}', s @ ..] => super::t1::n4144(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1955);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						['e', 'g', 'n', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2774);
							s.is_empty()
						}
						['à', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2776);
							s.is_empty()
						}
						['e', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['a', 'r', 't', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2778);
							s.is_empty()
						}
						['i', 'b', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3179);
									s.is_empty()
								}
								['u', 'l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 3180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['a', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3334);
											s.is_empty()
										}
										['o', 'n', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3335);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2781);
							match s {
								[] => true,
								['e', 's', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['a', 'l', 'l', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				['c', 'o', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['n', 'g', 'a', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3183);
									s.is_empty()
								}
								['m', 'r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3184);
									s.is_empty()
								}
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
			super::push(out, 1051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tta (3 syllables).
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1961);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3185);
									s.is_empty()
								}
								['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				['v', 'a', 'r', 'c', 'a', 'm', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['i', 'h', 'c', 'c', 'o', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['c', 'i', 'c', 'i', 'b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2785);
							s.is_empty()
						}
						['o', 'm', 'm', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'g', 'u', 'p', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				['v', 'i', 'c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 3187);
									s.is_empty()
								}
								['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uta (3 syllables).
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1056);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 't', 't', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1058);
			s.is_empty()
		}
		['p', 'a', 's', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lio (3 syllables).
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'b', 's', '\u{1}', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1420);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'b', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2978);
							s.is_empty()
						}
						['i', 'm', 'e', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2979);
							s.is_empty()
						}
						['t', 'o', 's', 'l', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2981);
							match s {
								[] => true,
								['c', 'a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3246);
									s.is_empty()
								}
								['o', 'e', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', 'i', 't', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['v', 'n', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'b', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1422);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						['o', 'e', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 3249);
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
						['p', 'i', 'c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3251);
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
						['c', 'i', 'n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						['d', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'c', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['l', 'l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				['m', 'o', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						['s', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						['t', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								['o', 'u', 'l', 'f', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3253);
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
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3254);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3255);
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
						['s', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						['t', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inio (3 syllables).
pub(crate) fn n1799(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'r', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 2340);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				['i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 3001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 2341);
			s.is_empty()
		}
		['g', 'u', 'l', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2342);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				['l', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				['o', 'g', 's', '\u{1}', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2343);
			s.is_empty()
		}
		['v', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnio (3 syllables).
pub(crate) fn n1800(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 't', 'n', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 2345);
			s.is_empty()
		}
		['i', 't', 'n', 'i', 't', '\u{1}', s @ ..] => {
			super::push(out, 2346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onio (3 syllables).
pub(crate) fn n1801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1426);
	match s {
		[] => true,
		['r', 't', 'i', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oio (3 syllables).
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['u', 'c', 'l', 'i', 'm', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pio (3 syllables).
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1428);
			s.is_empty()
		}
		['r', 'a', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1429);
			s.is_empty()
		}
		['s', 'a', 'n', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio (3 syllables).
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1431);
			match s {
				[] => true,
				['f', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['n', 'e', 'g', 'a', 'u', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['f', 'i', 'c', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				['i', 'h', 'c', 'c', 'a', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2352);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['o', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3006);
							s.is_empty()
						}
						['r', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 3007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2353);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3008);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'b', 's', '\u{1}', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['t', 'a', 'r', 'o', 'b', 'a', 'l', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['v', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['o', 't', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						['u', 't', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'u', 's', 's', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1435);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word patassio (3 syllables).
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	s.is_empty()
}

/// Words ending -tio (3 syllables).
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1436);
			s.is_empty()
		}
		['c', 'o', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2360);
					s.is_empty()
				}
				['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 't', 's', 'm', 'r', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1437);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						['o', 'n', 'a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'w', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'e', 's', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2364);
					s.is_empty()
				}
				['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uio (3 syllables).
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1439);
			s.is_empty()
		}
		['q', 'c', 'a', 'l', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zio (3 syllables).
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	match s {
		[] => true,
		['a', 'c', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1441);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1442);
			match s {
				[] => true,
				['v', 'r', 'e', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vita (3 syllables).
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1040);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1917);
			s.is_empty()
		}
		['i', 'm', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', 's', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zita (3 syllables).
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1919);
			s.is_empty()
		}
		['e', 'p', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 1920);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word microscopista (3 syllables).
pub(crate) fn n4143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3383);
	s.is_empty()
}

/// The word spettroscopista (3 syllables).
pub(crate) fn n4144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3384);
	s.is_empty()
}

