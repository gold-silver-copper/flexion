use flexion::Path;

/// Words ending -tlich.
pub(crate) fn n1957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1569);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2672);
			match s {
				[] => true,
				['a', 't', 's', 'n', s @ ..] => {
					super::push(out, 3770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5084);
									s.is_empty()
								}
								['n', 'u', s @ ..] => {
									super::push(out, 5085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4565);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4566);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5297);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 5298);
											s.is_empty()
										}
										['s', 's', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 5299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4567);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5086);
									s.is_empty()
								}
								['r', 'i', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5300);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 5301);
											s.is_empty()
										}
										['t', 'k', s @ ..] => {
											super::push(out, 5302);
											s.is_empty()
										}
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
					super::push(out, 3771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2673);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4568);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4569);
							match s {
								[] => true,
								['r', 'r', s @ ..] => {
									super::push(out, 5088);
									match s {
										[] => true,
										['e', 'd', s @ ..] => {
											super::push(out, 5303);
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
								['h', s @ ..] => {
									super::push(out, 5089);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5090);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5304);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 5305);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5306);
											s.is_empty()
										}
										['s', 'u', 'a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 5307);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 4570);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 5308);
											s.is_empty()
										}
										['t', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5309);
											s.is_empty()
										}
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
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4571);
							match s {
								[] => true,
								['d', 'n', 'u', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 5091);
									s.is_empty()
								}
								['o', 'h', s @ ..] => {
									match s {
										[] => true,
										['-', 't', 'h', 'c', 'i', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 5310);
											s.is_empty()
										}
										['t', 'h', 'c', 'i', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 5311);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'h', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5312);
											s.is_empty()
										}
										['e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4572);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 5092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'n', 'ü', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2674);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4573);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5093);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5094);
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
						['\u{1}', s @ ..] => {
							super::push(out, 4574);
							s.is_empty()
						}
						['h', 'r', 'e', 'n', s @ ..] => {
							super::push(out, 4575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2675);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2676);
			match s {
				[] => true,
				['e', s @ ..] => super::t2::n4720(s, out),
				['n', 'e', 'k', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['w', 't', 'n', 'a', 'r', 'e', 'v', 't', s @ ..] => {
							super::push(out, 4579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 3777);
					s.is_empty()
				}
				['ö', s @ ..] => super::t2::n4725(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t2::n3394(s, out),
		['t', s @ ..] => super::t2::n3395(s, out),
		['u', s @ ..] => super::t2::n3396(s, out),
		['ö', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2679);
			s.is_empty()
		}
		['ü', s @ ..] => super::t2::n3398(s, out),
		_ => false,
	}
}

/// Words ending -ulich.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1570);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3787);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', 'n', 'a', 't', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zlich.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2681);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				['ü', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'r', 'e', 'v', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						['s', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5105);
									s.is_empty()
								}
								['r', 'e', 'd', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 's', s @ ..] => {
					match s {
						[] => true,
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4589);
							s.is_empty()
						}
						['u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 3791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßlich.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2682);
			s.is_empty()
		}
		['e', 'i', s @ ..] => {
			match s {
				[] => true,
				['l', 'h', 'c', 's', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3792);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2683);
			s.is_empty()
		}
		['ö', 't', 's', 'm', 'u', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 2684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gisch.
pub(crate) fn n1980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1587);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3815);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3817);
					s.is_empty()
				}
				['t', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 3818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2709);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3819);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2710);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2711);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3821);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5111);
									s.is_empty()
								}
								['d', 'ä', 'p', 'r', 'e', 'd', 'n', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 5112);
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
							super::push(out, 4600);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4601);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 't', s @ ..] => {
									super::push(out, 5113);
									s.is_empty()
								}
								['h', 't', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5114);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5115);
									s.is_empty()
								}
								['k', 'ö', '\u{1}', s @ ..] => {
									super::push(out, 5116);
									s.is_empty()
								}
								['l', 'i', s @ ..] => {
									super::push(out, 5117);
									s.is_empty()
								}
								['m', 'l', 'a', s @ ..] => {
									super::push(out, 5118);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5119);
									s.is_empty()
								}
								['p', 'o', 'r', 'h', s @ ..] => {
									super::push(out, 5120);
									s.is_empty()
								}
								['t', 'a', 'h', s @ ..] => {
									super::push(out, 5121);
									s.is_empty()
								}
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
			super::push(out, 2712);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 3822);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5122);
									s.is_empty()
								}
								['n', 'e', 't', s @ ..] => {
									super::push(out, 5123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
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
				['o', 'e', 'g', 't', s @ ..] => {
					super::push(out, 3823);
					s.is_empty()
				}
				['u', 'b', s @ ..] => {
					match s {
						[] => true,
						['i', 'e', 'r', s @ ..] => {
							super::push(out, 4604);
							s.is_empty()
						}
						['m', 'a', s @ ..] => {
							super::push(out, 4605);
							s.is_empty()
						}
						['n', 'e', 'u', 'e', s @ ..] => {
							super::push(out, 4606);
							s.is_empty()
						}
						['s', 'n', 'e', 'g', s @ ..] => {
							super::push(out, 4607);
							s.is_empty()
						}
						['z', s @ ..] => {
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

/// Words ending -hisch.
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2713);
			match s {
				[] => true,
				['e', 'i', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3824);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3825);
					match s {
						[] => true,
						['e', 'r', 'r', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5124);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5125);
									match s {
										[] => true,
										['s', 'ö', '\u{1}', s @ ..] => {
											super::push(out, 5318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 5126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3826);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2714);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2715);
			match s {
				[] => true,
				['a', 'r', 'g', 'o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4609);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5127);
									s.is_empty()
								}
								['d', 'i', '\u{1}', s @ ..] => {
									super::push(out, 5128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'r', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', 'o', 'l', 'i', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3829);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2716);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4611);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4612);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4613);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5129);
									s.is_empty()
								}
								['i', 'z', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 5130);
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
						['\u{1}', s @ ..] => {
							super::push(out, 4614);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4615);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4616);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3831);
					s.is_empty()
				}
				['n', 'i', 'r', 'y', 'b', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3832);
					s.is_empty()
				}
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4618);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 4619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kisch.
pub(crate) fn n1983(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 's', s @ ..] => {
			super::push(out, 2718);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2719);
			match s {
				[] => true,
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 3836);
					s.is_empty()
				}
				['r', 'ü', 't', '\u{1}', s @ ..] => {
					super::push(out, 3837);
					s.is_empty()
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['r', 's', 'n', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4620);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 't', 'z', s @ ..] => {
			super::push(out, 2720);
			s.is_empty()
		}
		['i', 'h', s @ ..] => {
			super::push(out, 2721);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2722);
			s.is_empty()
		}
		['n', 'ä', s @ ..] => {
			match s {
				[] => true,
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3838);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'ü', 't', 't', s @ ..] => {
			super::push(out, 2723);
			s.is_empty()
		}
		['s', 'a', 'b', s @ ..] => {
			super::push(out, 2724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lisch.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2725);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3840);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3841);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5319);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 5320);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 5131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						['s', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3842);
					s.is_empty()
				}
				['n', 'r', 'e', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3843);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4624);
							s.is_empty()
						}
						['t', 'a', 'e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 4625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', 'm', 'i', 't', 'n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3844);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2726);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3846);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3847);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2727);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2728);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', s @ ..] => {
			super::push(out, 2729);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'y', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2730);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3851);
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'b', s @ ..] => {
					super::push(out, 3852);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3853);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2731);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', s @ ..] => {
							super::push(out, 4627);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4630);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 5132);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 5133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3855);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2732);
			s.is_empty()
		}
		['y', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 2733);
			s.is_empty()
		}
		['ä', 'f', 't', 's', 'e', 'w', '-', s @ ..] => {
			super::push(out, 2734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -misch.
pub(crate) fn n1985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1589);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3857);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3858);
					s.is_empty()
				}
				['l', 's', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4631);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'y', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4633);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['d', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5134);
									s.is_empty()
								}
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2736);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 3860);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3861);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 3862);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4634);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 3864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2737);
			s.is_empty()
		}
		['l', 'e', s @ ..] => {
			super::push(out, 2738);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3865);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4636);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 5136);
									s.is_empty()
								}
								['k', 'ö', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5321);
											s.is_empty()
										}
										['o', 'e', s @ ..] => {
											super::push(out, 5322);
											s.is_empty()
										}
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
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3866);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2739);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2740);
			s.is_empty()
		}
		['ä', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3868);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2741);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nisch.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1590);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2742);
			match s {
				[] => true,
				['b', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				['e', 'r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3872);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4637);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 4639);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4641);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 4642);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4643);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4644);
							s.is_empty()
						}
						['z', 'u', s @ ..] => {
							super::push(out, 4645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3873);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', 'b', s @ ..] => {
									super::push(out, 5137);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5323);
											match s {
												[] => true,
												['m', 'a', s @ ..] => {
													match s {
														[] => true,
														['-', 's', 'u', '\u{1}', s @ ..] => super::t2::n6710(s, out),
														['d', s @ ..] => super::t2::n6711(s, out),
														['n', 'i', 'e', 't', 'a', 'l', '\u{1}', s @ ..] => super::t2::n6712(s, out),
														['o', 'r', 'f', 'a', '\u{1}', s @ ..] => super::t2::n6713(s, out),
														['r', s @ ..] => super::t2::n6714(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 5324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 5138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5139);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 5140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3874);
					match s {
						[] => true,
						['s', 't', s @ ..] => {
							super::push(out, 4648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', s @ ..] => {
					super::push(out, 3875);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4649);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5141);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4650);
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
					super::push(out, 3876);
					s.is_empty()
				}
				['i', 'g', s @ ..] => {
					super::push(out, 3877);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3878);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3880);
					s.is_empty()
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3882);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2743);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
					super::push(out, 3884);
					s.is_empty()
				}
				['d', 'r', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3886);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4651);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5325);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 5326);
											s.is_empty()
										}
										['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 5327);
											s.is_empty()
										}
										['l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5328);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5329);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 5330);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5331);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4652);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3887);
					match s {
						[] => true,
						['l', 'y', 'b', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4654);
							s.is_empty()
						}
						['u', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', 'e', 'n', 'e', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3888);
					s.is_empty()
				}
				['s', 's', 'e', 't', s @ ..] => {
					super::push(out, 3889);
					s.is_empty()
				}
				['t', 'n', 'e', 'r', 'o', s @ ..] => {
					super::push(out, 3890);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3891);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3892);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', s @ ..] => {
			super::push(out, 2744);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2745);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => super::t2::n5794(s, out),
						['r', 'y', 't', '\u{1}', s @ ..] => super::t2::n5795(s, out),
						['t', 'i', 'r', 'b', '\u{1}', s @ ..] => super::t2::n5796(s, out),
						_ => false,
					}
				}
				['ä', 'm', s @ ..] => super::t2::n4871(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n3485(s, out),
		['r', s @ ..] => super::t2::n3486(s, out),
		['u', 'a', s @ ..] => super::t2::n3487(s, out),
		['y', s @ ..] => super::t2::n3488(s, out),
		['ä', 'd', '\u{1}', s @ ..] => super::t2::n3489(s, out),
		_ => false,
	}
}

