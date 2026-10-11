use flexion::Path;

/// Words ending -lior.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7873);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['q', 'e', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 569);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4203);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 11705);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4205);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7875);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 7876);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11706);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 11707);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 7877);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7878);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7879);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11708);
											match s {
												[] => true,
												['l', 'o', 't', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n31643(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t3::n31644(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 11709);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 11710);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7880);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 11712);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 11713);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11714);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14816);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 14817);
													match s {
														[] => true,
														['a', s @ ..] => super::t3::n31645(s, out),
														['t', s @ ..] => super::t3::n31646(s, out),
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
							super::push(out, 4206);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4207);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4208);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1762);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7881);
									match s {
										[] => true,
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4209);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t3::n626(s, out),
		['u', s @ ..] => super::t3::n627(s, out),
		_ => false,
	}
}

/// The word firmior.
pub(crate) fn n5145(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nfirmior.
pub(crate) fn n5146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4211);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nior.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4212);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4213);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1776);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4214);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 578);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1783);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4216);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1787);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 583);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 584);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['t', 'r', 'o', 'p', 'o', s @ ..] => {
					super::push(out, 1789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pior.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	s.is_empty()
}

/// Words ending -rior.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4219);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4220);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4222);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1794);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1796);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1797);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4224);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7882);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4225);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7884);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 590);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 592);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4226);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1805);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4228);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sior.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 597);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4230);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7886);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7888);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 600);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4232);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4233);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									super::push(out, 7889);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11715);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 11716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4234);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4235);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7890);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4236);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4237);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7892);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7893);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7894);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 11717);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 11718);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 11719);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 11720);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4238);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7895);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 11721);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 11722);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4239);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7897);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4241);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11723);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4242);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4243);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 7899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1812);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7900);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7901);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7902);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7903);
									s.is_empty()
								}
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
			super::push(out, 601);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7904);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7905);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 11725);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 1814);
					match s {
						[] => true,
						['m', 'e', 'd', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				['f', s @ ..] => super::t3::n2015(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tior.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7906);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7907);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 14818);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 14819);
													s.is_empty()
												}
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
					super::push(out, 1818);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4248);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11726);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 11727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1819);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4250);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4252);
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
				['m', s @ ..] => {
					super::push(out, 1822);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4253);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4258);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 11728);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4260);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7908);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11729);
											match s {
												[] => true,
												['s', 'n', 'o', 'c', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n31647(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t3::n31648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 11730);
											match s {
												[] => true,
												['m', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n31649(s, out),
														['m', 'i', '\u{1}', s @ ..] => super::t3::n31650(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t3::n31651(s, out),
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
									super::push(out, 7909);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7910);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 11731);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11732);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4261);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4262);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4263);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1825);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4266);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7911);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7912);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => super::t3::n25796(s, out),
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
				['v', s @ ..] => super::t3::n2025(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t3::n658(s, out),
		['e', s @ ..] => super::t3::n659(s, out),
		['i', s @ ..] => super::t3::n660(s, out),
		['l', s @ ..] => super::t3::n661(s, out),
		['n', s @ ..] => super::t3::n662(s, out),
		['o', s @ ..] => super::t3::n663(s, out),
		['p', s @ ..] => super::t3::n664(s, out),
		['r', s @ ..] => super::t3::n665(s, out),
		['s', s @ ..] => super::t3::n666(s, out),
		['u', s @ ..] => super::t3::n667(s, out),
		_ => false,
	}
}