/// Words ending -pisch.
pub(crate) fn n1988(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2753);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2754);
			match s {
				[] => true,
				['k', 's', 'o', 'r', 'k', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3903);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3904);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4674);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2755);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2756);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4676);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4677);
							s.is_empty()
						}
						['e', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4678);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4679);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5147);
									s.is_empty()
								}
								['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'n', 'u', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4680);
							s.is_empty()
						}
						['t', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -risch.
pub(crate) fn n1989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1591);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2757);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3906);
					s.is_empty()
				}
				['d', 'i', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				['g', 'n', s @ ..] => {
					super::push(out, 3908);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3909);
					match s {
						[] => true,
						['c', 's', 'd', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4682);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4683);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4684);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 5149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'i', 'p', s @ ..] => {
					super::push(out, 3910);
					s.is_empty()
				}
				['m', 'm', s @ ..] => {
					super::push(out, 3911);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'u', s @ ..] => {
							super::push(out, 4685);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3912);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4687);
							s.is_empty()
						}
						['g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4688);
							s.is_empty()
						}
						['ä', '\u{1}', s @ ..] => {
							super::push(out, 4689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3913);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 4690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2758);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2759);
			match s {
				[] => true,
				['n', 'o', s @ ..] => {
					super::push(out, 3915);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2760);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'z', 't', 'e', 's', 'e', 'g', '-', 'i', 's', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 4691);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							super::push(out, 4693);
							s.is_empty()
						}
						['n', 'ä', 'h', 'u', 'e', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4694);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3918);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3919);
					match s {
						[] => true,
						['n', 'ä', 'g', 'l', s @ ..] => {
							super::push(out, 4696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4697);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'z', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5150);
									s.is_empty()
								}
								['s', 'l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3920);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4699);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4700);
							s.is_empty()
						}
						['e', 'i', 'p', 's', 'u', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4701);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 5332);
											s.is_empty()
										}
										['u', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5333);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', 'z', 'n', 'e', 's', 'b', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4702);
							s.is_empty()
						}
						['o', 'r', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 4703);
							s.is_empty()
						}
						['t', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4705);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4706);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5154);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 4707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4708);
							s.is_empty()
						}
						['g', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4709);
							s.is_empty()
						}
						['h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4710);
							s.is_empty()
						}
						['t', 'n', 'r', 'ä', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4712);
							s.is_empty()
						}
						['ä', 'l', 'k', 'f', 'u', 'a', 'n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4713);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3921);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4715);
							match s {
								[] => true,
								['w', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 5334);
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											super::push(out, 5335);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4716);
							match s {
								[] => true,
								['e', 'i', 'n', 'z', 't', 'u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3922);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						['l', 'a', 't', s @ ..] => {
							super::push(out, 4719);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4721);
							s.is_empty()
						}
						['s', 'y', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['ä', 'r', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'm', 'h', 'e', 'n', 'r', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3923);
					s.is_empty()
				}
				['y', 'a', 'b', 'r', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				['z', s @ ..] => super::t2::n4915(s, out),
				['ß', 'e', 'i', 'n', 'z', 't', 'u', 'n', '\u{1}', s @ ..] => super::t2::n4916(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t2::n3500(s, out),
		['g', s @ ..] => super::t2::n3501(s, out),
		['h', s @ ..] => super::t2::n3502(s, out),
		['i', s @ ..] => super::t2::n3503(s, out),
		['m', s @ ..] => super::t2::n3504(s, out),
		['o', s @ ..] => super::t2::n3505(s, out),
		['r', s @ ..] => super::t2::n3506(s, out),
		['t', s @ ..] => super::t2::n3507(s, out),
		['u', s @ ..] => super::t2::n3508(s, out),
		['y', s @ ..] => super::t2::n3509(s, out),
		['ä', s @ ..] => super::t2::n3510(s, out),
		_ => false,
	}
}

/// Words ending -sisch.
pub(crate) fn n1990(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3943);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3944);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					super::push(out, 3945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3946);
					match s {
						[] => true,
						['d', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4736);
							s.is_empty()
						}
						['r', 'f', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3947);
					s.is_empty()
				}
				['m', 'a', 'n', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4738);
							s.is_empty()
						}
						['i', 'h', s @ ..] => {
							super::push(out, 4739);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5347);
											s.is_empty()
										}
										['t', 's', 'o', 'd', 'r', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 5348);
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
									super::push(out, 5167);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 5168);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3949);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2771);
			match s {
				[] => true,
				['c', 'ä', 's', 'r', 'e', 'b', s @ ..] => {
					super::push(out, 3951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2772);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3952);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2773);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2774);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						['u', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4741);
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
					match s {
						[] => true,
						['g', 'e', 'n', s @ ..] => {
							super::push(out, 4742);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 5170);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 5171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4743);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 4744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', s @ ..] => {
			super::push(out, 2775);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2776);
			match s {
				[] => true,
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tisch.
pub(crate) fn n1991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1592);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3955);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3956);
					s.is_empty()
				}
				['h', 'p', 'm', 'e', s @ ..] => {
					super::push(out, 3957);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3958);
					s.is_empty()
				}
				['k', 'i', s @ ..] => {
					super::push(out, 3959);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3960);
					match s {
						[] => true,
						['a', 'r', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5172);
									s.is_empty()
								}
								['o', 'l', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 5349);
											s.is_empty()
										}
										['t', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5350);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 5174);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5175);
									s.is_empty()
								}
								['t', 's', 'y', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5351);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 5352);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5176);
									s.is_empty()
								}
								['e', 'l', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5177);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5178);
									s.is_empty()
								}
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 'r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5180);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', 'i', '\u{1}', s @ ..] => {
									super::push(out, 5182);
									s.is_empty()
								}
								['l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5183);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 5353);
											s.is_empty()
										}
										['h', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5184);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['p', 'm', 'y', 's', '\u{1}', s @ ..] => {
											super::push(out, 5355);
											s.is_empty()
										}
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4745);
							match s {
								[] => true,
								['i', 'r', 'a', s @ ..] => {
									super::push(out, 5185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 5186);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							super::push(out, 4747);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3961);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4749);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5188);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5357);
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													match s {
														[] => true,
														['l', 'a', 'i', 'z', 'o', 's', '\u{1}', s @ ..] => super::t2::n6715(s, out),
														['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => super::t2::n6716(s, out),
														['t', 's', 'o', 'p', '\u{1}', s @ ..] => super::t2::n6717(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', 'ü', s @ ..] => {
											super::push(out, 5358);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 5421);
													s.is_empty()
												}
												['u', 'a', s @ ..] => {
													super::push(out, 5422);
													s.is_empty()
												}
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
							super::push(out, 4751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4752);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4753);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2778);
			match s {
				[] => true,
				['ä', 't', 's', 'r', s @ ..] => {
					super::push(out, 3962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n3523(s, out),
		['h', s @ ..] => {
			super::push(out, 2780);
			s.is_empty()
		}
		['i', s @ ..] => super::t2::n3525(s, out),
		['k', s @ ..] => super::t2::n3526(s, out),
		['l', s @ ..] => super::t2::n3527(s, out),
		['n', s @ ..] => super::t2::n3528(s, out),
		['o', s @ ..] => super::t2::n3529(s, out),
		['p', s @ ..] => super::t2::n3530(s, out),
		['r', 'a', s @ ..] => super::t2::n3531(s, out),
		['s', s @ ..] => super::t2::n3532(s, out),
		['t', 'ö', s @ ..] => super::t2::n3533(s, out),
		['u', s @ ..] => super::t2::n3534(s, out),
		['y', s @ ..] => super::t2::n3535(s, out),
		['ä', s @ ..] => super::t2::n3536(s, out),
		_ => false,
	}
}

/// Words ending -auisch.
pub(crate) fn n3537(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4006);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4007);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -visch.
pub(crate) fn n1993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1594);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2790);
			match s {
				[] => true,
				['l', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 4009);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wisch.
pub(crate) fn n1994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1595);
	match s {
		[] => true,
		['a', 'l', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				['t', 's', 'o', 't', s @ ..] => {
					super::push(out, 4011);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -toxisch.
pub(crate) fn n1995(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2791);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2792);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zisch.
pub(crate) fn n1996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1596);
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
			super::push(out, 2794);
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßisch.
pub(crate) fn n1997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	s.is_empty()
}

/// Words ending -ropäisch.
pub(crate) fn n1998(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2795);
			s.is_empty()
		}
		['u', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				['d', 'r', s @ ..] => {
					super::push(out, 4014);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4015);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 4016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -röisch.
pub(crate) fn n1999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1598);
	s.is_empty()
}

/// Words ending -asiert.
pub(crate) fn n4113(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 's', s @ ..] => {
			super::push(out, 4269);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4270);
			s.is_empty()
		}
		['r', 'n', s @ ..] => {
			super::push(out, 4271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -isiert.
pub(crate) fn n4114(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4272);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4273);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4942);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
					super::push(out, 4944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4274);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4275);
			match s {
				[] => true,
				['a', 't', 'i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssiert.
pub(crate) fn n4116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3271);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4277);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 4946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tiert.
pub(crate) fn n2706(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3272);
			s.is_empty()
		}
		['k', 'e', s @ ..] => {
			super::push(out, 3273);
			s.is_empty()
		}
		['l', 'a', 'x', s @ ..] => {
			super::push(out, 3274);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'o', s @ ..] => {
							match s {
								[] => true,
								['s', 'i', s @ ..] => {
									super::push(out, 5269);
									s.is_empty()
								}
								['t', 'k', s @ ..] => {
									super::push(out, 5270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						['m', 'g', s @ ..] => {
							super::push(out, 4948);
							match s {
								[] => true,
								['a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 5271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3275);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3276);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['v', 'n', s @ ..] => {
							super::push(out, 4949);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3277);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3278);
			match s {
				[] => true,
				['k', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4279);
					s.is_empty()
				}
				['p', 'm', 'a', 'n', 'r', 'i', 'h', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uiert.
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 3279);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 3280);
			s.is_empty()
		}
		['t', 'i', s @ ..] => {
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4281);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viert.
pub(crate) fn n2708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['i', 't', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4283);
					match s {
						[] => true,
						['a', 'e', s @ ..] => {
							super::push(out, 4951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4284);
					s.is_empty()
				}
				['o', 'm', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4952);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ziert.
pub(crate) fn n2710(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3282);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4285);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4286);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4954);
							s.is_empty()
						}
						['p', 'm', s @ ..] => {
							super::push(out, 4955);
							match s {
								[] => true,
								['o', 'k', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5272);
									s.is_empty()
								}
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
			super::push(out, 3283);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4956);
							match s {
								[] => true,
								['i', 'f', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 5273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'e', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3284);
			match s {
				[] => true,
				['a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -entlich.
pub(crate) fn n4720(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'r', 'o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5095);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 5096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'f', 'ö', '\u{1}', s @ ..] => {
			super::push(out, 4576);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5097);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5099);
					s.is_empty()
				}
				['e', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4577);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4578);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5101);
					s.is_empty()
				}
				['s', 'i', 'l', 'f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5103);
					s.is_empty()
				}
				['e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5104);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -örtlich.
pub(crate) fn n4725(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4580);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 4581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stlich.
pub(crate) fn n3394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2677);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3778);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				['w', 'd', s @ ..] => {
					super::push(out, 4583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'h', 'c', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			super::push(out, 3779);
			s.is_empty()
		}
		['r', 'ü', 'f', 'r', s @ ..] => {
			super::push(out, 3780);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 3781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttlich.
pub(crate) fn n3395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2678);
	match s {
		[] => true,
		['i', 'b', s @ ..] => {
			super::push(out, 3782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utlich.
pub(crate) fn n3396(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3783);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ütlich.
pub(crate) fn n3398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2680);
	match s {
		[] => true,
		['m', 'e', 'g', 'z', s @ ..] => {
			super::push(out, 3785);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3786);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word us-amerikanisch.
pub(crate) fn n6710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5444);
	s.is_empty()
}

/// Words ending -damerikanisch.
pub(crate) fn n6711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5445);
	s.is_empty()
}