/// Words ending -uior.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1865);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vior.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4316);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 617);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xior.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 622);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alis.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 646);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1901);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4337);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7942);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7943);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11746);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 11747);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4338);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4339);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4340);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7944);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4341);
							match s {
								[] => true,
								['o', 'r', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'e', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 4342);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7946);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4343);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4344);
							match s {
								[] => true,
								['a', 'm', 'e', s @ ..] => {
									super::push(out, 7949);
									match s {
										[] => true,
										['h', 't', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11748);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 4345);
							match s {
								[] => true,
								['r', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1903);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4346);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7951);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1904);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 7952);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11749);
											match s {
												[] => true,
												['q', 's', 'e', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 1907);
					match s {
						[] => true,
						['c', 'i', 'm', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4348);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4350);
							match s {
								[] => true,
								['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7954);
									s.is_empty()
								}
								['h', 'r', 'r', 'o', 'm', 'e', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7955);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', 'u', 'c', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'n', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t3::n722(s, out),
		['h', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1915);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n724(s, out),
		['m', s @ ..] => super::t3::n725(s, out),
		['n', s @ ..] => super::t3::n726(s, out),
		['p', s @ ..] => super::t3::n727(s, out),
		['r', s @ ..] => super::t3::n728(s, out),
		['s', s @ ..] => super::t3::n729(s, out),
		['t', s @ ..] => super::t3::n730(s, out),
		['u', s @ ..] => super::t3::n731(s, out),
		['v', s @ ..] => super::t3::n732(s, out),
		['x', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilis.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['b', s @ ..] => super::t3::n736(s, out),
		['c', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['a', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', 'f', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 4516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8132);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 666);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4517);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8133);
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
							super::push(out, 4518);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4519);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 668);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1993);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1994);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4520);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4521);
							s.is_empty()
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
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4522);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4523);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
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
						['u', 'c', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1999);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 4524);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 8134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2001);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t3::n745(s, out),
		['v', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						['l', s @ ..] => super::t3::n5590(s, out),
						['n', s @ ..] => {
							super::push(out, 4545);
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

/// Words ending -formis.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2020);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4553);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8153);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4554);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8156);
					match s {
						[] => true,
						['a', 'n', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8157);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8158);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8159);
					match s {
						[] => true,
						['c', 'a', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8160);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11898);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 11899);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 14883);
									s.is_empty()
								}
								['u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 14884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11900);
							match s {
								[] => true,
								['b', 'a', 't', 'e', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8161);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11901);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11902);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8162);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11904);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 14886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11905);
							match s {
								[] => true,
								['h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 14887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8163);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11907);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 14888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11908);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14889);
									match s {
										[] => true,
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 16799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11910);
							match s {
								[] => true,
								['l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8164);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11911);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8165);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 11913);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8166);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8167);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11914);
							s.is_empty()
						}
						['i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 11915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4555);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8169);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8170);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word claripennis.
pub(crate) fn n26004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14892);
	s.is_empty()
}

/// The word obscuripennis.
pub(crate) fn n26005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14893);
	s.is_empty()
}