/// The word lateinamerikanisch.
pub(crate) fn n6712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5446);
	s.is_empty()
}

/// The word afroamerikanisch.
pub(crate) fn n6713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5447);
	s.is_empty()
}

/// Words ending -ramerikanisch.
pub(crate) fn n6714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5448);
	s.is_empty()
}

/// Words ending -mannisch.
pub(crate) fn n5794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4656);
	s.is_empty()
}

/// The word tyrannisch.
pub(crate) fn n5795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4657);
	s.is_empty()
}

/// The word britannisch.
pub(crate) fn n5796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4658);
	s.is_empty()
}

/// Words ending -männisch.
pub(crate) fn n4871(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4659);
			s.is_empty()
		}
		['d', 'i', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4660);
			s.is_empty()
		}
		['f', 'u', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4661);
			s.is_empty()
		}
		['h', 'c', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4662);
			s.is_empty()
		}
		['s', 't', 'a', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 4663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onisch.
pub(crate) fn n3485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2746);
	match s {
		[] => true,
		['d', 'r', s @ ..] => {
			super::push(out, 3894);
			s.is_empty()
		}
		['f', 'n', s @ ..] => {
			super::push(out, 3895);
			s.is_empty()
		}
		['h', 'p', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4664);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3896);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4666);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', s @ ..] => {
			super::push(out, 3898);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4668);
					s.is_empty()
				}
				['o', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5143);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5145);
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3900);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4670);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['o', 's', s @ ..] => {
					super::push(out, 4672);
					s.is_empty()
				}
				['u', 'e', s @ ..] => {
					super::push(out, 4673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnisch.
pub(crate) fn n3486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2747);
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3901);
			s.is_empty()
		}
		['o', 'f', 'i', 'l', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aunisch.
pub(crate) fn n3487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2748);
	s.is_empty()
}