/// Words ending -uennis.
pub(crate) fn n2278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2038);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 4567);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -innis.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['p', 'i', 'v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pnis.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['m', 'e', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnis.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['o', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4569);
							match s {
								[] => true,
								['g', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8177);
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
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4570);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'n', 'e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4571);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8179);
									match s {
										[] => true,
										['l', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'r', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -unis.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word subnuclearis.
pub(crate) fn n26006(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -liaris.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2043);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4574);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -laris.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8182);
							match s {
								[] => true,
								['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['c', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4579);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2047);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4580);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 11930);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8184);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8185);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8186);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11932);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 8187);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 8188);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 14894);
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
		['o', s @ ..] => {
			super::push(out, 2048);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2049);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4583);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8189);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 11934);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 11935);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11936);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14895);
											match s {
												[] => true,
												['c', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t3::n11554(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t3::n5646(s, out),
				['g', s @ ..] => super::t3::n5647(s, out),
				['l', s @ ..] => super::t3::n5648(s, out),
				['m', s @ ..] => super::t3::n5649(s, out),
				['n', s @ ..] => super::t3::n5650(s, out),
				['p', s @ ..] => super::t3::n5651(s, out),
				['s', s @ ..] => super::t3::n5652(s, out),
				['t', s @ ..] => super::t3::n5653(s, out),
				['v', s @ ..] => super::t3::n5654(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -naris.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2053);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2054);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4593);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4595);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2056);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -taris.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 704);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4596);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2059);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bris.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 2061);
					match s {
						[] => true,
						['b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4599);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 708);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eris.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['a', 'h', 'c', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oris.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 712);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tris.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['e', 'v', 'i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4601);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', s @ ..] => {
							super::push(out, 4602);
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8200);
									s.is_empty()
								}
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
			super::push(out, 716);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4603);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8201);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11951);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4604);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 8202);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 11952);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4605);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8203);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 4606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 8205);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11953);
											match s {
												[] => true,
												['l', '\u{1}', s @ ..] => {
													super::push(out, 14900);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 11954);
											match s {
												[] => true,
												['c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 14901);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 8206);
									match s {
										[] => true,
										['a', 'l', s @ ..] => {
											super::push(out, 11955);
											match s {
												[] => true,
												['f', '\u{1}', s @ ..] => {
													super::push(out, 14902);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 11956);
											match s {
												[] => true,
												['l', 'c', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 14903);
													s.is_empty()
												}
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
				['u', s @ ..] => {
					super::push(out, 2070);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nsis.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['e', s @ ..] => super::t3::n801(s, out),
		_ => false,
	}
}

/// Words ending -secubans.
pub(crate) fn n11692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8290);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocubans.
pub(crate) fn n5764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4674);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word excubans.
pub(crate) fn n5765(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nubans.
pub(crate) fn n2370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2102);
	s.is_empty()
}

/// Words ending -cans.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2103);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2104);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 729);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4675);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2108);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2110);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8291);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4679);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8292);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8293);
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4680);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8295);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4681);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4682);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n824(s, out),
		['l', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4717);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8336);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['n', s @ ..] => super::t3::n826(s, out),
		['o', s @ ..] => super::t3::n827(s, out),
		['r', s @ ..] => super::t3::n828(s, out),
		['s', s @ ..] => super::t3::n829(s, out),
		['u', s @ ..] => super::t3::n830(s, out),
		_ => false,
	}
}

/// Words ending -dans.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2134);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8341);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2135);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2137);
					match s {
						[] => true,
						['e', 'h', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4733);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 741);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 8342);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8343);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 2140);
					match s {
						[] => true,
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['i', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8344);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2143);
					match s {
						[] => true,
						['r', 'o', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4734);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 12075);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4735);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2145);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2146);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 742);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4736);
							match s {
								[] => true,
								['r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4737);
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
			super::push(out, 743);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n2424(s, out),
				['e', s @ ..] => super::t3::n2425(s, out),
				['u', s @ ..] => super::t3::n2426(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t3::n837(s, out),
		['r', s @ ..] => super::t3::n838(s, out),
		['u', s @ ..] => super::t3::n839(s, out),
		['y', s @ ..] => super::t3::n840(s, out),
		_ => false,
	}
}

/// Words ending -eans.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['l', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2158);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8354);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4750);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4751);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8355);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 8356);
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
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['q', 'a', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fans.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gans.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2164);
					match s {
						[] => true,
						['f', 'u', s @ ..] => {
							super::push(out, 4752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4753);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4755);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 756);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4756);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 4757);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8357);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 8358);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8359);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8360);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12079);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 12080);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4759);
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
					super::push(out, 2168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4760);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8361);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 757);
			match s {
				[] => true,
				['l', s @ ..] => super::t3::n2457(s, out),
				['m', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4766);
							match s {
								[] => true,
								['r', s @ ..] => super::t3::n11877(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n2459(s, out),
				['t', s @ ..] => super::t3::n2460(s, out),
				['v', s @ ..] => super::t3::n2461(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n857(s, out),
		['n', s @ ..] => super::t3::n858(s, out),
		['o', s @ ..] => super::t3::n859(s, out),
		['r', s @ ..] => super::t3::n860(s, out),
		['u', s @ ..] => super::t3::n861(s, out),
		_ => false,
	}
}

/// Words ending -hans.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['c', 'c', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4789);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ians.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['l', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8375);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2185);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8377);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4790);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 12084);
											match s {
												[] => true,
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4791);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 765);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2189);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4794);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				['a', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['c', 'n', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4797);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['f', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 12085);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'm', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['o', 'r', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8378);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2199);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => super::t3::n2498(s, out),
				['o', 'r', 'p', s @ ..] => super::t3::n2499(s, out),
				['x', 'e', '\u{1}', s @ ..] => super::t3::n2500(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n870(s, out),
		['s', s @ ..] => super::t3::n871(s, out),
		['t', s @ ..] => super::t3::n872(s, out),
		['v', s @ ..] => super::t3::n873(s, out),
		_ => false,
	}
}

/// Words ending -lans.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
						['c', s @ ..] => {
							super::push(out, 4810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 12088);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4812);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8381);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 8382);
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
		['i', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4813);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2230);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2231);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8383);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => super::t3::n11964(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t3::n2538(s, out),
				['m', s @ ..] => super::t3::n2539(s, out),
				['p', s @ ..] => super::t3::n2540(s, out),
				['t', s @ ..] => super::t3::n2541(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n879(s, out),
		['o', s @ ..] => super::t3::n880(s, out),
		['p', s @ ..] => super::t3::n881(s, out),
		['u', s @ ..] => super::t3::n882(s, out),
		_ => false,
	}
}

/// Words ending -mans.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2258);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4872);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4873);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2261);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2262);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8402);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 12100);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8403);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2263);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2266);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 786);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2267);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4874);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4875);
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
				['r', s @ ..] => {
					super::push(out, 2269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4877);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 2270);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2271);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4880);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8404);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2272);
					match s {
						[] => true,
						['o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => super::t3::n12057(s, out),
								['p', 'a', '\u{1}', s @ ..] => super::t3::n12058(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n886(s, out),
		['m', s @ ..] => super::t3::n887(s, out),
		['o', s @ ..] => super::t3::n888(s, out),
		['r', s @ ..] => super::t3::n889(s, out),
		['s', s @ ..] => super::t3::n890(s, out),
		['u', s @ ..] => super::t3::n891(s, out),
		_ => false,
	}
}

/// Words ending -nans.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8414);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8415);
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
						['i', s @ ..] => {
							super::push(out, 4883);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4884);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 794);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8416);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4885);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 8417);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => super::t3::n12093(s, out),
								['n', s @ ..] => super::t3::n12094(s, out),
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t3::n2610(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n897(s, out),
		['m', s @ ..] => super::t3::n898(s, out),
		['n', s @ ..] => super::t3::n899(s, out),
		['o', s @ ..] => super::t3::n900(s, out),
		['r', s @ ..] => super::t3::n901(s, out),
		['u', s @ ..] => super::t3::n902(s, out),
		_ => false,
	}
}