/// Words ending -ynisch.
pub(crate) fn n3488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2749);
	s.is_empty()
}

/// The word dänisch.
pub(crate) fn n3489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2750);
	s.is_empty()
}

/// Words ending -zerisch.
pub(crate) fn n4915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3925);
	s.is_empty()
}

/// The word nutznießerisch.
pub(crate) fn n4916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3926);
	s.is_empty()
}

/// Words ending -frisch.
pub(crate) fn n3500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2761);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3927);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -grisch.
pub(crate) fn n3501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2762);
	s.is_empty()
}

/// Words ending -hrisch.
pub(crate) fn n3502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2763);
	s.is_empty()
}

/// Words ending -irisch.
pub(crate) fn n3503(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 3929);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3930);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3931);
			s.is_empty()
		}
		['t', 'a', s @ ..] => {
			super::push(out, 3932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mrisch.
pub(crate) fn n3504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2764);
	s.is_empty()
}

/// Words ending -orisch.
pub(crate) fn n3505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2765);
	match s {
		[] => true,
		['h', 'p', 'a', s @ ..] => {
			super::push(out, 3933);
			s.is_empty()
		}
		['s', 'u', s @ ..] => {
			super::push(out, 3934);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5157);
							s.is_empty()
						}
						['i', 'z', s @ ..] => {
							super::push(out, 5158);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5336);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', 'o', s @ ..] => {
							super::push(out, 5159);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5160);
							match s {
								[] => true,
								['i', 'b', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 5161);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'g', s @ ..] => {
									super::push(out, 5339);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 5340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4724);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4725);
					s.is_empty()
				}
				['k', 'i', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				['n', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 4727);
					s.is_empty()
				}
				['o', 'n', s @ ..] => {
					super::push(out, 4728);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				['s', 'i', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5163);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5164);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 5165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrisch.