/// Words ending -oans.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pans.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['r', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4934);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4935);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8462);
									match s {
										[] => true,
										['m', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 14942);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n31820(s, out),
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
									super::push(out, 8463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2320);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8464);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 805);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2321);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2322);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4936);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8465);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 808);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['c', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rans.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8466);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4939);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8467);
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
						['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['l', 'e', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8468);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 4940);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8469);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['c', 'u', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8470);
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
		['c', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8471);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8472);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2331);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 4945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n914(s, out),
		['g', s @ ..] => super::t4::n915(s, out),
		['i', s @ ..] => super::t4::n916(s, out),
		['o', s @ ..] => super::t4::n917(s, out),
		['p', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n919(s, out),
		['t', s @ ..] => super::t4::n920(s, out),
		['u', s @ ..] => super::t4::n921(s, out),
		['y', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['g', s @ ..] => super::t4::n2724(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sans.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5002);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2391);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5003);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8513);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 8514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'e', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2393);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8515);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 825);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5005);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8516);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 8517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'g', 'r', 'e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2396);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 828);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2398);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5006);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', s @ ..] => {
					super::push(out, 2399);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tans.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2402);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5009);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5010);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 12151);
											match s {
												[] => true,
												['i', 'r', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5015);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5016);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 14946);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31824(s, out),
														_ => false,
													}
												}
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t4::n2744(s, out),
				['i', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => super::t4::n12406(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t4::n6466(s, out),
						['n', s @ ..] => super::t4::n6467(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t4::n2746(s, out),
				['o', s @ ..] => super::t4::n2747(s, out),
				['r', s @ ..] => super::t4::n2748(s, out),
				['u', s @ ..] => super::t4::n2749(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n933(s, out),
		['i', s @ ..] => super::t4::n934(s, out),
		['l', s @ ..] => super::t4::n935(s, out),
		['n', s @ ..] => super::t4::n936(s, out),
		['o', s @ ..] => super::t4::n937(s, out),
		['p', s @ ..] => super::t4::n938(s, out),
		['r', s @ ..] => super::t4::n939(s, out),
		['s', s @ ..] => super::t4::n940(s, out),
		['u', s @ ..] => super::t4::n941(s, out),
		_ => false,
	}
}

/// Words ending -uans.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['v', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 842);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 843);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2467);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 5109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8578);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2469);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 8579);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 8580);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12173);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 12174);
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
								['l', s @ ..] => {
									super::push(out, 8581);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2470);
					match s {
						[] => true,
						['i', 'p', 'o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 847);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2471);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8582);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2472);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5114);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['e', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t4::n2828(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vans.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2480);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5117);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5119);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8584);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8585);
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
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5121);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5124);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8586);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5125);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8588);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 855);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5126);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5127);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -xans.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2484);
					match s {
						[] => true,
						['p', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5128);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 858);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zans.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8589);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2490);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2491);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5129);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2492);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2494);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5132);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5133);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5134);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 8591);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2495);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5136);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8593);
									match s {
										[] => true,
										['b', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14951);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 14952);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31834(s, out),
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
									super::push(out, 8594);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 12175);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5137);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8595);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8596);
									s.is_empty()
								}
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