pub(crate) fn n3506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2766);
	match s {
		[] => true,
		['e', 't', '\u{1}', s @ ..] => {
			super::push(out, 3935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trisch.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2767);
	match s {
		[] => true,
		['e', 'm', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4730);
					match s {
						[] => true,
						['y', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5341);
									s.is_empty()
								}
								['l', 'a', s @ ..] => {
									super::push(out, 5342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3936);
			match s {
				[] => true,
				['e', 'z', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5166);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5343);
									s.is_empty()
								}
								['n', 'y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 5344);
									s.is_empty()
								}
								['p', 'o', 'r', 'h', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5345);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5346);
									s.is_empty()
								}
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

/// Words ending -urisch.
pub(crate) fn n3508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2768);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3937);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3938);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 3939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrisch.
pub(crate) fn n3509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2769);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 3940);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3941);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ärisch.
pub(crate) fn n3510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2770);
	match s {
		[] => true,
		['t', 'i', 'l', 'i', 'm', 'a', s @ ..] => {
			super::push(out, 3942);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sozialdemokratisch.
pub(crate) fn n6715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5449);
	s.is_empty()
}

/// The word gegendemokratisch.
pub(crate) fn n6716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5450);
	s.is_empty()
}

/// The word postdemokratisch.
pub(crate) fn n6717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5451);
	s.is_empty()
}