/// Words ending -ibens.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 861);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5144);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2504);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5145);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2506);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5146);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 12176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 12177);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12178);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 14953);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 12179);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 12180);
									match s {
										[] => true,
										['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 12181);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14954);
											match s {
												[] => true,
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 14955);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 14956);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 12182);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 12183);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['p', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n35173(s, out),
														['u', 's', '\u{1}', s @ ..] => super::t4::n35174(s, out),
														_ => false,
													}
												}
												['t', 'n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 12184);
									match s {
										[] => true,
										['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 12185);
									match s {
										[] => true,
										['s', 'o', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 12186);
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
						['x', s @ ..] => {
							super::push(out, 8600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lambens.
pub(crate) fn n6710(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8601);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umbens.
pub(crate) fn n2866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2508);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8602);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8603);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12187);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rbens.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['o', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubens.
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5148);
					match s {
						[] => true,
						['d', 'i', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8605);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5150);
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
		['r', s @ ..] => {
			super::push(out, 2512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cens.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2514);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8606);
									match s {
										[] => true,
										['n', 'e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2515);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5151);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8607);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5153);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5154);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5155);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2516);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2518);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5156);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8609);
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
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t4::n973(s, out),
		['o', s @ ..] => super::t4::n974(s, out),
		['r', s @ ..] => super::t4::n975(s, out),
		['s', s @ ..] => super::t4::n976(s, out),
		['u', s @ ..] => super::t4::n977(s, out),
		_ => false,
	}
}

/// Words ending -dens.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2528);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2531);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8693);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5210);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5211);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5212);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5213);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 877);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2533);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5214);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5215);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5216);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5217);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5218);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t4::n2908(s, out),
				['m', s @ ..] => super::t4::n2909(s, out),
				['p', s @ ..] => super::t4::n2910(s, out),
				['r', s @ ..] => super::t4::n2911(s, out),
				['s', s @ ..] => super::t4::n2912(s, out),
				['x', s @ ..] => super::t4::n2913(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n982(s, out),
		['n', s @ ..] => super::t4::n983(s, out),
		['o', s @ ..] => super::t4::n984(s, out),
		['r', s @ ..] => super::t4::n985(s, out),
		['u', s @ ..] => super::t4::n986(s, out),
		_ => false,
	}
}

/// Words ending -gens.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5275);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8736);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8737);
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5276);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5277);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2571);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5279);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8738);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 8739);
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
							super::push(out, 5280);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5281);
							match s {
								[] => true,
								['c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5282);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8740);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t4::n19925(s, out),
										_ => false,
									}
								}
								['o', s @ ..] => super::t4::n13023(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t4::n6976(s, out),
						['s', s @ ..] => super::t4::n6977(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n2954(s, out),
				['p', s @ ..] => super::t4::n2955(s, out),
				['r', s @ ..] => super::t4::n2956(s, out),
				['s', s @ ..] => super::t4::n2957(s, out),
				['v', s @ ..] => super::t4::n2958(s, out),
				['x', s @ ..] => super::t4::n2959(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t4::n990(s, out),
		['n', s @ ..] => super::t4::n991(s, out),
		['o', s @ ..] => super::t4::n992(s, out),
		['r', s @ ..] => super::t4::n993(s, out),
		['u', s @ ..] => super::t4::n994(s, out),
		_ => false,
	}
}

/// Words ending -hens.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5312);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8770);
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5313);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8771);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8772);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 8773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8774);
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5316);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5317);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5318);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5319);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -iens.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5320);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t4::n1000(s, out),
		['d', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8799);
									match s {
										[] => true,
										['r', 't', 'e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2598);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5340);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5341);
							match s {
								[] => true,
								['b', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5342);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 12312);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 15020);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 15021);
													match s {
														[] => true,
														['r', 'p', '\u{1}', s @ ..] => super::t4::n31943(s, out),
														_ => false,
													}
												}
												['d', s @ ..] => {
													super::push(out, 15022);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31944(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 15023);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31945(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 15024);
													match s {
														[] => true,
														['n', 'a', '\u{1}', s @ ..] => super::t4::n31946(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['g', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 12313);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 12314);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 12315);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 12316);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12317);
											s.is_empty()
										}
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
					super::push(out, 2599);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2600);
					match s {
						[] => true,
						['a', 'l', 'b', s @ ..] => super::t4::n7090(s, out),
						['o', 'c', '\u{1}', s @ ..] => super::t4::n7091(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n2993(s, out),
				['r', s @ ..] => super::t4::n2994(s, out),
				['u', s @ ..] => super::t4::n2995(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n1002(s, out),
		['f', s @ ..] => super::t4::n1003(s, out),
		['g', s @ ..] => super::t4::n1004(s, out),
		['l', s @ ..] => super::t4::n1005(s, out),
		['m', s @ ..] => super::t4::n1006(s, out),
		['n', s @ ..] => super::t4::n1007(s, out),
		['o', s @ ..] => super::t4::n1008(s, out),
		['p', s @ ..] => super::t4::n1009(s, out),
		['r', s @ ..] => super::t4::n1010(s, out),
		['s', s @ ..] => super::t4::n1011(s, out),
		['t', s @ ..] => super::t4::n1012(s, out),
		['u', s @ ..] => super::t4::n1013(s, out),
		['v', s @ ..] => super::t4::n1014(s, out),
		['x', 'e', '\u{1}', s @ ..] => super::t4::n1015(s, out),
		_ => false,
	}
}

/// Words ending -lens.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2649);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 913);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2652);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
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
						['c', s @ ..] => {
							super::push(out, 5387);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8869);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12352);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 12353);
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
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8870);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8871);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => super::t4::n7201(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n3064(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n1020(s, out),
		['p', s @ ..] => super::t4::n1021(s, out),
		['u', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mens.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2666);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2667);
					match s {
						[] => true,
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8880);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 'e', s @ ..] => {
					super::push(out, 2669);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5393);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
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
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2671);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8881);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2673);
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
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5397);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5398);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8883);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 8884);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8885);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5399);
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
					super::push(out, 2674);
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

/// Words ending -nens.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2675);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5400);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8886);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8887);
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
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['i', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5401);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 927);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2677);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5402);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2679);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 8888);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5404);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5406);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5407);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8889);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n1033(s, out),
		['r', s @ ..] => super::t4::n1034(s, out),
		_ => false,
	}
}

/// Words ending -oens.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