/// Words ending -etisch.
pub(crate) fn n3523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2779);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4755);
					s.is_empty()
				}
				['r', 'e', 'n', 'y', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4758);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5189);
							match s {
								[] => true,
								['p', 'm', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 5359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3963);
			s.is_empty()
		}
		['j', 'w', 'o', 's', 'o', s @ ..] => {
			super::push(out, 3964);
			s.is_empty()
		}
		['l', 'h', s @ ..] => {
			super::push(out, 3965);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3966);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4760);
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 4762);
					s.is_empty()
				}
				['g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3968);
			match s {
				[] => true,
				['n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4764);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itisch.
pub(crate) fn n3525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2781);
	match s {
		[] => true,
		['i', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5191);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 5192);
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
				['a', s @ ..] => {
					super::push(out, 4766);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					super::push(out, 4767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['a', 'b', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 4768);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 4769);
					s.is_empty()
				}
				['d', 'n', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4770);
					s.is_empty()
				}
				['e', 'd', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4771);
					s.is_empty()
				}
				['h', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4772);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5193);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5194);
							s.is_empty()
						}
						['h', 'c', 'a', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5195);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 5196);
							s.is_empty()
						}
						['r', 'u', 't', 'l', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5197);
							s.is_empty()
						}
						['t', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'n', s @ ..] => {
			super::push(out, 3973);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ktisch.
pub(crate) fn n3526(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3975);
			match s {
				[] => true,
				['m', 'i', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4773);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4774);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 4776);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5199);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 5200);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									super::push(out, 5360);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'd', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4778);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 4779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltisch.
pub(crate) fn n3527(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3977);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntisch.
pub(crate) fn n3528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2782);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4780);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4781);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4782);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4783);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5201);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5362);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4784);
					s.is_empty()
				}
				['t', 't', s @ ..] => {
					super::push(out, 4785);
					s.is_empty()
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
						['i', 's', s @ ..] => {
							super::push(out, 5202);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3980);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4787);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4788);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otisch.
pub(crate) fn n3529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2783);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			super::push(out, 3982);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3983);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3984);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3985);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4789);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4790);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'r', s @ ..] => {
			super::push(out, 3986);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3987);
			match s {
				[] => true,
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3988);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3989);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 4793);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3990);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 3991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptisch.
pub(crate) fn n3530(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3992);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4795);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4796);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5205);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'k', 's', s @ ..] => {
			super::push(out, 3993);
			s.is_empty()
		}
		['y', 'r', s @ ..] => {
			super::push(out, 3994);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -artisch.
pub(crate) fn n3531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2784);
	s.is_empty()
}

/// Words ending -stisch.
pub(crate) fn n3532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2785);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4797);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5207);
							s.is_empty()
						}
						['s', 'u', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4798);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5209);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 5210);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5364);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4799);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4800);
					s.is_empty()
				}
				['r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4801);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4802);
					s.is_empty()
				}
				['y', 'd', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 4803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 3995);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3996);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'd', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4805);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 5366);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'g', 'a', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5211);
							s.is_empty()
						}
						['r', 'a', 'g', 't', 'n', 'a', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5212);
							s.is_empty()
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
								['\u{1}', s @ ..] => {
									super::push(out, 5368);
									s.is_empty()
								}
								['n', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5369);
									s.is_empty()
								}
								['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5371);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5372);
									s.is_empty()
								}
								['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'i', 'z', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4806);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5213);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5214);
							s.is_empty()
						}
						['r', 'e', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 5215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => super::t2::n6445(s, out),
						['p', 'o', 's', '\u{1}', s @ ..] => super::t2::n6446(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t2::n5976(s, out),
				['l', s @ ..] => super::t2::n5977(s, out),
				['m', s @ ..] => super::t2::n5978(s, out),
				['n', s @ ..] => super::t2::n5979(s, out),
				['o', 'g', 'e', '\u{1}', s @ ..] => super::t2::n5980(s, out),
				['p', 'a', 'k', 's', 'e', '\u{1}', s @ ..] => super::t2::n5981(s, out),
				['r', s @ ..] => super::t2::n5982(s, out),
				['s', 's', 'a', 'r', '\u{1}', s @ ..] => super::t2::n5983(s, out),
				['t', s @ ..] => super::t2::n5984(s, out),
				['u', s @ ..] => super::t2::n5985(s, out),
				['v', 'i', 't', s @ ..] => super::t2::n5986(s, out),
				['w', s @ ..] => super::t2::n5987(s, out),
				['x', s @ ..] => super::t2::n5988(s, out),
				['z', s @ ..] => super::t2::n5989(s, out),
				_ => false,
			}
		}
		['n', 'e', 'p', 's', 'e', 'g', '\u{1}', s @ ..] => super::t2::n5009(s, out),
		['o', s @ ..] => super::t2::n5010(s, out),
		['s', 'i', 'z', 'r', 'a', 'n', '\u{1}', s @ ..] => super::t2::n5011(s, out),
		['u', s @ ..] => super::t2::n5012(s, out),
		['y', 'm', '\u{1}', s @ ..] => super::t2::n5013(s, out),
		_ => false,
	}
}

/// Words ending -öttisch.
pub(crate) fn n3533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2786);
	s.is_empty()
}

/// Words ending -utisch.
pub(crate) fn n3534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2787);
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 4002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytisch.
pub(crate) fn n3535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2788);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4814);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ätisch.
pub(crate) fn n3536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2789);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4003);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4004);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -chistisch.
pub(crate) fn n6445(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'a', 'v', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 5374);
			s.is_empty()
		}
		['o', 's', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5375);
			s.is_empty()
		}
		['r', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5376);
			s.is_empty()
		}
		['s', 'a', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5423);
					s.is_empty()
				}
				['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sophistisch.
pub(crate) fn n6446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5216);
	s.is_empty()
}

/// Words ending -kistisch.
pub(crate) fn n5976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4807);
	s.is_empty()
}

/// Words ending -listisch.
pub(crate) fn n5977(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5378);
					match s {
						[] => true,
						['d', 'i', 'n', s @ ..] => {
							super::push(out, 5425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5426);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5427);
							s.is_empty()
						}
						['z', 'o', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5452);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5379);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5428);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5429);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5430);
							s.is_empty()
						}
						['r', 'e', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['t', 'n', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5432);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5454);
									s.is_empty()
								}
								['t', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5455);
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
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5433);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5456);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 5457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'd', 'i', 'v', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5381);
					s.is_empty()
				}
				['h', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5382);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 5383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5217);
			s.is_empty()
		}
		['o', 'p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5218);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mistisch.
pub(crate) fn n5978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4808);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
			super::push(out, 5220);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5384);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nistisch.
pub(crate) fn n5979(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5386);
					s.is_empty()
				}
				['u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5221);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5434);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5436);
							s.is_empty()
						}
						['u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'r', 'a', 'd', 'l', 'a', 'i', 'z', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 5389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5222);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'e', s @ ..] => {
									super::push(out, 5458);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											super::push(out, 5462);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 5463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['m', 'm', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'o', 'p', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word egoistisch.
pub(crate) fn n5980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4809);
	s.is_empty()
}

/// The word eskapistisch.
pub(crate) fn n5981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4810);
	s.is_empty()
}

/// Words ending -ristisch.
pub(crate) fn n5982(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'u', 'k', 'ä', 's', '\u{1}', s @ ..] => {
					super::push(out, 5392);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5460);
									s.is_empty()
								}
								['t', 'u', '\u{1}', s @ ..] => {
									super::push(out, 5461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 'l', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5223);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5393);
					s.is_empty()
				}
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5394);
					s.is_empty()
				}
				['r', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 5395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['e', 'y', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 5396);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 5397);
					s.is_empty()
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 5398);
					s.is_empty()
				}
				['t', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word rassistisch.
pub(crate) fn n5983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4811);
	s.is_empty()
}

/// Words ending -tistisch.
pub(crate) fn n5984(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', 'o', 'p', 'r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5400);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5401);
					s.is_empty()
				}
				['v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 5403);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 5224);
			s.is_empty()
		}
		['n', 'e', 'd', 'e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5225);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5405);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5226);
			s.is_empty()
		}
		['ä', 'f', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uistisch.
pub(crate) fn n5985(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5228);
			s.is_empty()
		}
		['r', 't', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5229);
			s.is_empty()
		}
		['s', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tivistisch.
pub(crate) fn n5986(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5231);
			s.is_empty()
		}
		['i', 's', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wistisch.
pub(crate) fn n5987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4812);
	s.is_empty()
}

/// Words ending -xistisch.
pub(crate) fn n5988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4813);
	s.is_empty()
}

/// Words ending -zistisch.
pub(crate) fn n5989(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5407);
					s.is_empty()
				}
				['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5408);
					s.is_empty()
				}
				['s', 's', 'a', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5409);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', 'e', 'l', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5442);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5233);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gespenstisch.
pub(crate) fn n5009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3997);
	s.is_empty()
}

/// Words ending -ostisch.
pub(crate) fn n5010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3998);
	s.is_empty()
}

/// The word narzisstisch.
pub(crate) fn n5011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3999);
	s.is_empty()
}

/// Words ending -ustisch.
pub(crate) fn n5012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4000);
	s.is_empty()
}

/// The word mystisch.
pub(crate) fn n5013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4001);
	s.is_empty()
}

